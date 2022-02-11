extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;
use crate::elf::{Elf, EIdent, Description, ElfNAddr, ElfNOff};
use crate::{utils, descriptive_field, address_field, offset_field, size_field, raw_field, hex_field, decimal_field};

// GRID, FULL_ROW_SELECT
const FLAGS: nwg::ListViewExFlags = nwg::ListViewExFlags::from_bits_truncate(nwg::ListViewExFlags::GRID.bits() | nwg::ListViewExFlags::FULL_ROW_SELECT.bits());

#[derive(Default, NwgPartial)]
pub struct ElfHeaderView {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_events(OnListViewClick: [ElfHeaderView::select])]
    #[nwg_control(position: (0, 0), size: (600, 348), item_count: 1, list_style: ListViewStyle::Detailed, ex_flags: FLAGS)]
    list: nwg::ListView,

    // e_ident view
    #[nwg_control(position: (0, 348), size: (600, 232), flags: "NONE")]
    e_ident_frame: nwg::Frame,
    
    #[nwg_partial(parent: e_ident_frame)]
    e_ident_view: EIdentView
}

impl ElfHeaderView {
    pub fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);
        self.list.set_headers_enabled(true);
        self.layout.add_child((0, 0), (100, 60), &self.list);
        self.init_colummns();

        self.e_ident_frame.set_visible(false);
        self.layout.add_child((0, 60), (100, 40), &self.e_ident_frame);
        self.e_ident_view.init(&self.e_ident_frame)
    }

    fn init_colummns(&self) {
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
        self.list.insert_column(field_col);
        self.list.insert_column(value_col);
        self.list.insert_column(data_col);
    }

    pub fn reset(&self) {
        self.list.clear();
        self.e_ident_view.reset();
        self.e_ident_frame.set_visible(false);
    }

    pub fn populate(&self, elf: &Elf) {
        self.list.clear();

        // get byte order
        let is_little_endian = elf.is_little_endian();

        // insert e_ident field
        raw_field!("e_ident", elf.hdr.e_ident.raw, self.list, 0);

        // populate e_ident view
        self.e_ident_view.populate(&elf.hdr.e_ident);

        // insert e_type field
        descriptive_field!("e_type", elf.hdr.e_type, self.list, 1);

        // insert e_machine field
        descriptive_field!("e_machine", elf.hdr.e_machine, self.list, 2);

        // insert e_version field
        hex_field!("e_version", elf.hdr.e_version, self.list, 3);

        // insert e_entry field
        address_field!("e_entry", elf.hdr.e_entry, self.list, 4, is_little_endian);

        // insert e_phoff field
        offset_field!("e_phoff", elf.hdr.e_phoff, self.list, 5, is_little_endian);

        // insert e_shoff field
        offset_field!("e_shoff", elf.hdr.e_shoff, self.list, 6, is_little_endian);

        // insert e_flags field
        hex_field!("e_flags", elf.hdr.e_flags, self.list, 7);

        // insert e_ehsize field
        size_field!("e_ehsize", elf.hdr.e_ehsize, self.list, 8);

        // insert e_phentsize field
        size_field!("e_phentsize", elf.hdr.e_phentsize, self.list, 9);

        // insert e_phnum field
        decimal_field!("e_phnum", elf.hdr.e_phnum, self.list, 10);

        // insert e_shentsize field
        size_field!("e_shentsize", elf.hdr.e_shentsize, self.list, 11);

        // insert e_shnum field
        decimal_field!("e_shnum", elf.hdr.e_shnum, self.list, 12);

        // insert e_shstrndx field
        decimal_field!("e_shstrndx", elf.hdr.e_shstrndx, self.list, 13);
    }

    fn select(&self) {
        if let Some(item) = self.list.selected_item() {
            match item {
                0 => {
                    self.e_ident_frame.set_visible(true);
                }
                _ => ()
            }
        }
    }
}

#[derive(Default, NwgPartial)]
struct EIdentView {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(position: (0, 0), size: (600, 232), item_count: 1, list_style: ListViewStyle::Detailed, ex_flags: FLAGS)]
    list: nwg::ListView
}

impl EIdentView {
    fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);
        self.list.set_headers_enabled(true);
        self.layout.add_child((0, 0), (100, 100), &self.list);
        self.init_colummns();
    }

    fn init_colummns(&self) {
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
        self.list.insert_column(field_col);
        self.list.insert_column(value_col);
        self.list.insert_column(data_col);
    }

    fn reset(&self) {
        self.list.clear();
    }

    fn populate(&self, e_ident: &EIdent) {
        self.list.clear();

        // insert EI_MAG field
        let ei_mag_field = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: Some("EI_MAG".to_owned())
        };
        let ei_mag_value = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: Some(format!("0x{:x}  {}{}{}", e_ident.ei_mag0, e_ident.ei_mag1 as char, e_ident.ei_mag2 as char, e_ident.ei_mag3 as char))
        };
        let ei_mag_data = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 2,
            text: Some(format!("{:X} {:X} {:X} {:X}", e_ident.ei_mag0, e_ident.ei_mag1, e_ident.ei_mag2, e_ident.ei_mag3))
        };
        self.list.insert_item(ei_mag_field);
        self.list.insert_item(ei_mag_value);
        self.list.insert_item(ei_mag_data);

        // insert EI_CLASS field
        descriptive_field!("EI_CLASS", e_ident.ei_class, self.list, 1);

        // insert EI_DATA field
        descriptive_field!("EI_DATA", e_ident.ei_data, self.list, 2);

        // insert EI_VERSION field
        descriptive_field!("EI_VERSION", e_ident.ei_version, self.list, 3);

        // insert EI_OSABI field
        descriptive_field!("EI_OSABI", e_ident.ei_osabi, self.list, 4);

        // insert EI_ABIVERSION field
        decimal_field!("EI_ABIVERSION", e_ident.ei_abi_version, self.list, 5);

        // insert EI_PAD field
        raw_field!("EI_PAD", e_ident.ei_pad, self.list, 6);
    }
}