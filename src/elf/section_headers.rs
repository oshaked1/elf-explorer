use super::{Description, ElfHeader, ElfNAddr, ElfNOff};
use crate::utils::{self, RcSlice};

pub struct SectionHeaderTable {
    pub shdrs32: Option<Vec<SectionHeader32>>,
    pub shdrs64: Option<Vec<SectionHeader64>>,
}

impl SectionHeaderTable {
    pub fn from(raw: RcSlice<u8>, hdr: &ElfHeader) -> Self {
        match hdr.is_64_bit() {
            true => Self {
                shdrs32: None,
                shdrs64: Some(Self::from_64_bit(raw, hdr)),
            },
            false => Self {
                shdrs32: Some(Self::from_32_bit(raw, hdr)),
                shdrs64: None,
            },
        }
    }

    fn from_32_bit(raw: RcSlice<u8>, hdr: &ElfHeader) -> Vec<SectionHeader32> {
        let is_little_endian = hdr.is_little_endian();
        let is_64_bit = false;
        let start_offset = hdr.e_shoff.to_usize();
        let end_offset = start_offset as usize + (hdr.e_shnum as usize * hdr.e_shentsize as usize);
        let shdrs_raw = RcSlice::from(&raw, start_offset, end_offset);
        let mut shdrs = Vec::new();
        for i in 0..hdr.e_shnum {
            let temp = RcSlice::from(
                &shdrs_raw,
                (i * hdr.e_shentsize) as usize,
                ((i + 1) * hdr.e_shentsize) as usize,
            );
            let sh_name = temp.read_u32(0, is_little_endian);
            let sh_type = SHType(temp.read_u32(4, is_little_endian));
            let sh_flags = SHFlags32(temp.read_u32(8, is_little_endian));
            let sh_addr = temp.read_elfn_addr(12, is_little_endian, is_64_bit);
            let sh_offset = temp.read_elfn_off(16, is_little_endian, is_64_bit);
            let sh_size = temp.read_u32(20, is_little_endian);
            let sh_link = temp.read_u32(24, is_little_endian);
            let sh_info = temp.read_u32(28, is_little_endian);
            let sh_addralign = temp.read_u32(32, is_little_endian);
            let sh_entsize = temp.read_u32(36, is_little_endian);
            shdrs.push(SectionHeader32 {
                name: None,
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addralign,
                sh_entsize,
            });
        }
        shdrs
    }

    fn from_64_bit(raw: RcSlice<u8>, hdr: &ElfHeader) -> Vec<SectionHeader64> {
        let is_little_endian = hdr.is_little_endian();
        let is_64_bit = true;
        let start_offset = hdr.e_shoff.to_usize();
        let end_offset = start_offset as usize + (hdr.e_shnum as usize * hdr.e_shentsize as usize);
        let shdrs_raw = RcSlice::from(&raw, start_offset, end_offset);
        let mut shdrs = Vec::new();
        for i in 0..hdr.e_shnum {
            let temp = RcSlice::from(
                &shdrs_raw,
                (i * hdr.e_shentsize) as usize,
                ((i + 1) * hdr.e_shentsize) as usize,
            );
            let sh_name = temp.read_u32(0, is_little_endian);
            let sh_type = SHType(temp.read_u32(4, is_little_endian));
            let sh_flags = SHFlags64(temp.read_u64(8, is_little_endian));
            let sh_addr = temp.read_elfn_addr(16, is_little_endian, is_64_bit);
            let sh_offset = temp.read_elfn_off(24, is_little_endian, is_64_bit);
            let sh_size = temp.read_u64(32, is_little_endian);
            let sh_link = temp.read_u32(40, is_little_endian);
            let sh_info = temp.read_u32(44, is_little_endian);
            let sh_addralign = temp.read_u64(48, is_little_endian);
            let sh_entsize = temp.read_u64(56, is_little_endian);
            shdrs.push(SectionHeader64 {
                name: None,
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addralign,
                sh_entsize,
            });
        }
        shdrs
    }

    pub fn populate_names(&mut self, filedata: RcSlice<u8>, hdr: &ElfHeader) {
        match hdr.is_64_bit() {
            true => {
                let strtab_hdr = &self.shdrs64.as_ref().unwrap()[hdr.e_shstrndx as usize];
                let offset = strtab_hdr.sh_offset.to_usize();
                let size = strtab_hdr.sh_size as usize;
                let strtab;
                if size != 0 {
                    strtab = &filedata.get()[offset..offset + size];
                }
                // some ELF files have a valid string table but the length specified in the header is 0
                else {
                    strtab = &filedata.get()[offset..filedata.end];
                }
                for shdr in self.shdrs64.as_mut().unwrap() {
                    let offset = shdr.sh_name as usize;
                    if let Ok(string) = utils::raw_to_str(&strtab[offset..]).1 {
                        shdr.name = Some(string.to_owned());
                    }
                }
            }
            false => {
                let strtab_hdr = &self.shdrs32.as_ref().unwrap()[hdr.e_shstrndx as usize];
                let offset = strtab_hdr.sh_offset.to_usize();
                let size = strtab_hdr.sh_size as usize;
                let strtab;
                if size != 0 {
                    strtab = &filedata.get()[offset..offset + size];
                } else {
                    strtab = &filedata.get()[offset..filedata.end];
                }
                for shdr in self.shdrs32.as_mut().unwrap() {
                    let offset = shdr.sh_name as usize;
                    if let Ok(string) = utils::raw_to_str(&strtab[offset..]).1 {
                        shdr.name = Some(string.to_owned());
                    }
                }
            }
        };
    }
}

pub struct SectionHeader32 {
    pub name: Option<String>,
    pub sh_name: u32,
    pub sh_type: SHType,
    pub sh_flags: SHFlags32,
    pub sh_addr: ElfNAddr,
    pub sh_offset: ElfNOff,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

pub struct SectionHeader64 {
    pub name: Option<String>,
    pub sh_name: u32,
    pub sh_type: SHType,
    pub sh_flags: SHFlags64,
    pub sh_addr: ElfNAddr,
    pub sh_offset: ElfNOff,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

pub struct SHType(pub u32);

impl Description for SHType {
    fn to_str(&self) -> String {
        match self.0 {
            0 => "NULL".to_owned(),
            1 => "PROGBITS".to_owned(),
            2 => "SYMTAB".to_owned(),
            3 => "STRTAB".to_owned(),
            4 => "RELA".to_owned(),
            5 => "HASH".to_owned(),
            6 => "DYNAMIC".to_owned(),
            7 => "NOTE".to_owned(),
            8 => "NOBITS".to_owned(),
            9 => "REL".to_owned(),
            10 => "SHLIB".to_owned(),
            11 => "DYNSYM".to_owned(),
            14 => "INIT_ARRAY".to_owned(),
            15 => "FINI_ARRAY".to_owned(),
            0x6ffffff6 => "GNU_HASH".to_owned(),
            0x6ffffffd => "VERDEF".to_owned(),
            0x6ffffffe => "VERNEED".to_owned(),
            0x6fffffff => "VERSYM".to_owned(),
            other => format!("<unknown: 0x{:x}>", other),
        }
    }
}

pub struct SHFlags32(pub u32);

impl Description for SHFlags32 {
    fn to_str(&self) -> String {
        let mut s = String::with_capacity(4);
        if self.0 & 1 != 0 {
            s.push('W');
        }
        if self.0 & 2 != 0 {
            s.push('A');
        }
        if self.0 & 4 != 0 {
            s.push('X');
        }
        if self.0 & 0x40 != 0 {
            s.push('I');
        }
        s
    }
}

pub struct SHFlags64(pub u64);

impl Description for SHFlags64 {
    fn to_str(&self) -> String {
        let mut s = String::with_capacity(4);
        if self.0 & 1 != 0 {
            s.push('W');
        }
        if self.0 & 2 != 0 {
            s.push('A');
        }
        if self.0 & 4 != 0 {
            s.push('X');
        }
        if self.0 & 0x40 != 0 {
            s.push('I');
        }
        s
    }
}
