use crate::parser::binary_reader::BinaryReader;

pub struct LittleEndianReader {}

impl BinaryReader for LittleEndianReader {
    fn get8(buffer: [u8; 1]) -> u8 {
        u8::from_le_bytes(buffer)
    }

    fn get16(buffer: [u8; 2]) -> u16 {
        u16::from_le_bytes(buffer)
    }

    fn get32(buffer: [u8; 4]) -> u32 {
        u32::from_le_bytes(buffer)
    }

    fn get64(buffer: [u8; 8]) -> u64 {
        u64::from_le_bytes(buffer)
    }
}