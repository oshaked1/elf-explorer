use crate::utils::RcSlice;
use std::rc::Rc;
mod elf_header;
pub use elf_header::*;
mod program_headers;
pub use program_headers::*;
mod section_headers;
pub use section_headers::*;

const ELF_HDR_MAX_SIZE: usize = 64;

pub trait Description {
    fn to_str(&self) -> String;
}

pub struct Elf {
    is_little_endian: bool,
    is_64_bit: bool,
    pub hdr: ElfHeader,
    pub phdr_table: ProgramHeaderTable,
    pub shdr_table: SectionHeaderTable
}

impl Elf {
    pub fn from(raw: Vec<u8>) -> Result<Self, ParsingError>  {
        let len = raw.len();
        let raw = RcSlice::new(Rc::new(raw), 0, len);
        let hdr = ElfHeader::from(RcSlice::from(&raw, 0, ELF_HDR_MAX_SIZE))?;
        let is_little_endian = hdr.is_little_endian();
        let is_64_bit = hdr.is_64_bit();

        let phdr_table = ProgramHeaderTable::from(RcSlice::from(&raw, 0, len), &hdr);
        let mut shdr_table = SectionHeaderTable::from(RcSlice::from(&raw, 0, len), &hdr);

        shdr_table.populate_names(RcSlice::from(&raw, 0, len), &hdr);

        Ok(Self { is_little_endian, is_64_bit, hdr, phdr_table, shdr_table })
    }

    pub fn is_little_endian(&self) -> bool {
        self.is_little_endian
    }

    pub fn is_64_bit(&self) -> bool {
        self.is_64_bit
    }
}

pub enum ElfNAddr {
    Elf32Addr(u32),
    Elf64Addr(u64)
}

impl ElfNAddr {
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::Elf32Addr(addr) => addr.to_owned() as u64,
            Self::Elf64Addr(addr) => addr.to_owned()
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::Elf32Addr(addr) => addr.to_owned() as usize,
            Self::Elf64Addr(addr) => addr.to_owned() as usize
        }
    }
}

pub enum ElfNOff {
    Elf32Off(u32),
    Elf64Off(u64)
}

impl ElfNOff {
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::Elf32Off(off) => off.to_owned() as u64,
            Self::Elf64Off(off) => off.to_owned()
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::Elf32Off(off) => off.to_owned() as usize,
            Self::Elf64Off(off) => off.to_owned() as usize
        }
    }
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidByteOrder(String),
    InvalidNativeSize(String)
}