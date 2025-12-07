use crate::parser::e_machine::EMachine::Unknown;
use crate::utils::parse_error::ParseError;

#[derive(Debug, Clone)]
pub enum EMachine {
    NoSpecificInstructionSet,
    x86,

    Unknown(u16)
}

impl From<u16> for EMachine {
    fn from(value: u16) -> Self {
        match value {
            0x00 => EMachine::NoSpecificInstructionSet,
            0x03 => EMachine::x86,
            n => Unknown(n),
        }
    }
}