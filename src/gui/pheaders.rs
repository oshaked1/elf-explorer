use native_windows_gui as nwg;

// Program headers methods
impl super::ElfExplorer {
    pub fn pheaders_init(&self) {
        self.pheaders_list.set_headers_enabled(true);
        self.pheaders_layout.add_child((0, 0), (100, 60), &self.pheaders_list);
        self.pheaders_init_colummns();
    }

    fn pheaders_init_colummns(&self) {
        let p_type_col = nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(70),
            text: Some("p_type".to_owned())
        };
        let p_offset_col = nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(70),
            text: Some("p_offset".to_owned())
        };
        let p_vaddr_col = nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(70),
            text: Some("p_vaddr".to_owned())
        };
        let p_paddr_col = nwg::InsertListViewColumn {
            index: Some(3),
            fmt: None,
            width: Some(70),
            text: Some("p_paddr".to_owned())
        };
        let p_filesz_col = nwg::InsertListViewColumn {
            index: Some(4),
            fmt: None,
            width: Some(70),
            text: Some("p_filesz".to_owned())
        };
        let p_memsz_col = nwg::InsertListViewColumn {
            index: Some(5),
            fmt: None,
            width: Some(70),
            text: Some("p_memsz".to_owned())
        };
        let p_flags_col = nwg::InsertListViewColumn {
            index: Some(6),
            fmt: None,
            width: Some(70),
            text: Some("p_flags".to_owned())
        };
        let p_align_col = nwg::InsertListViewColumn {
            index: Some(7),
            fmt: None,
            width: Some(70),
            text: Some("p_align".to_owned())
        };
        self.pheaders_list.insert_column(p_type_col);
        self.pheaders_list.insert_column(p_offset_col);
        self.pheaders_list.insert_column(p_vaddr_col);
        self.pheaders_list.insert_column(p_paddr_col);
        self.pheaders_list.insert_column(p_filesz_col);
        self.pheaders_list.insert_column(p_memsz_col);
        self.pheaders_list.insert_column(p_flags_col);
        self.pheaders_list.insert_column(p_align_col);
    }

    pub fn pheaders_select_event(&self) {
        if let Some(item) = self.pheaders_list.selected_item() {
            // create a shortcut to the function which sets the field description
            let set = |text: &str| self.field_desc.set(text);

            match item {
                _ => ()
            };
        }
    }
}