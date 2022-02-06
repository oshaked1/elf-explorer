use crate::utils::RcSlice;
use std::rc::Rc;
mod elf_header;
pub use elf_header::*;

const ELF_HDR_MAX_SIZE: usize = 64;

pub struct Elf {
    pub raw: RcSlice<u8>,
    pub hdr: ElfHdr
}

impl Elf {
    pub fn from(raw: Vec<u8>) -> Self {
        let len = raw.len();
        let raw = RcSlice::new(Rc::new(raw), 0, len);
        let hdr = ElfHdr::from(RcSlice::from(&raw, 0, ELF_HDR_MAX_SIZE));
        Self { raw, hdr }
    }
}