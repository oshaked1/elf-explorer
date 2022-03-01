use native_windows_gui as nwg;

use crate::gui::ElfExplorer;
use crate::elf::sections::StrtabSection;

impl ElfExplorer {
    pub fn strtab_init(&self) {
        self.strtab_list.set_headers_enabled(true);
        self.strtab_layout.add_child((0, 0), (100, 100), &self.strtab_list);
        self.strtab_init_columns();
    }

    fn strtab_init_columns(&self) {
        let offset_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(80),
            text: Some("Offset".to_owned())
        };
        let string_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(500),
            text: Some("String".to_owned())
        };
        self.strtab_list.insert_column(offset_col);
        self.strtab_list.insert_column(string_col);
    }

    pub fn strtab_populate(&self, strtab: &StrtabSection) {
        self.strtab_list.clear();

        for (i, (offset, string)) in strtab.strings.iter().enumerate() {
            let offset = nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 0,
                text: Some(format!("0x{:x}", offset))
            };
            let string = nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 1,
                text: Some(string.to_owned())
            };
            self.strtab_list.insert_item(offset);
            self.strtab_list.insert_item(string);
        }
    }
}