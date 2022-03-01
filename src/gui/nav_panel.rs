use native_windows_gui as nwg;
use nwg::TreeItem;

use crate::elf::Elf;

// Nav panel methods
impl super::ElfExplorer {
    pub fn nav_panel_init(&self) {
        self.nav_panel_layout
            .add_child((0, 0), (0, 100), &self.nav_panel_tree);

        let mut font = nwg::Font::default();

        nwg::Font::builder()
            .family("MS Shell Dlg")
            .size(15)
            .build(&mut font)
            .expect("Failed to build font");

        self.nav_panel_tree.set_font(Some(&font));
    }

    pub fn nav_panel_init_items(&self, elf: &Elf) {
        let tv = &self.nav_panel_tree;
        tv.clear();

        tv.insert_item("ELF Header", None, nwg::TreeInsert::Root);
        tv.insert_item("Program Headers", None, nwg::TreeInsert::Root);
        let sheaders = tv.insert_item("Section Headers", None, nwg::TreeInsert::Root);

        self.sheaders_init_navigation_items(&sheaders, elf);
    }

    pub fn nav_panel_select_event(&self) {
        let tree = &self.nav_panel_tree;

        let elf = self.elf.borrow();
        let elf = elf.as_ref().unwrap();

        let item = tree.selected_item();
        if let None = item {
            return;
        }
        let item = item.unwrap();

        // create a shortcut to the function which sets the field description
        let set = |text: &str| self.field_desc.set(text);

        match self.nav_panel_item_root(&item) {
            None => {
                let text = match self.nav_panel_tree.item_text(&item) {
                    None => return,
                    Some(text) => text,
                };

                match &text[..] {
                    "ELF Header" => {
                        if !self.elf_header_frame.visible() {
                            self.elf_header_reset();
                            self.elf_header_populate(elf)
                        }
                        self.set_all_frames_invisible();
                        self.elf_header_frame.set_visible(true);
                        set("The ELF header contains general information as well as the locations of the program and section header tables");
                    }
                    "Program Headers" => {
                        if !self.pheaders_frame.visible() {
                            self.pheaders_reset();
                            self.pheaders_populate(elf)
                        }
                        self.set_all_frames_invisible();
                        self.pheaders_frame.set_visible(true);
                        set("Program headers contain segments which describe the memory layout of the program and are necessary for loading it");
                    }
                    "Section Headers" => {
                        if !self.sheaders_frame.visible() {
                            self.sheaders_reset();
                            self.sheaders_populate(elf)
                        }
                        self.set_all_frames_invisible();
                        self.sheaders_frame.set_visible(true);
                        set("Section headers contain linking and debugging information");
                    }
                    _ => set(""),
                }
            }
            Some(root) => {
                let text = match self.nav_panel_tree.item_text(&root) {
                    None => return,
                    Some(text) => text,
                };

                match &text[..] {
                    "Section Headers" => self.section_nav_select_event(&item, elf),
                    _ => (),
                }
            }
        }
    }

    pub fn nav_panel_item_depth(&self, item: &TreeItem) -> u32 {
        let tree = &self.nav_panel_tree;
        let mut depth = 0;

        let mut parent = tree.parent(item);
        while parent != None {
            depth += 1;
            parent = tree.parent(&parent.unwrap());
        }
        depth
    }

    fn nav_panel_item_root(&self, item: &TreeItem) -> Option<TreeItem> {
        self.nav_panel_item_parent(item, 0)
    }

    fn nav_panel_item_parent(&self, item: &TreeItem, depth: u32) -> Option<TreeItem> {
        let mut current_depth = self.nav_panel_item_depth(item);
        if depth >= current_depth {
            return None;
        }

        let mut parent = self.nav_panel_tree.parent(item);
        current_depth -= 1;
        while current_depth > depth {
            parent = self.nav_panel_tree.parent(&parent.unwrap());
            current_depth -= 1;
        }
        parent
    }
}
