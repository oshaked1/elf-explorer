use std::rc::Rc;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::elf::ElfNAddr;

pub struct RcSlice<T> {
    rc: Rc<Vec<T>>,
    start: usize,
    end: usize,
}

impl<T> RcSlice<T> {
    pub fn new(rc: Rc<Vec<T>>, start: usize, end: usize) -> Self {
        assert!(end <= rc.len());
        Self { rc, start, end }
    }

    pub fn from(rcslice: &RcSlice<T>, start: usize, end: usize) -> Self {
        assert!(rcslice.start + end <= rcslice.end);
        Self {
            rc: rcslice.rc.clone(),
            start: rcslice.start + start,
            end: rcslice.start + end,
        }
    }

    pub fn get(&self) -> &[T] {
        &self.rc[self.start..self.end]
    }
}

impl RcSlice<u8> {
    pub fn read_u16(&self, offset: usize, is_little_endian: bool) -> u16 {
        match is_little_endian {
            true => (&self.get()[offset..offset+2]).read_u16::<LittleEndian>().unwrap(),
            false => (&self.get()[offset..offset+2]).read_u16::<BigEndian>().unwrap()
        }
    }

    pub fn read_u32(&self, offset: usize, is_little_endian: bool) -> u32 {
        match is_little_endian {
            true => (&self.get()[offset..offset+4]).read_u32::<LittleEndian>().unwrap(),
            false => (&self.get()[offset..offset+4]).read_u32::<BigEndian>().unwrap()
        }
    }

    pub fn read_elfn_addr(&self, offset: usize, is_little_endian: bool, is_64_bit: bool) -> crate::elf::ElfNAddr {
        match is_64_bit {
            true => {
                match is_little_endian {
                    true => ElfNAddr::Elf64Addr((&self.get()[offset..offset+8]).read_u64::<LittleEndian>().unwrap()),
                    false => ElfNAddr::Elf64Addr((&self.get()[offset..offset+8]).read_u64::<BigEndian>().unwrap())
                }
            },
            false => {
                match is_little_endian {
                    true => ElfNAddr::Elf32Addr((&self.get()[offset..offset+4]).read_u32::<LittleEndian>().unwrap()),
                    false => ElfNAddr::Elf32Addr((&self.get()[offset..offset+4]).read_u32::<BigEndian>().unwrap())
                }
            }
        }
    }
}
