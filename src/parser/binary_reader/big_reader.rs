use crate::parser::binary_reader::BinaryReader;

pub struct BigEndianReader {}

impl BinaryReader for BigEndianReader {
    fn get8(buffer: [u8; 1]) -> u8 { buffer[0] }

    fn get16(buffer: [u8; 2]) -> u16 {
        u16::from_be_bytes(buffer)
    }

    fn get32(buffer: [u8; 4]) -> u32 {
        u32::from_be_bytes(buffer)
    }

    fn get64(buffer: [u8; 8]) -> u64 {
        u64::from_be_bytes(buffer)
    }
}