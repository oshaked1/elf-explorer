use native_windows_gui as nwg;

use crate::elf::{Description, EIdent, Elf, ElfNAddr, ElfNOff};
use crate::{
    address_field, decimal_field, descriptive_field, hex_field, offset_field, raw_field,
    size_field, utils,
};

// ELF header methods
impl super::ElfExplorer {
    pub fn elf_header_init(&self) {
        self.elf_header_list.set_headers_enabled(true);
        self.elf_header_layout
            .add_child((0, 0), (100, 60), &self.elf_header_list);
        self.elf_header_init_colummns();

        self.e_ident_frame.set_visible(false);
        self.elf_header_layout
            .add_child((0, 60), (100, 40), &self.e_ident_frame);
        self.e_ident_init();
    }

    fn elf_header_init_colummns(&self) {
        let field_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(100),
            text: Some("Field".to_owned()),
        };
        let value_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(240),
            text: Some("Value".to_owned()),
        };
        let data_col = nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(255),
            text: Some("Data".to_owned()),
        };
        self.elf_header_list.insert_column(field_col);
        self.elf_header_list.insert_column(value_col);
        self.elf_header_list.insert_column(data_col);
    }

    pub fn elf_header_reset(&self) {
        self.elf_header_list.clear();
        self.e_ident_reset();
        self.e_ident_frame.set_visible(false);
    }

    pub fn elf_header_populate(&self, elf: &Elf) {
        self.elf_header_list.clear();

        let list = &self.elf_header_list;

        // get byte order
        let is_little_endian = elf.is_little_endian();

        // insert e_ident field
        raw_field!("e_ident", elf.hdr.e_ident.raw, list, 0);

        // populate e_ident view
        self.e_ident_populate(&elf.hdr.e_ident);

        // insert e_type field
        descriptive_field!("e_type", elf.hdr.e_type, list, 1);

        // insert e_machine field
        descriptive_field!("e_machine", elf.hdr.e_machine, list, 2);

        // insert e_version field
        hex_field!("e_version", elf.hdr.e_version, list, 3);

        // insert e_entry field
        address_field!("e_entry", elf.hdr.e_entry, list, 4, is_little_endian);

        // insert e_phoff field
        offset_field!("e_phoff", elf.hdr.e_phoff, list, 5, is_little_endian);

        // insert e_shoff field
        offset_field!("e_shoff", elf.hdr.e_shoff, list, 6, is_little_endian);

        // insert e_flags field
        hex_field!("e_flags", elf.hdr.e_flags, list, 7);

        // insert e_ehsize field
        size_field!("e_ehsize", elf.hdr.e_ehsize, list, 8);

        // insert e_phentsize field
        size_field!("e_phentsize", elf.hdr.e_phentsize, list, 9);

        // insert e_phnum field
        decimal_field!("e_phnum", elf.hdr.e_phnum, list, 10);

        // insert e_shentsize field
        size_field!("e_shentsize", elf.hdr.e_shentsize, list, 11);

        // insert e_shnum field
        decimal_field!("e_shnum", elf.hdr.e_shnum, list, 12);

        // insert e_shstrndx field
        decimal_field!("e_shstrndx", elf.hdr.e_shstrndx, list, 13);
    }

    pub fn elf_header_select_event(&self) {
        if let Some(item) = self.elf_header_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                0 => {
                    self.e_ident_frame.set_visible(true);
                    set("ELF identifier (inner fields displayed below)")
                }
                1 => set("ELF type"),
                2 => set("Target CPU architecture"),
                3 => set("ELF version"),
                4 => set("Entry point (memory address)"),
                5 => set("Program header table file offset"),
                6 => set("Section header table file offset"),
                7 => set("Processor specific flags"),
                8 => set("Size of the ELF header"),
                9 => set("Size of each program header"),
                10 => set("Number of program headers"),
                11 => set("Size of each section header"),
                12 => set("Number of section headers"),
                13 => {
                    set("Section header table index of the entry that contains the section names")
                }
                _ => set(""),
            }
        }
    }
}

// e_ident methods
impl super::ElfExplorer {
    fn e_ident_init(&self) {
        self.e_ident_list.set_headers_enabled(true);
        self.e_ident_layout
            .add_child((0, 0), (100, 100), &self.e_ident_list);
        self.e_ident_init_colummns();
    }

    fn e_ident_init_colummns(&self) {
        let field_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(100),
            text: Some("Field".to_owned()),
        };
        let value_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(240),
            text: Some("Value".to_owned()),
        };
        let data_col = nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(255),
            text: Some("Data".to_owned()),
        };
        self.e_ident_list.insert_column(field_col);
        self.e_ident_list.insert_column(value_col);
        self.e_ident_list.insert_column(data_col);
    }

    fn e_ident_reset(&self) {
        self.e_ident_list.clear();
    }

    fn e_ident_populate(&self, e_ident: &EIdent) {
        self.e_ident_list.clear();

        let list = &self.e_ident_list;

        // insert EI_MAG field
        let ei_mag_field = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: Some("EI_MAG".to_owned()),
        };
        let ei_mag_value = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: Some(format!(
                "0x{:x}  {}{}{}",
                e_ident.ei_mag0,
                e_ident.ei_mag1 as char,
                e_ident.ei_mag2 as char,
                e_ident.ei_mag3 as char
            )),
        };
        let ei_mag_data = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 2,
            text: Some(format!(
                "{:X} {:X} {:X} {:X}",
                e_ident.ei_mag0, e_ident.ei_mag1, e_ident.ei_mag2, e_ident.ei_mag3
            )),
        };
        list.insert_item(ei_mag_field);
        list.insert_item(ei_mag_value);
        list.insert_item(ei_mag_data);

        // insert EI_CLASS field
        descriptive_field!("EI_CLASS", e_ident.ei_class, list, 1);

        // insert EI_DATA field
        descriptive_field!("EI_DATA", e_ident.ei_data, list, 2);

        // insert EI_VERSION field
        descriptive_field!("EI_VERSION", e_ident.ei_version, list, 3);

        // insert EI_OSABI field
        descriptive_field!("EI_OSABI", e_ident.ei_osabi, list, 4);

        // insert EI_ABIVERSION field
        decimal_field!("EI_ABIVERSION", e_ident.ei_abi_version, list, 5);

        // insert EI_PAD field
        raw_field!("EI_PAD", e_ident.ei_pad, list, 6);
    }

    pub fn e_ident_select_event(&self) {
        if let Some(item) = self.e_ident_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                0 => set("ELF magic (should be 0x7f ELF)"),
                1 => set("ELF class (32/64 bit)"),
                2 => set("Data format (little/big endian)"),
                3 => set("ELF identifier version"),
                4 => set("Operating system ABI"),
                5 => set("ABI version"),
                6 => set("Pad (should be all zeroes)"),
                _ => set(""),
            }
        }
    }
}
