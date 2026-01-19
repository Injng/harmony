use std::{fs::File, io::Read, path::Path};

use anyhow::{Result, anyhow};
use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    sequence::delimited,
};
use zip::ZipArchive;

#[derive(Debug, Clone)]
pub struct EpubMetadata {
    pub title: Option<String>,
    pub identifier: Option<String>,
    pub language: Option<String>,
    pub creator: Option<String>,
    pub cover: Option<Vec<u8>>,
}

fn parse_tag<'a>(input: &'a str, tag_name: &str) -> Result<String> {
    let start_tag_prefix = format!("<{}", tag_name);
    let end_tag = format!("</{}>", tag_name);

    let (input, _) = take_until::<_, _, nom::error::Error<&str>>(start_tag_prefix.as_str())
        .parse(input)
        .map_err(|_| anyhow!("Could not find tag {}", tag_name))?;
    let (input, _) = tag::<_, _, nom::error::Error<&str>>(start_tag_prefix.as_str())
        .parse(input)
        .map_err(|_| anyhow!("Could not find tag {}", tag_name))?;

    let (_, content) = delimited(
        take_until(">").and(tag(">")),
        take_until(end_tag.as_str()),
        tag(end_tag.as_str()),
    )
    .parse(input)
    .map_err(|_: nom::Err<nom::error::Error<&str>>| anyhow!("Could not find tag {}", tag_name))?;

    Ok(content.to_string())
}

/// Parse the cover item ID from `<meta name="cover" content="cover_id"/>`
fn parse_cover_id(input: &str) -> Option<String> {
    input
        .split("<meta")
        .find(|s| s.contains("name=\"cover\""))
        .and_then(|s| s.split("content=\"").nth(1))
        .and_then(|s| s.split('"').next())
        .map(|s| s.to_string())
}

/// Parse the cover href from `<item id="cover_id" href="path/to/cover.jpg" .../>`
fn parse_cover_href(input: &str, cover_id: &str) -> Option<String> {
    let id_pattern = format!("id=\"{}\"", cover_id);
    input
        .split("<item")
        .find(|s| s.contains(&id_pattern))
        .and_then(|s| s.split("href=\"").nth(1))
        .and_then(|s| s.split('"').next())
        .map(|s| s.to_string())
}

fn parse_epub_metadata(input: &str) -> (EpubMetadata, Option<String>) {
    let cover_href = parse_cover_id(input).and_then(|id| parse_cover_href(input, &id));

    (
        EpubMetadata {
            title: parse_tag(input, "dc:title").ok(),
            identifier: parse_tag(input, "dc:identifier").ok(),
            language: parse_tag(input, "dc:language").ok(),
            creator: parse_tag(input, "dc:creator").ok(),
            cover: None, // populated later from archive
        },
        cover_href,
    )
}

pub fn parse_epub_file(path: &Path) -> Result<EpubMetadata> {
    // open the file as a reader
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    // the path to the metadata is contained in the container.xml file
    let opf_path = {
        let mut container = archive.by_name("META-INF/container.xml")?;
        let mut contents = String::new();
        container.read_to_string(&mut contents)?;
        contents
            .split("full-path=\"")
            .nth(1)
            .and_then(|s| s.split('\"').next())
            .ok_or_else(|| anyhow!("[ERROR] Could not find rootfile path in container.xml"))?
            .to_string()
    };

    // get the contents of the opf file and parse the metadata from it
    let (mut metadata, cover_href) = {
        let mut opf_file = archive.by_name(&opf_path)?;
        let mut opf_contents = String::new();
        opf_file.read_to_string(&mut opf_contents)?;
        parse_epub_metadata(&opf_contents)
    };

    // read cover image if present
    if let Some(href) = cover_href {
        let opf_dir = Path::new(&opf_path).parent().unwrap_or(Path::new(""));
        let cover_path = opf_dir.join(&href).to_string_lossy().to_string();
        if let Ok(mut cover_file) = archive.by_name(&cover_path) {
            let mut cover_data = Vec::new();
            if cover_file.read_to_end(&mut cover_data).is_ok() {
                metadata.cover = Some(cover_data);
            }
        }
    }

    Ok(metadata)
}
