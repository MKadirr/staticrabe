use std::io::Read;

mod big_reader;
mod little_reader;

trait BinaryReader {
    fn get8(buffer: [u8; 1]) -> u8;
    fn get16(buffer: [u8; 2]) -> u16;
    fn get32(buffer: [u8; 4]) -> u32;
    fn get64(buffer: [u8; 8]) -> u64;
}
