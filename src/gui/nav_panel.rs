extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

impl super::ElfExplorer {
    pub fn nav_panel_init(&self) {
        self.nav_panel_layout.add_child((0, 0), (0, 100), &self.nav_panel_list);
        self.nav_panel_init_items();
    }

    fn nav_panel_init_items(&self) {
        let col = nwg::InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(195),
            text: None
        };
        self.nav_panel_list.insert_column(col);

        let elf_header = nwg::InsertListViewItem {
            index: None,
            column_index: 0,
            text: Some("ELF Header".to_owned())
        };
        self.nav_panel_list.insert_item(elf_header);
    }

    pub fn nav_panel_select(&self, item: usize) {
        self.nav_panel_list.select_item(item, true)
    }

    pub fn nav_panel_select_event(&self) {
        if let Some(item) = self.nav_panel_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                0 => set("ELF header, which contains general information as well as the locations of the program and section header tables"),
                _ => set("")
            }
        }
    }
}