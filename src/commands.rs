use std::{fs, path::Path, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(path: &str, chunk_type: &str, chunk_data: &str) -> Result<(), String> {
    let mut png = open_as_png(path)?;

    let chunk_type: ChunkType = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, chunk_data.into());
    png.append_chunk(chunk);

    let p = Path::new(path);

    fs::write(p.with_file_name("encoded.png"), png.as_bytes())
        .map_err(|e| format!("write to file {}: {}", path, e))
}

pub fn decode(path: &str, chunk_type: &str) -> Result<String, String> {
    let png = open_as_png(path)?;
    let found_chunk = png
        .chunk_by_type(chunk_type)
        .ok_or("chunk not found".to_string())?;

    found_chunk
        .data_as_string()
        .map_err(|e| format!("invalid chunk data: {}", e))
}

pub fn validate(chunk_type: &str) -> Result<(), String> {
    if let Err(e) = ChunkType::from_str(chunk_type) {
        return Err(format!("invalid chunk_type: {}", e));
    }
    Ok(())
}

pub fn get_chunks(path: &str) -> Result<Vec<Chunk>, String> {
    let png = open_as_png(path)?;
    let chunks = png.chunks().iter().map(|c| (**c).clone()).collect();
    Ok(chunks)
}

pub fn remove_chunk(path: &str, chunk_type: &str) -> Result<(), String> {
    let mut png = open_as_png(path)?;
    png.remove_chunk(chunk_type)?;

    let p = Path::new(path);
    fs::write(p.with_file_name(format!("removed_chunk_{}.png", chunk_type)), png.as_bytes())
        .map_err(|e| format!("write to file {}: {}", path, e))
}

fn open_as_png(path: &str) -> Result<Png, String> {
    let png_data = fs::read(path).map_err(|e| format!("open file {}: {}", path, e))?;
    let png = Png::try_from(png_data.as_slice())?;
    Ok(png)
}
