use native_windows_gui as nwg;
use nwg::TreeItem;

use crate::elf::Elf;
use crate::elf::sections::SectionType;

mod strtab;

impl super::ElfExplorer {
    pub fn section_nav_select_event(&self, item: &TreeItem, elf: &Elf) {
        if self.nav_panel_item_depth(item) != 1 {
            return
        }

        // create a shortcut to the function which sets the field description
        let set = |text: &str| self.field_desc.set(text);

        let text = match self.nav_panel_tree.item_text(item) {
            None => return,
            Some(text) => text
        };

        let index = text.split(":").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let section = &elf.sections.0[index];

        if let Some(name) = &section.name {
            set(&format!("{} section", name));
        }
        else {
            set("Unknown section");
        }

        match &section.section_type {
            SectionType::Strtab(strtab) => {
                self.set_all_frames_invisible();
                self.strtab_populate(strtab);
                self.strtab_frame.set_visible(true);
            }
            SectionType::Generic => {
                self.section_unimplemented(&section.type_name());
            }
        }
    }

    fn section_unimplemented(&self, section_type: &str) {
        self.set_all_frames_invisible();
        self.unimplemented_frame.set_visible(true);
        self.unimplemented_message.set_text(&format!("Information for {} sections is not implemented yet.", section_type));
    }
}