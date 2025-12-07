use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::utils::parse_error::ParseError;

pub trait AddressSpaceHandler {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ParseError>;

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ParseError> {
        let size = self.read(buf)?;

        if size < buf.len() {
            Err(ParseError::NotEnoughBytes)
        }
        else {
            Ok(())
        }
    }

    fn read_auto<const N : usize>(&mut self) -> Result<[u8; N], ParseError> {
        let mut buf = [0u8; N];

        self.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn read_one(&mut self) -> Result<u8, ParseError> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, ParseError>;

    fn rewind(&mut self) -> Result<(), ParseError> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }
}

pub struct CursorWrapper {
    _cursor : Cursor<Vec<u8>>,
}

impl CursorWrapper {
    pub(crate) fn new(buf: Vec<u8>) -> Self {
        Self {
            _cursor: Cursor::new(buf)
        }
    }
}

impl AddressSpaceHandler for CursorWrapper {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ParseError> {
        Ok(self._cursor.read(buf).map_err(|e| { ParseError::FailedToRead })?)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, ParseError> {
        Ok(self._cursor.seek(pos).map_err(|e| { ParseError::FailedToSeek })?)
    }
}
