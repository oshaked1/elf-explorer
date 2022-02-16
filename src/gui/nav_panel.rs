use native_windows_gui as nwg;

// Nav panel methods
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
            index: Some(0),
            column_index: 0,
            text: Some("ELF Header".to_owned())
        };
        self.nav_panel_list.insert_item(elf_header);

        let pheaders = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 0,
            text: Some("Program Headers".to_owned())
        };
        self.nav_panel_list.insert_item(pheaders);
    }

    pub fn set_all_frames_invisible(&self) {
        self.elf_header_frame.set_visible(false);
        self.pheaders_frame.set_visible(false);
    }

    pub fn nav_panel_select_event(&self) {
        if let Some(item) = self.nav_panel_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                0 => {
                    if !self.elf_header_frame.visible() {
                        self.elf_header_reset();
                        let elf = self.elf.borrow();
                        self.elf_header_populate(&elf.as_ref().unwrap())
                    }
                    self.set_all_frames_invisible();
                    self.elf_header_frame.set_visible(true);
                    set("The ELF header contains general information as well as the locations of the program and section header tables");
                }
                1 => {
                    if !self.pheaders_frame.visible() {
                        self.pheaders_reset();
                        let elf = self.elf.borrow();
                        self.pheaders_populate(&elf.as_ref().unwrap())
                    }
                    self.set_all_frames_invisible();
                    self.pheaders_frame.set_visible(true);
                    set("Program headers contain segments which describe the memory layout of the program and are necessary for loading it");
                }
                _ => set("")
            }
        }
    }
}