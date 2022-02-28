mod strtab;
pub use strtab::*;

use crate::utils::RcSlice;
use super::{ElfHeader, SectionHeaderTable, SHType, Description};

pub enum SectionType {
    Generic,
    Strtab(StrtabSection)
}

pub struct Sections(pub Vec<Section>);

impl Sections {
    pub fn from(filedata: RcSlice<u8>, hdr: &ElfHeader, shdrs: &SectionHeaderTable) -> Self {
        let mut sections = Vec::with_capacity(hdr.e_shnum as usize);

        match hdr.is_64_bit() {
            true => {
                for (i, shdr) in shdrs.shdrs64.as_ref().unwrap().iter().enumerate() {
                    let index = i;
                    let name = match shdr.name.as_ref() {
                        None => None,
                        Some(name) => Some(name.to_owned())
                    };
                    let file_offset = shdr.sh_offset.to_usize();
                    let size = shdr.sh_size as usize;
                    let sh_type = &shdr.sh_type;
                    let data = match sh_type.0 {
                        // NOBITS
                        8 => None,
                        _ => match file_offset {
                            0 => None,
                            offset => match size {
                                0 => None,
                                size => Some(RcSlice::from(&filedata, offset, offset + size))
                            }
                        }
                    };
                    sections.push(Section::from(index, name, file_offset, size, &sh_type, data));
                }
            }
            false => {
                for (i, shdr) in shdrs.shdrs32.as_ref().unwrap().iter().enumerate() {
                    let index = i;
                    let name = match shdr.name.as_ref() {
                        None => None,
                        Some(name) => Some(name.to_owned())
                    };
                    let file_offset = shdr.sh_offset.to_usize();
                    let size = shdr.sh_size as usize;
                    let sh_type = &shdr.sh_type;
                    let data = match sh_type.0 {
                        // NOBITS
                        8 => None,
                        _ => match file_offset {
                            0 => None,
                            offset => match size {
                                0 => None,
                                size => Some(RcSlice::from(&filedata, offset, offset + size))
                            }
                        }
                    };
                    sections.push(Section::from(index, name, file_offset, size, &sh_type, data));
                }
            }
        }
        Sections(sections)
    }
}

pub struct Section {
    pub index: usize,
    pub name: Option<String>,
    pub file_offset: usize,
    pub size: usize,
    sh_type: SHType,
    pub section_type: SectionType,
    data: Option<RcSlice<u8>>
}

impl Section {
    fn from(index: usize, name: Option<String>, file_offset: usize, size: usize, sh_type: &super::SHType, data: Option<RcSlice<u8>>) -> Self {
        let data_copy = match data.as_ref() {
            None => None,
            Some(data) => Some(data.clone())
        };
        let section_type = match sh_type.0 {
            3 => SectionType::Strtab(StrtabSection::from(data_copy)),
            _ => SectionType::Generic
        };
        Self { index, name, file_offset, size, sh_type: SHType(sh_type.0), section_type, data }
    }

    pub fn type_name(&self) -> String {
        self.sh_type.to_str()
    }

    pub fn data(&self) -> Option<&[u8]> {
        match self.data.as_ref() {
            None => None,
            Some(data) => Some(data.get())
        }
    }
}