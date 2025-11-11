use std::io::SeekFrom;
use crate::parser::cursor_wrapper::{AddressSpaceHandler, CursorWrapper};
use crate::parser::header::ElfHeader;
use crate::parser::parse_error::ParseError;
use crate::parser::program_header::ProgramHeader;

#[repr(u32)]
#[derive(Debug)]
pub enum ShType {
    SHT_NULL,
    SHT_PROGBITS,
    SHT_SYMTAB,
    SHT_STRTAB,
    SHT_RELA,
    SHT_HASH,
    SHT_DYNAMIC,
    SHT_NOTE,
    SHT_NOBITS,
    SHT_REL,
    SHT_SHLIB,
    SHT_DYNSYM,
    SHT_INIT_ARRAY,
    SHT_FINI_ARRAY,
    SHT_PREINIT_ARRAY,
    SHT_GROUP,
    SHT_SYMTAB_SHNDX,
    SHT_RELR,
    SHT_OS,

    Unknown(u32)
}

#[derive(Debug)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl ElfHeader {
    fn parse_section_header(&self, reader: &mut CursorWrapper) -> Result<SectionHeader, ParseError> {
        Ok(SectionHeader {
            sh_name: self.e_indet.read_four(reader)?,
            sh_type: match self.e_indet.read_four(reader)? {
                0x0 => ShType::SHT_NULL,
                0x1 => ShType::SHT_PROGBITS,
                0x2 => ShType::SHT_SYMTAB,
                0x3 => ShType::SHT_STRTAB,
                0x4 => ShType::SHT_RELA,
                0x5 => ShType::SHT_HASH,
                0x6 => ShType::SHT_DYNAMIC,
                0x7 => ShType::SHT_NOTE,
                0x8 => ShType::SHT_NOBITS,
                0x9 => ShType::SHT_REL,
                0x0A => ShType::SHT_SHLIB,
                0x0B => ShType::SHT_DYNSYM,
                0x0E => ShType::SHT_INIT_ARRAY,
                0x0F =>	ShType::SHT_FINI_ARRAY,
                0x10 => ShType::SHT_PREINIT_ARRAY,
                0x11 =>	ShType::SHT_GROUP,
                0x12 =>	ShType::SHT_SYMTAB_SHNDX,
                0x13 =>	ShType::SHT_RELR,
                x @ 0x60000000 ..=  0x6FFFFFFF => ShType::SHT_OS,

                n => ShType::Unknown(n)
            },
            sh_flags: self.e_indet.read_addr(reader)?,
            sh_addr: self.e_indet.read_addr(reader)?,
            sh_offset: self.e_indet.read_addr(reader)?,
            sh_size: self.e_indet.read_addr(reader)?,
            sh_link: self.e_indet.read_four(reader)?,
            sh_info: self.e_indet.read_four(reader)?,
            sh_addralign: self.e_indet.read_addr(reader)?,
            sh_entsize: self.e_indet.read_addr(reader)?,
        })
    }

    pub fn parse_section_headers(&self, reader: &mut CursorWrapper) -> Result<Vec<SectionHeader>, ParseError> {
        reader.seek(SeekFrom::Start(self.e_shoff))?;

        let mut res : Vec<SectionHeader> = Vec::new();

        for i in 0..self.e_shnum {
            res.push(self.parse_section_header(reader)?)
        }

        Ok(res)
    }
}