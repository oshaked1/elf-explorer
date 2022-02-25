use native_windows_gui as nwg;

use crate::elf::{Elf, Description, SectionHeader32, SectionHeader64, ElfNOff, ElfNAddr};
use crate::{utils, descriptive_field, address_field, offset_field, size_field, decimal_field, hex_field};

// Section header table methods
impl super::ElfExplorer {
    pub fn sheaders_init(&self) {
        self.sheaders_list.set_headers_enabled(true);
        self.sheaders_layout.add_child((0, 0), (100, 60), &self.sheaders_list);
        self.sheaders_init_colummns();

        self.shdr_frame.set_visible(false);
        self.sheaders_layout.add_child((0, 60), (100, 40), &self.shdr_frame);
        self.shdr_init();
    }

    fn sheaders_init_colummns(&self) {
        let index_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(100),
            text: Some("Index".to_owned())
        };
        let type_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(495),
            text: Some("Type".to_owned())
        };
        self.sheaders_list.insert_column(index_col);
        self.sheaders_list.insert_column(type_col);
    }

    pub fn sheaders_reset(&self) {
        self.sheaders_list.clear();
        self.shdr_reset();
        self.shdr_frame.set_visible(false);
    }

    pub fn sheaders_populate(&self, elf: &Elf) {
        self.sheaders_list.clear();

        match elf.is_64_bit() {
            true => {
                let shdrs = elf.shdr_table.shdrs64.as_ref().unwrap();
                for (i, shdr) in shdrs.iter().enumerate() {
                    let index = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(format!("{}", i))
                    };
                    let s_type = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(shdr.sh_type.to_str())
                    };
                    self.sheaders_list.insert_item(index);
                    self.sheaders_list.insert_item(s_type);
                }
            },
            false => {
                let shdrs = elf.shdr_table.shdrs32.as_ref().unwrap();
                for (i, shdr) in shdrs.iter().enumerate() {
                    let index = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(format!("{}", i))
                    };
                    let s_type = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(shdr.sh_type.to_str())
                    };
                    self.sheaders_list.insert_item(index);
                    self.sheaders_list.insert_item(s_type);
                }
            }
        }
    }

    pub fn sheaders_select_event(&self) {
        if let Some(item) = self.sheaders_list.selected_item() {
            self.shdr_reset();

            let sh_type: u32;

            let elf = &*self.elf.borrow();
            let elf = elf.as_ref().unwrap();
            match elf.is_64_bit() {
                true => {
                    let shdr = &elf.shdr_table.shdrs64.as_ref().unwrap()[item];
                    self.shdr_populate_64bit(elf, shdr);
                    sh_type = shdr.sh_type.0;
                }
                false => {
                    let shdr = &elf.shdr_table.shdrs32.as_ref().unwrap()[item];
                    self.shdr_populate_32bit(elf, shdr);
                    sh_type = shdr.sh_type.0;
                }
            }

            let desc = match sh_type {
                0 => "NULL represents an entry with no associated section",
                1 => "PROGBITS sections contain information defined by the program",
                2 => "SYMTAB sections hold a symbol table",
                3 => "STRTAB sections hold a string table",
                4 => "RELA sections hold relocation entries with explicit addends",
                5 => "HASH sections hold a symbol hash table, which is required for dynamically linked files",
                6 => "DYNAMIC sections contain information for dynamic linking",
                7 => "NOTE sections contain special information that marks the file in some way",
                8 => "NOBITS sections are similar to PROGBITS sections, but occupy no space in the file",
                9 => "REL sections hold relocation entries without explicit addends",
                10 => "SHLIB identify reserved sections",
                11 => "DYNSYM sections hold a minimal symbol table",
                14 => "INIT_ARRAY sections hold a set of pointers to initialization functions",
                15 => "FINI_ARRAY sections hold a set of pointers to termination functions",
                0x6ffffff6 => "GNU_HASH sections hold a GNU style symbol hash table",
                0x6ffffffd => "VERDEF sections contain versioning information",
                0x6ffffffe => "VERNEED sections contain dependency information",
                0x6fffffff => "VERSYM sections describe the relation between symbols and version information",
                _ => "Unknown section header type"
            };
            self.field_desc.set(desc);

            self.shdr_frame.set_visible(true);
        }
    }
}

// Section header methods
impl super::ElfExplorer {
    pub fn shdr_init(&self) {
        self.shdr_list.set_headers_enabled(true);
        self.shdr_layout.add_child((0, 0), (100, 100), &self.shdr_list);
        self.shdr_init_colummns();
    }

    fn shdr_init_colummns(&self) {
        let field_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(100),
            text: Some("Field".to_owned())
        };
        let value_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(240),
            text: Some("Value".to_owned())
        };
        let data_col = nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(255),
            text: Some("Data".to_owned())
        };
        self.shdr_list.insert_column(field_col);
        self.shdr_list.insert_column(value_col);
        self.shdr_list.insert_column(data_col);
    }

    pub fn shdr_reset(&self) {
        self.shdr_list.clear();
    }

    pub fn shdr_populate_32bit(&self, elf: &Elf, shdr: &SectionHeader32) {
        let list = &self.shdr_list;
        let is_little_endian = elf.is_little_endian();

        // insert sh_name field
        decimal_field!("sh_name", shdr.sh_name, list, 0);

        // insert sh_type field
        descriptive_field!("sh_type", shdr.sh_type, list, 1);

        // insert sh_flags field
        descriptive_field!("sh_flags", shdr.sh_flags, list, 2);

        // insert sh_addr field
        address_field!("sh_addr", shdr.sh_addr, list, 3, is_little_endian);

        // insert sh_offset field
        offset_field!("sh_offset", shdr.sh_offset, list, 4, is_little_endian);

        // insert sh_size field
        size_field!("sh_size", shdr.sh_size, list, 5);

        // insert sh_link field
        decimal_field!("sh_link", shdr.sh_link, list, 6);

        // insert sh_info field
        hex_field!("sh_info", shdr.sh_info, list, 7);

        // insert sh_addralign field
        decimal_field!("sh_addralign", shdr.sh_addralign, list, 8);

        // insert sh_entsize field
        size_field!("sh_entsize", shdr.sh_entsize, list, 9);
    }

    pub fn shdr_populate_64bit(&self, elf: &Elf, shdr: &SectionHeader64) {
        let list = &self.shdr_list;
        let is_little_endian = elf.is_little_endian();

        // insert sh_name field
        decimal_field!("sh_name", shdr.sh_name, list, 0);

        // insert sh_type field
        descriptive_field!("sh_type", shdr.sh_type, list, 1);

        // insert sh_flags field
        descriptive_field!("sh_flags", shdr.sh_flags, list, 2);

        // insert sh_addr field
        address_field!("sh_addr", shdr.sh_addr, list, 3, is_little_endian);

        // insert sh_offset field
        offset_field!("sh_offset", shdr.sh_offset, list, 4, is_little_endian);

        // insert sh_size field
        size_field!("sh_size", shdr.sh_size, list, 5);

        // insert sh_link field
        decimal_field!("sh_link", shdr.sh_link, list, 6);

        // insert sh_info field
        hex_field!("sh_info", shdr.sh_info, list, 7);

        // insert sh_addralign field
        decimal_field!("sh_addralign", shdr.sh_addralign, list, 8);

        // insert sh_entsize field
        size_field!("sh_entsize", shdr.sh_entsize, list, 9);
    }

    pub fn shdr_select_event(&self) {
        if let Some(item) = self.shdr_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            // set description based on field order of 32 bit pheader
            match item {
                0 => set("Section name"),
                1 => set("Section type"),
                2 => set("Section flags (W=Write, A=Alloc, X=Execute, I=Info Link)"),
                3 => set("Section address in memory"),
                4 => set("Section offset in file"),
                5 => set("Section size"),
                6 => set("Section header table link (interpretation depends on section type)"),
                7 => set("Extra info (interpretation depends on section type)"),
                8 => set("Address alignment of section"),
                9 => set("Entry size (for sections that hold a table of some sort)"),
                _ => set("")
            };
        }
    }
}