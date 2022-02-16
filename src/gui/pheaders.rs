use native_windows_gui as nwg;

use crate::elf::{Elf, Description};

// Program header table methods
impl super::ElfExplorer {
    pub fn pheaders_init(&self) {
        self.pheaders_list.set_headers_enabled(true);
        self.pheaders_layout.add_child((0, 0), (100, 60), &self.pheaders_list);
        self.pheaders_init_colummns();

        self.phdr_frame.set_visible(false);
        self.phdr_layout.add_child((0, 60), (100, 40), &self.phdr_frame);
        self.phdr_init();
    }

    fn pheaders_init_colummns(&self) {
        let index_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(100),
            text: Some("Index".to_owned())
        };
        let type_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(180),
            text: Some("Type".to_owned())
        };
        self.pheaders_list.insert_column(index_col);
        self.pheaders_list.insert_column(type_col);
    }

    pub fn pheaders_reset(&self) {
        self.pheaders_list.clear();
        self.phdr_reset();
        self.phdr_frame.set_visible(false);
    }

    pub fn pheaders_populate(&self, elf: &Elf) {
        self.pheaders_list.clear();

        match elf.is_64_bit() {
            true => {
                let phdrs = elf.phdr_table.phdrs64.as_ref().unwrap();
                for (i, phdr) in phdrs.iter().enumerate() {
                    let index = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(format!("{}", i))
                    };
                    let p_type = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(phdr.p_type.to_str())
                    };
                    self.pheaders_list.insert_item(index);
                    self.pheaders_list.insert_item(p_type);
                }
            },
            false => {
                let phdrs = elf.phdr_table.phdrs32.as_ref().unwrap();
                for (i, phdr) in phdrs.iter().enumerate() {
                    let index = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(format!("{}", i))
                    };
                    let p_type = nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(phdr.p_type.to_str())
                    };
                    self.pheaders_list.insert_item(index);
                    self.pheaders_list.insert_item(p_type);
                }
            }
        }
    }

    pub fn pheaders_select_event(&self) {
        if let Some(item) = self.pheaders_list.selected_item() {
            let p_type: u32;
            
            let elf = &*self.elf.borrow();
            let elf = elf.as_ref().unwrap();
            match elf.is_64_bit() {
                true => {
                    let phdr = &elf.phdr_table.phdrs64.as_ref().unwrap()[item];
                    p_type = phdr.p_type.0;
                }
                false => {
                    let phdr = &elf.phdr_table.phdrs32.as_ref().unwrap()[item];
                    p_type = phdr.p_type.0;
                }
            }

            let desc = match p_type {
                 0 => "NULL headers should be ignored",
                 1 => "LOAD headers specify a segment of the program that should be loaded into memory",
                 2 => "DYNAMIC headers specify information for the dynamic linker",
                 3 => "INTERP headers specify the filesystem path of the dynamic linker",
                 4 => "NOTE headers contain system specific information used for various purposes",
                 5 => "SHLIB headers are reserved and currently save no purpose",
                 6 => "A PHDR header specifies the location of the program header table itself in the file and in memory, if it is to be loaded into memory",
                 7 => "TLS headers specify a thread-local storage section",
                 0x6474e550 => "GNU_EH_FRAME headers specify exception handler information for GCC",
                 0x6474e551 => "GNU_STACK headers specify whether the stack should have execute permissions",
                 0x6474e552 => "GNU_RELRO headers spceify which parts of memory should be marked as read-only after relocation",
                 0x6474e553 => "GNU_PROPERTY headers specify special handling requirements for the kernel and dynamic linker",
                 _ => "Unknown program header type"
            };
            self.field_desc.set(desc);
        }
    }
}

// Program header methods
impl super::ElfExplorer {
    pub fn phdr_init(&self) {
        self.phdr_list.set_headers_enabled(true);
        self.phdr_layout.add_child((0, 0), (100, 100), &self.phdr_list);
        self.phdr_init_colummns();
    }

    fn phdr_init_colummns(&self) {
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
        self.phdr_list.insert_column(field_col);
        self.phdr_list.insert_column(value_col);
        self.phdr_list.insert_column(data_col);
    }

    pub fn phdr_reset(&self) {
        self.phdr_list.clear();
    }

    pub fn phdr_select_event(&self) {
        if let Some(item) = self.pheaders_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                _ => ()
            };
        }
    }
}