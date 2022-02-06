extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

#[derive(Default, NwgPartial)]
pub struct NavPanel {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(position: (0, 0), size: (200, 580), item_count: 1, list_style: ListViewStyle::Detailed, ex_flags: nwg::ListViewExFlags::FULL_ROW_SELECT)]
    list: nwg::ListView
}

impl NavPanel {
    pub fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);
        self.layout.add_child((0, 0), (0, 100), &self.list);
        self.init_items();
    }

    fn init_items(&self) {
        let col = nwg::InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(195),
            text: None
        };
        self.list.insert_column(col);

        let elf_header = nwg::InsertListViewItem {
            index: None,
            column_index: 0,
            text: Some("ELF Header".to_owned())
        };
        self.list.insert_item(elf_header);
    }

    pub fn select(&self, item: usize) {
        self.list.select_item(item, true)
    }
}