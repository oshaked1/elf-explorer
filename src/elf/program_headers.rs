use crate::utils::RcSlice;
use super::{ElfNAddr, ElfNOff, Description, ElfHeader};

pub struct ProgramHeaderTable {
    pub phdrs32: Option<Vec<ProgramHeader32>>,
    pub phdrs64: Option<Vec<ProgramHeader64>>
}

impl ProgramHeaderTable {
    pub fn from(raw: RcSlice<u8>, hdr: &ElfHeader) -> Self {
        match hdr.is_64_bit() {
            true => Self { phdrs32: None, phdrs64: Some(Self::from_64_bit(raw, hdr)) },
            false => Self { phdrs32: Some(Self::from_32_bit(raw, hdr)), phdrs64: None }
        }
    }

    fn from_32_bit(raw: RcSlice<u8>, hdr: &ElfHeader) -> Vec<ProgramHeader32> {
        let is_little_endian = hdr.is_little_endian();
        let is_64_bit = false;
        let start_offset = hdr.e_phoff.to_int() as usize;
        let end_offset = start_offset as usize + (hdr.e_phnum as usize * hdr.e_phentsize as usize);
        let phdrs_raw = RcSlice::from(&raw, start_offset, end_offset);
        let mut phdrs = Vec::new();
        for i in 0..hdr.e_phnum {
            let temp = RcSlice::from(&phdrs_raw, (i * hdr.e_phentsize) as usize, ((i+1) * hdr.e_phentsize) as usize);
            let p_type = PType(temp.read_u32(0, is_little_endian));
            let p_offset = temp.read_elfn_off(4, is_little_endian, is_64_bit);
            let p_vaddr = temp.read_elfn_addr(8, is_little_endian, is_64_bit);
            let p_paddr = temp.read_elfn_addr(12, is_little_endian, is_64_bit);
            let p_filesz = temp.read_u32(16, is_little_endian);
            let p_memsz = temp.read_u32(20, is_little_endian);
            let p_flags = PFlags(temp.read_u32(24, is_little_endian));
            let p_align = temp.read_u32(28, is_little_endian);
            phdrs.push(ProgramHeader32 {
                p_type,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_flags,
                p_align
            });
        }
        phdrs
    }

    fn from_64_bit(raw: RcSlice<u8>, hdr: &ElfHeader) -> Vec<ProgramHeader64> {
        let is_little_endian = hdr.is_little_endian();
        let is_64_bit = true;
        let start_offset = hdr.e_phoff.to_int() as usize;
        let end_offset = start_offset as usize + (hdr.e_phnum as usize * hdr.e_phentsize as usize);
        let phdrs_raw = RcSlice::from(&raw, start_offset, end_offset);
        let mut phdrs = Vec::new();
        for i in 0..hdr.e_phnum {
            let temp = RcSlice::from(&phdrs_raw, (i * hdr.e_phentsize) as usize, ((i+1) * hdr.e_phentsize) as usize);
            let p_type = PType(temp.read_u32(0, is_little_endian));
            let p_flags = PFlags(temp.read_u32(4, is_little_endian));
            let p_offset = temp.read_elfn_off(8, is_little_endian, is_64_bit);
            let p_vaddr = temp.read_elfn_addr(16, is_little_endian, is_64_bit);
            let p_paddr = temp.read_elfn_addr(24, is_little_endian, is_64_bit);
            let p_filesz = temp.read_u64(32, is_little_endian);
            let p_memsz = temp.read_u64(40, is_little_endian);
            let p_align = temp.read_u64(48, is_little_endian);
            phdrs.push(ProgramHeader64 {
                p_type,
                p_flags,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_align
            });
        }
        phdrs
    }
}

pub struct ProgramHeader32 {
    pub p_type: PType,
    pub p_offset: ElfNOff,
    pub p_vaddr: ElfNAddr,
    pub p_paddr: ElfNAddr,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: PFlags,
    pub p_align: u32
}

pub struct ProgramHeader64 {
    pub p_type: PType,
    pub p_flags: PFlags,
    pub p_offset: ElfNOff,
    pub p_vaddr: ElfNAddr,
    pub p_paddr: ElfNAddr,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64
}

pub struct PType(pub u32);

impl Description for PType {
    fn to_str(&self) -> String {
        match self.0 {
            0 => "NULL".to_owned(),
            1 => "LOAD".to_owned(),
            2 => "DYNAMIC".to_owned(),
            3 => "INTERP".to_owned(),
            4 => "NOTE".to_owned(),
            5 => "SHLIB".to_owned(),
            6 => "PHDR".to_owned(),
            7 => "TLS".to_owned(),
            0x6474e550 => "GNU_EH_FRAME".to_owned(),
            0x6474e551 => "GNU_STACK".to_owned(),
            0x6474e552 => "GNU_RELRO".to_owned(),
            0x6474e553 => "GNU_PROPERTY".to_owned(),
            other => format!("<unknown: 0x{:x}>", other)
        }
    }
}

pub struct PFlags(pub u32);

impl Description for PFlags {
    fn to_str(&self) -> String {
        let mut s = String::with_capacity(3);
        if self.0 & 1 != 0 {
            s.push('R');
        }
        if self.0 & 2 != 0 {
            s.push('W');
        }
        if self.0 & 4 != 0 {
            s.push('E');
        }
        s
    }
}