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

fn parse_epub_metadata(input: &str) -> Result<EpubMetadata> {
    println!("{}", input);
    Ok(EpubMetadata {
        title: parse_tag(input, "dc:title").ok(),
        identifier: parse_tag(input, "dc:identifier").ok(),
        language: parse_tag(input, "dc:language").ok(),
        creator: parse_tag(input, "dc:creator").ok(),
    })
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
    let mut opf_file = archive.by_name(&opf_path)?;
    let mut opf_contents = String::new();
    opf_file.read_to_string(&mut opf_contents)?;
    let metadata = parse_epub_metadata(&opf_contents)
        .map_err(|e| anyhow!("[ERROR] Failed to parse OPF metadata: {:?}", e))?;
    Ok(metadata)
}
