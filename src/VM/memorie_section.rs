use crate::utils::cursor_wrapper::CursorWrapper;
use std::fmt;
use std::fmt::{write, Formatter};

pub struct MemorySection {
    pub name: String,
    pub real_addr: u64,
    pub size: u64,
    pub data: Vec<u8>,
}

impl MemorySection {
    pub fn get8(&self, addr: u64) -> Option<u8> {
        if addr < self.real_addr {
            return None;
        }

        let max_addr  = self.real_addr.checked_add(self.size)?;

        if max_addr <= addr + 1 {
            return None;
        }

        todo!()
    }

    pub fn get16(&self, addr: u64) -> Option<u16> {
        todo!()
    }

    pub fn get32(&self, addr: u64) -> Option<u32> {
        todo!()
    }

    pub fn get64(&self, addr: u64) -> Option<u64> {
        todo!()
    }
}

impl fmt::Display for MemorySection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}: {}\n", self.real_addr, self.real_addr + self.size, self.name);

        for i in 0..self.size {

        }

        write!(f, "\n")
    }
}