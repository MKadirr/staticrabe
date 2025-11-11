use crate::parser::cursor_wrapper::CursorWrapper;

pub struct MemorySection {
    pub real_addr: u64,
    pub size: u64,
    pub data: CursorWrapper,
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