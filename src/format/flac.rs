use std::{collections::HashMap, path::Path};

use anyhow::{Result, anyhow};
use nom::{
    IResult,
    bytes::complete::{tag, take},
    number::complete::{be_u8, be_u16, be_u24, be_u32, be_u64, le_u32},
};

#[derive(Debug, Clone)]
pub enum FlacBlockType {
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    Cuesheet,
    Picture,
}

impl FlacBlockType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(FlacBlockType::StreamInfo),
            1 => Some(FlacBlockType::Padding),
            2 => Some(FlacBlockType::Application),
            3 => Some(FlacBlockType::SeekTable),
            4 => Some(FlacBlockType::VorbisComment),
            5 => Some(FlacBlockType::Cuesheet),
            6 => Some(FlacBlockType::Picture),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FlacPictureType {
    Other,
    PngIcon,
    GeneralIcon,
    FrontCover,
    BackCover,
    LinerNotes,
    MediaLabel,
    LeadArtist,
    Artist,
    Conductor,
    Orchestra,
    Composer,
    Lyricist,
    RecordingLocation,
    Recording,
    Performance,
    ScreenCapture,
    BrightlyColoredFish,
    Illustration,
    ArtistLogo,
    PublisherLogo,
}

impl FlacPictureType {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => FlacPictureType::Other,
            1 => FlacPictureType::PngIcon,
            2 => FlacPictureType::GeneralIcon,
            3 => FlacPictureType::FrontCover,
            4 => FlacPictureType::BackCover,
            5 => FlacPictureType::LinerNotes,
            6 => FlacPictureType::MediaLabel,
            7 => FlacPictureType::LeadArtist,
            8 => FlacPictureType::Artist,
            9 => FlacPictureType::Conductor,
            10 => FlacPictureType::Orchestra,
            11 => FlacPictureType::Composer,
            12 => FlacPictureType::Lyricist,
            13 => FlacPictureType::RecordingLocation,
            14 => FlacPictureType::Recording,
            15 => FlacPictureType::Performance,
            16 => FlacPictureType::ScreenCapture,
            17 => FlacPictureType::BrightlyColoredFish,
            18 => FlacPictureType::Illustration,
            19 => FlacPictureType::ArtistLogo,
            20 => FlacPictureType::PublisherLogo,
            _ => FlacPictureType::Other,
        }
    }
}

#[derive(Debug, Clone)]
struct FlacMetadataHeader {
    is_last: bool,
    block_type: Option<FlacBlockType>,
    block_size: u32,
}

#[derive(Debug, Clone)]
pub struct FlacStreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bps: u8,
    pub total_samples: u64,
    pub checksum: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct FlacPicture {
    pub picture_type: FlacPictureType,
    pub media_type: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub color_depth: u32,
    pub colors: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct FlacMetadata {
    pub stream_info: FlacStreamInfo,
    pub tags: HashMap<String, Vec<String>>,
    pub pictures: Vec<FlacPicture>,
}

fn parse_flac_marker(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("fLaC")(input)
}

fn parse_metadata_header(input: &[u8]) -> IResult<&[u8], FlacMetadataHeader> {
    let (input, first) = be_u8(input)?;
    let is_last = (first & 0x80) != 0;
    let block_type = FlacBlockType::from_u8(first & 0x7f);
    let (input, block_size) = be_u24(input)?;
    Ok((
        input,
        FlacMetadataHeader {
            is_last,
            block_type,
            block_size,
        },
    ))
}

fn parse_streaminfo(input: &[u8]) -> IResult<&[u8], FlacStreamInfo> {
    // first 10 bytes contain block and frame size information
    let (input, min_block_size) = be_u16(input)?;
    let (input, max_block_size) = be_u16(input)?;
    let (input, min_frame_size) = be_u24(input)?;
    let (input, max_frame_size) = be_u24(input)?;

    // next 8 bytes contain sample rate (20), channels (3), bps (5), total samples (36)
    let (input, data) = be_u64(input)?;
    let sample_rate = (data >> 44) as u32;
    let channels = ((data >> 41) & 0x7) as u8;
    let bps = ((data >> 36) & 0x1F) as u8;
    let total_samples = data & 0xFFFFFFFFF;

    // final 16 bytes contain an md5 checksum of the audio data
    let (input, bytes) = take(16usize)(input)?;
    let mut checksum = [0u8; 16];
    checksum.copy_from_slice(bytes);

    Ok((
        input,
        FlacStreamInfo {
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bps,
            total_samples,
            checksum,
        },
    ))
}

fn parse_vorbis_field(input: &[u8]) -> IResult<&[u8], (String, String)> {
    // first 4 bytes are the size of the field, then comes the utf-8 encoded field
    let (input, size) = le_u32(input)?;
    let (input, comment_bytes) = take(size)(input)?;
    let comment = String::from_utf8_lossy(comment_bytes).to_string();

    // split the field at the first instance of =
    if let Some((key, value)) = comment.split_once('=') {
        return Ok((input, (key.to_owned(), value.to_owned())));
    } else {
        return Ok((input, (comment, String::new())));
    }
}

fn parse_vorbis_comments(input: &[u8]) -> IResult<&[u8], HashMap<String, Vec<String>>> {
    // we have the size of the vendor string, the vendor, and the number of fields
    let (input, vendor_size) = le_u32(input)?;
    let (input, _vendor_string) = take(vendor_size)(input)?;
    let (input, fields) = le_u32(input)?;

    // iterate over the number of fields and parse them
    let mut rest = input;
    let mut comments: HashMap<String, Vec<String>> = HashMap::new();
    for _ in 0..fields {
        let (new_rest, (key, value)) = parse_vorbis_field(rest)?;
        comments.entry(key).or_insert_with(Vec::new).push(value);
        rest = new_rest;
    }
    Ok((rest, comments))
}

fn parse_picture(input: &[u8]) -> IResult<&[u8], FlacPicture> {
    // picture type, media type string, and the description of the picture
    let (input, picture_type_bytes) = be_u32(input)?;
    let picture_type = FlacPictureType::from_u32(picture_type_bytes);
    let (input, media_str_length) = be_u32(input)?;
    let (input, media_type_bytes) = take(media_str_length)(input)?;
    let media_type = String::from_utf8_lossy(media_type_bytes).to_string();
    let (input, description_length) = be_u32(input)?;
    let (input, description_bytes) = take(description_length)(input)?;
    let description = String::from_utf8_lossy(description_bytes).to_string();

    // information about the picture itself
    let (input, width) = be_u32(input)?;
    let (input, height) = be_u32(input)?;
    let (input, color_depth) = be_u32(input)?;
    let (input, colors) = be_u32(input)?;

    // length of data (32) and then the picture data
    let (input, size) = be_u32(input)?;
    let (input, data_bytes) = take(size)(input)?;
    let data = data_bytes.to_vec();

    Ok((
        input,
        FlacPicture {
            picture_type,
            media_type,
            description,
            width,
            height,
            color_depth,
            colors,
            data,
        },
    ))
}

fn parse_flac_metadata(input: &[u8]) -> IResult<&[u8], FlacMetadata> {
    // parse "fLaC" marker and setup parsing output
    let (mut input, _) = parse_flac_marker(input)?;
    let mut stream_info: Option<FlacStreamInfo> = None;
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    let mut pictures: Vec<FlacPicture> = Vec::new();

    // loop through metadata blocks until we hit the last block
    loop {
        let (rest, header) = parse_metadata_header(input)?;
        match header.block_type {
            Some(FlacBlockType::StreamInfo) => {
                let (rest, info) = parse_streaminfo(rest)?;
                stream_info = Some(info);
                input = rest;
            }
            Some(FlacBlockType::VorbisComment) => {
                let (rest, comments) = parse_vorbis_comments(rest)?;
                tags = comments;
                input = rest;
            }
            Some(FlacBlockType::Picture) => {
                let (rest, picture) = parse_picture(rest)?;
                pictures.push(picture);
                input = rest;
            }
            _ => {
                let (rest, _) = take(header.block_size)(rest)?;
                input = rest;
            }
        }
        if header.is_last {
            break;
        }
    }

    // error if there is no stream info
    let stream_info = stream_info.ok_or_else(|| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
    })?;

    Ok((
        input,
        FlacMetadata {
            stream_info,
            tags,
            pictures,
        },
    ))
}

pub fn parse_flac_file(path: &Path) -> Result<FlacMetadata> {
    let data = std::fs::read(path)?;
    let (_, metadata) = parse_flac_metadata(&data)
        .map_err(|e| anyhow!("[ERROR] Failed to parse FLAC file: {:?}", e))?;
    Ok(metadata)
}
