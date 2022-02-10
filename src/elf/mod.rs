use crate::utils::RcSlice;
use std::rc::Rc;
mod elf_header;
pub use elf_header::*;

const ELF_HDR_MAX_SIZE: usize = 64;

pub struct Elf {
    is_little_endian: bool,
    pub hdr: ElfHdr
}

impl Elf {
    pub fn from(raw: Vec<u8>) -> Result<Self, ParsingError>  {
        let len = raw.len();
        let raw = RcSlice::new(Rc::new(raw), 0, len);
        let hdr = ElfHdr::from(RcSlice::from(&raw, 0, ELF_HDR_MAX_SIZE))?;
        let is_little_endian = hdr.is_little_endian();

        Ok(Self { is_little_endian, hdr })
    }

    pub fn is_little_endian(&self) -> bool {
        self.is_little_endian
    }
}

pub enum ElfNAddr {
    Elf32Addr(u32),
    Elf64Addr(u64)
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidByteOrder(String),
    InvalidNativeSize(String)
}