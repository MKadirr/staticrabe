use std::io::SeekFrom;
use crate::utils::cursor_wrapper::{AddressSpaceHandler, CursorWrapper};
use crate::parser::header::{Archi, ElfHeader};
use crate::utils::parse_error::ParseError;
use crate::parser::program_header::PType::{Unknown, PT_DYNAMIC, PT_INTERP, PT_LOAD, PT_NOTE, PT_NULL, PT_OS, PT_PHDR, PT_PROC, PT_SHLIB, PT_TLS};

#[derive(Debug)]
enum PType {
    PT_NULL,
    PT_LOAD,
    PT_DYNAMIC,
    PT_INTERP,
    PT_NOTE,
    PT_SHLIB,
    PT_PHDR,
    PT_TLS,
    PT_OS(u32),
    PT_PROC(u32),

    Unknown(u32),
}

#[derive(Debug)]
pub struct ProgramHeader {
    p_type: PType,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}

impl ElfHeader {

    fn parse_program_header(&self, reader: &mut CursorWrapper) -> Result<ProgramHeader, ParseError> {

        let p_type = match self.e_indet.read_four(reader)? {
            0x00000000 => PT_NULL,
            0x00000001 => PT_LOAD,
            0x00000002 => PT_DYNAMIC,
            0x00000003 => PT_INTERP,
            0x00000004 => PT_NOTE,
            0x00000005 => PT_SHLIB,
            0x00000006 => PT_PHDR,
            0x00000007 => PT_TLS,
            x @ 0x60000000 ..= 0x6FFFFFFF => PT_OS(x),
            x @ 0x70000000 ..= 0x7FFFFFFF => PT_PROC(x),
            x => Unknown(x),
        };

        let mut p_flags : u32 = 0;

        if let Archi::X64 = self.e_indet.ei_class {
            p_flags = self.e_indet.read_four(reader)?;
        }

        let p_offset = self.e_indet.read_addr(reader)?;
        let p_vaddr = self.e_indet.read_addr(reader)?;
        let p_paddr = self.e_indet.read_addr(reader)?;
        let p_filesz = self.e_indet.read_addr(reader)?;
        let p_memsz = self.e_indet.read_addr(reader)?;

        if let Archi::X32 = self.e_indet.ei_class {
            p_flags = self.e_indet.read_four(reader)?;
        }

        let p_align = self.e_indet.read_addr(reader)?;

        Ok(ProgramHeader {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        })
    }
    pub fn parse_program_headers(&self, reader: &mut CursorWrapper) -> Result<Vec<ProgramHeader>, ParseError> {
        reader.seek(SeekFrom::Start(self.e_phoff))?;

        let mut res : Vec<ProgramHeader> = Vec::new();

        for i in 0..self.e_phnum {
            res.push(self.parse_program_header(reader)?)
        }

        Ok(res)
    }
}