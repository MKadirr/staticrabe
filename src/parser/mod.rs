use crate::utils::cursor_wrapper::CursorWrapper;
use crate::parser::elf_file::ElfFile;
use crate::utils::parse_error::ParseError;


mod header;

mod binary_reader;
mod elf_file;
mod utils;
mod e_machine;
mod program_header;
mod section_header;
mod section;

pub fn parse_file(mut reader: &mut CursorWrapper) -> Result<ElfFile, ParseError> {
    let mut elf_file = ElfFile::parse(reader)?;

    // println!("yeah File: {:#x?}", elf_file);

    todo!();
}