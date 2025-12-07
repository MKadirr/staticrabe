use std::io::SeekFrom;
use crate::utils::cursor_wrapper::{AddressSpaceHandler, CursorWrapper};
use crate::utils::parse_error::ParseError;
use crate::parser::section_header::SectionHeader;

pub struct RawSection {

}

impl SectionHeader {
    pub fn get_section(&self, reader: &mut CursorWrapper) -> Result<Vec<u8>, ParseError> {
        reader.seek(SeekFrom::Start(self.sh_offset))?;

        let mut ret : Vec<u8> = vec![0; self.sh_size as usize];
        reader.read_exact(ret.as_mut_slice())?;

        Ok(ret)
    }
}