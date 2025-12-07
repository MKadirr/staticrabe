use crate::utils::cursor_wrapper::CursorWrapper;
use crate::parser::header::ElfHeader;
use crate::utils::parse_error::ParseError;
use crate::parser::program_header::ProgramHeader;
use crate::parser::section_header::SectionHeader;

use std::collections::{hash_map, HashMap};

#[derive(Debug)]
pub struct ElfFile {
    elf_header: ElfHeader,
    program_headers: Vec<ProgramHeader>,
    section_headers: Vec<SectionHeader>,
}

fn get_name(buffer: &Vec<u8>, idx: u32) -> String {
    let mut start = idx as usize;

    for i in idx as usize..buffer.len() {
        if buffer[i] == 0 {
            if i == start {
                return String::from("");
            }
            else {
                return String::from_utf8((&buffer[start..i]).to_vec()).unwrap();
            }
        }
    }

    panic!("No string here")
}

impl ElfFile {
    pub fn parse(reader: &mut CursorWrapper) -> Result<Self, ParseError> {

        let elf_header = ElfHeader::parse(reader)?;
        let program_headers : Vec<ProgramHeader> = elf_header.parse_program_headers(reader)?;
        let section_headers : Vec<SectionHeader> = elf_header.parse_section_headers(reader)?;

        // println!("{}", elf_header.e_shsstrndx);
        // println!("{:#x?}", section_headers[elf_header.e_shsstrndx as usize]);

        let names_buffer = section_headers[elf_header.e_shsstrndx as usize].get_section(reader)?;

        // println!("{:#x?}", names_buffer);

        for section in &section_headers {
            println!("{:#x?}", get_name(&names_buffer, section.sh_name));
            println!("{:#x?}", section);
        }

        Ok(ElfFile {
            elf_header,
            program_headers,
            section_headers,
        })
    }
}