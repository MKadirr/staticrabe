#[derive(Debug)]
pub enum ParseError {
    NotEnoughBytes,
    FailedToRead,
    FailedToSeek,

    NotElfFile,
    UnsupportedEiClass,
    UnsupportedEndianness,
    UnsupportedOsABI,
    UnsupportedElfVersion,
    UnsupportedEType,
}