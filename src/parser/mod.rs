use crate::parser::cursor_wrapper::CursorWrapper;
use crate::parser::elf_file::ElfFile;
use crate::parser::header::ElfHeader;
use crate::parser::parse_error::ParseError;


mod header;
mod parse_error;

mod binary_reader;
mod elf_file;
mod utils;
pub mod cursor_wrapper;
mod e_machine;
mod program_header;
mod section_header;
mod section;

pub fn parse_file(mut reader: &mut CursorWrapper) -> Result<ElfFile, ParseError> {
    let mut elf_file = ElfFile::parse(reader)?;

    // println!("yeah File: {:#x?}", elf_file);

    todo!();
}