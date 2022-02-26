use native_windows_gui as nwg;

// Nav panel methods
impl super::ElfExplorer {
    pub fn nav_panel_init(&self) {
        self.nav_panel_layout.add_child((0, 0), (0, 100), &self.nav_panel_tree);
        self.nav_panel_init_items();

        let mut font = nwg::Font::default();

        nwg::Font::builder()
            .family("MS Shell Dlg")
            .size(16)
            .build(&mut font)
            .expect("Failed to build font");
        
        self.nav_panel_tree.set_font(Some(&font));
    }

    fn nav_panel_init_items(&self) {
        let tv = &self.nav_panel_tree;

        tv.insert_item("ELF Header", None, nwg::TreeInsert::Root);
        tv.insert_item("Program Headers", None, nwg::TreeInsert::Root);
        tv.insert_item("Section Headers", None, nwg::TreeInsert::Root);
    }

    pub fn set_all_frames_invisible(&self) {
        self.elf_header_frame.set_visible(false);
        self.pheaders_frame.set_visible(false);
        self.sheaders_frame.set_visible(false);
    }

    pub fn nav_panel_select_event(&self) {
        let item = self.nav_panel_tree.selected_item();
        if let None = item {
            return;
        }
        let item = item.unwrap();

        // create a shortcut to the function which sets the field description
        let set = |text: &str| self.field_desc.set(text);

        let text = self.nav_panel_tree.item_text(&item);
        if let None = text {
            return;
        }
        let text = text.unwrap();

        match &text[..] {
            "ELF Header" => {
                if !self.elf_header_frame.visible() {
                    self.elf_header_reset();
                    let elf = self.elf.borrow();
                    self.elf_header_populate(&elf.as_ref().unwrap())
                }
                self.set_all_frames_invisible();
                self.elf_header_frame.set_visible(true);
                set("The ELF header contains general information as well as the locations of the program and section header tables");
            }
            "Program Headers" => {
                if !self.pheaders_frame.visible() {
                    self.pheaders_reset();
                    let elf = self.elf.borrow();
                    self.pheaders_populate(&elf.as_ref().unwrap())
                }
                self.set_all_frames_invisible();
                self.pheaders_frame.set_visible(true);
                set("Program headers contain segments which describe the memory layout of the program and are necessary for loading it");
            }
            "Section Headers" => {
                if !self.sheaders_frame.visible() {
                    self.sheaders_reset();
                    let elf = self.elf.borrow();
                    self.sheaders_populate(&elf.as_ref().unwrap())
                }
                self.set_all_frames_invisible();
                self.sheaders_frame.set_visible(true);
                set("Section headers contain linking and debugging information");
            }
            _ => set("")
        }
    }
}