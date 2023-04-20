use std::{fs, path::Path, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(path: &str, chunk_type: &str, chunk_data: &str) -> Result<(), String> {
    let mut png = open_as_png(path)?;

    let chunk_type: ChunkType = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, chunk_data.into());
    png.append_chunk(chunk);

    let p = Path::new(path);

    Ok(fs::write(p.with_file_name("encoded.png"), png.as_bytes())
        .map_err(|e| format!("write to file {}: {}", path, e))?)
}

pub fn decode(path: &str, chunk_type: &str) -> Result<String, String> {
    let png = open_as_png(path)?;
    let found_chunk = png
        .chunk_by_type(chunk_type)
        .ok_or("chunk not found".to_string())?;

    Ok(found_chunk
        .data_as_string()
        .map_err(|e| format!("invalid chunk data: {}", e))?)
}

fn open_as_png(path: &str) -> Result<Png, String> {
    let png_data = fs::read(path).map_err(|e| format!("open file {}: {}", path, e))?;
    let png = Png::try_from(png_data.as_slice())?;
    Ok(png)
}