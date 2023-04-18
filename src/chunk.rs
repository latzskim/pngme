use std::fmt::Display;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

fn calc_crc(chunk_type_bytes: [u8; 4], chunk_data_bytes: &[u8]) -> u32 {
    let bytes: Vec<u8> = chunk_type_bytes
        .iter()
        .chain((chunk_data_bytes).iter())
        .copied()
        .collect();

    CRC_32_ISO.checksum(&bytes)
}

pub const CRC_32_ISO: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);


#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc_iso: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let crc_iso = calc_crc(chunk_type.bytes(), chunk_data.as_ref());

        Chunk {
            chunk_type,
            chunk_data,
            crc_iso,
        }
    }

    pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc_iso
    }

    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.chunk_data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let len = self.length();
        len.to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc_iso.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

// TODO: Refactor
impl TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let len_be_bytes: &[u8] = value[0..4].try_into().or(Err("read length bytes"))?;
        let length = u32::from_be_bytes(
            len_be_bytes
                .try_into()
                .or(Err("length u32 from_be_bytes"))?,
        );

        let chunk_bytes: [u8; 4] = value[4..8].try_into().or(Err("read chunk bytes"))?;
        let chunk_type = ChunkType::try_from(chunk_bytes).or(Err("create ChunkType from bytes"))?;

        let chunk_data = Vec::from(&value[8..(length + 8) as usize]);
        let crc_iso = u32::from_be_bytes(
            value[(length + 8) as usize..]
                .try_into()
                .or(Err("crc from_be_bytes"))?,
        );

        if crc_iso != calc_crc(chunk_bytes, &chunk_data) {
            return Err(String::from("corrupted crc!"));
        }

        Ok(Chunk {
            chunk_type,
            chunk_data,
            crc_iso,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "chunk_type: {}, chunk_data: {}, crc: {}",
            self.chunk_type,
            self.data_as_string().unwrap(),
            self.crc_iso
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
