use crate::parser::cursor_wrapper::{AddressSpaceHandler, CursorWrapper};
use crate::parser::e_machine::EMachine;
use crate::parser::header::EType::{ET_CORE, ET_DYN, ET_EXEC, ET_NONE, ET_OS, ET_PROC, ET_REL};
use crate::parser::parse_error::ParseError;

#[derive(Debug, Clone)]
pub enum Archi {
    X32,
    X64,
}

impl Endian {
    pub fn parse16(&self, buf: &[u8; 2]) -> u16 {
        match self {
            Endian::Little => u16::from_le_bytes(*buf),
            Endian::Big => u16::from_be_bytes(*buf),
        }
    }

    pub fn parse32(&self, buf: &[u8; 4]) -> u32 {
        match self {
            Endian::Little => u32::from_le_bytes(*buf),
            Endian::Big => u32::from_be_bytes(*buf),
        }
    }

    pub fn parse64(&self, buf: &[u8; 8]) -> u64 {
        match self {
            Endian::Little => u64::from_le_bytes(*buf),
            Endian::Big => u64::from_be_bytes(*buf),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone)]
pub enum EType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_OS(u16),
    ET_LOOS,
    ET_HIOS,
    ET_PROC(u16),
    ET_LOPROC,
    ET_HIPROC,
}

#[derive(Debug, Clone)]
pub enum OsABI {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    True64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    NuxiCloudABI,
    StratusTechnologiesOpenVOS,
}

#[derive(Debug, Clone)]
pub struct EIndent {
    pub ei_mag: [u8; 4],
    pub ei_class: Archi,
    pub ei_data: Endian,
    pub ei_version: u8,
    pub ei_osabi: OsABI,
    pub ei_abiversion: u8,
    pub ei_pad: [u8; 7],
}

#[derive(Debug)]
pub struct ElfHeader {
    pub e_indet: EIndent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shsstrndx: u16,
}

impl ElfHeader {
    pub fn parse(reader: &mut CursorWrapper) -> Result<ElfHeader, ParseError> {
        let e_indet = EIndent::parse(reader)?;

        Ok(Self {
            e_indet: e_indet.clone(),
            e_type: match e_indet.read_two(reader)? {
                0x00 => ET_NONE,
                0x01 => ET_REL,
                0x02 => ET_EXEC,
                0x03 => ET_DYN,
                0x04 => ET_CORE,
                x @ 0xFE00 ..= 0xFEFF => ET_OS(x),
                x @ 0xFF00 ..= 0xFFFF => ET_PROC(x),

                _ => { return Err(ParseError::UnsupportedEType) }
            },
            e_machine: EMachine::from(e_indet.read_two(reader)?),
            e_version: e_indet.read_four(reader)?,
            e_entry: e_indet.read_addr(reader)?,
            e_phoff: e_indet.read_addr(reader)?,
            e_shoff: e_indet.read_addr(reader)?,
            e_flags: e_indet.read_four(reader)?,
            e_ehsize: e_indet.read_two(reader)?,
            e_phentsize: e_indet.read_two(reader)?,
            e_phnum: e_indet.read_two(reader)?,
            e_shentsize: e_indet.read_two(reader)?,
            e_shnum: e_indet.read_two(reader)?,
            e_shsstrndx: e_indet.read_two(reader)?,
        })
    }
}

impl EIndent {
    pub fn parse(reader: &mut CursorWrapper) -> Result<EIndent, ParseError> {
        let mut ei_mag = [0u8; 4];
        reader.read_exact(&mut ei_mag)?;

        if ei_mag[0] != 0x7F ||
            ei_mag[1] != 0x45 ||
            ei_mag[2] != 0x4c ||
            ei_mag[3] != 0x46 {
            return Err(ParseError::NotElfFile)
        }

        let mut res : EIndent = EIndent {
            ei_mag,
            ei_class: match reader.read_one()? {
                1 => Archi::X32,
                2 => Archi::X64,
                _ => { return Err(ParseError::UnsupportedEiClass); }
            },
            ei_data: match reader.read_one()? {
                1 => Endian::Little,
                2 => Endian::Big,
                _ => { return Err(ParseError::UnsupportedEndianness) }
            },
            ei_version: reader.read_one()?,
            ei_osabi: match reader.read_one()? {
                0x00 => OsABI::SystemV,
                0x01 => OsABI::HPUX,
                0x02 => OsABI::NetBSD,
                0x03 => OsABI::Linux,
                0x04 => OsABI::GNUHurd,
                0x06 => OsABI::Solaris,
                0x07 => OsABI::AIX,
                0x08 => OsABI::IRIX,
                0x09 => OsABI::FreeBSD,
                0x0A => OsABI::True64,
                0x0B => OsABI::NovellModesto,
                0x0C => OsABI::OpenBSD,
                0x0D => OsABI::OpenVMS,
                0x0E => OsABI::NonStopKernel,
                0x0F => OsABI::AROS,
                0x10 => OsABI::FenixOS,
                0x11 => OsABI::NuxiCloudABI,
                0x12 => OsABI::StratusTechnologiesOpenVOS,
                _ => { return Err(ParseError::UnsupportedOsABI) }
            },
            ei_abiversion: reader.read_one()?,
            ei_pad: reader.read_auto()?,
        };

        if res.ei_version != 1 {
            return Err(ParseError::UnsupportedElfVersion)
        }

        Ok(res)
    }

    pub fn read_two(&self, reader: &mut CursorWrapper) -> Result<u16, ParseError> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;

        Ok(self.ei_data.parse16(&buf))
    }

    pub fn read_four(&self, reader : &mut CursorWrapper) -> Result<u32, ParseError> {
        let buf : [u8; 4] = reader.read_auto()?;

        Ok(self.ei_data.parse32(&buf))
    }

    pub fn read_height(&self, reader : &mut CursorWrapper) -> Result<u64, ParseError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;

        Ok(self.ei_data.parse64(&buf))
    }

    pub fn read_addr(&self, reader: &mut CursorWrapper) -> Result<u64, ParseError> {
        match self.ei_class {
            Archi::X32 => {
                Ok(self.read_four(reader)? as u64)
            }
            Archi::X64 => {
                self.read_height(reader)
            }
        }
    }
}