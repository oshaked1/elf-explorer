use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::utils::RcSlice;
use super::ParsingError;

const EI_NIDENT: usize = 16;

pub struct ElfHdr {
    pub raw: RcSlice<u8>,
    pub e_ident: EIdent,
    pub e_type: EType,
}

impl ElfHdr {
    pub fn from(raw: RcSlice<u8>) -> Result<Self, ParsingError> {
        let e_ident = EIdent::from(RcSlice::from(&raw, 0, EI_NIDENT));

        let is_little_endian = match e_ident.ei_data.val {
            1 => true,
            2 => false,
            _ => { return Err(ParsingError::InvalidByteOrder("Could not determine byte order from EI_DATA field".to_owned())); }
        };
        
        let e_type: u16;
        if is_little_endian {
            e_type = (&raw.get()[16..=17]).read_u16::<LittleEndian>().unwrap();
        }
        else {
            e_type = (&raw.get()[16..=17]).read_u16::<BigEndian>().unwrap();
        }
        
        Ok(Self { raw, e_ident, e_type: EType { val: e_type } })
    }
}

pub struct EIdent {
    pub raw: RcSlice<u8>,
    pub ei_mag0: u8,
    pub ei_mag1: u8,
    pub ei_mag2: u8,
    pub ei_mag3: u8,
    pub ei_class: EiClass,
    pub ei_data: EiData,
    pub ei_version: EiVersion,
    pub ei_osabi: EiOsAbi,
    pub ei_abi_version: u8,
    pub ei_pad: RcSlice<u8>,
}

impl EIdent {
    pub fn from(raw: RcSlice<u8>) -> Self {
        let temp = raw.get();
        let ei_mag0 = temp[0];
        let ei_mag1 = temp[1];
        let ei_mag2 = temp[2];
        let ei_mag3 = temp[3];
        let ei_class = EiClass { val: temp[4] };
        let ei_data = EiData { val: temp[5] };
        let ei_version = EiVersion { val: temp[6] };
        let ei_osabi = EiOsAbi { val: temp[7] };
        let ei_abi_version = temp[8];
        let ei_pad = RcSlice::from(&raw, 9, EI_NIDENT);
        Self {
            raw,
            ei_mag0,
            ei_mag1,
            ei_mag2,
            ei_mag3,
            ei_class,
            ei_data,
            ei_version,
            ei_osabi,
            ei_abi_version,
            ei_pad,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct EiClass {
    pub val: u8
}

impl EiClass {
    pub fn to_str(&self) -> String {
        match self.val {
            0 => String::from("none"),
            1 => String::from("ELF32"),
            2 => String::from("ELF64"),
            other => format!("<unknown: 0x{:x}>", other)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EiData {
    pub val: u8
}

impl EiData {
    pub fn to_str(&self) -> String {
        match self.val {
            0 => String::from("none"),
            1 => String::from("2's complement, little endian"),
            2 => String::from("2's complement, big endian"),
            other => format!("<unknown: 0x{:x}>", other)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EiVersion {
    pub val: u8
}

impl EiVersion {
    pub fn to_str(&self) -> String {
        match self.val {
            0 => String::from("0"),
            1 => String::from("1 (current)"),
            other => format!("{}", other)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EiOsAbi {
    pub val: u8
}

// https://github.com/eliben/pyelftools/blob/master/elftools/elf/enums.py#L35
// https://github.com/eliben/pyelftools/blob/master/elftools/elf/descriptions.py#L308
impl EiOsAbi {
    pub fn to_str(&self) -> String {
        match self.val {
            0 => String::from("UNIX - System V"),
            1 => String::from("UNIX - HP-UX"),
            2 => String::from("UNIX - NetBSD"),
            3 => String::from("UNIX - Linux"),
            4 => String::from("UNIX - GNU/Hurd"),
            6 => String::from("UNIX - Solaris"),
            7 => String::from("UNIX - AIX"),
            8 => String::from("UNIX - IRIX"),
            9 => String::from("UNIX - FreeBSD"),
            10 => String::from("UNIX - TRU64"),
            11 => String::from("Novell - Modesto"),
            12 => String::from("UNIX - OpenBSD"),
            13 => String::from("VMS - OpenVMS"),
            14 => String::from("HP - Non-Stop Kernel"),
            15 => String::from("AROS"),
            16 => String::from("Fenix OS"),
            17 => String::from("Nuxi - CloudABI"),
            53 => String::from("Sortix"),
            64 => String::from("ARM - EABI"),
            97 => String::from("ARM - ABI"),
            102 => String::from("CellOS Lv-2"),
            255 => String::from("Standalone App"),
            other => format!("<unknown: 0x{:x}>", other)
        }
    }
}

pub struct EType {
    pub val: u16
}

impl EType {
    pub fn to_str(&self) -> String {
        match self.val {
            0 => "NONE (None)".to_owned(),
            1 => "REL (Relocatable file)".to_owned(),
            2 => "EXEC (Executable file)".to_owned(),
            3 => "DYN (Shared object file)".to_owned(),
            4 => "CORE (Core file)".to_owned(),
            other => format!("<unknown: 0x{:x}>", other)
        }
    }
}