extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;
use crate::elf::{Elf, EIdent};
use crate::utils;

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
        let e_ident_field = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: Some("e_ident".to_owned())
        };
        let e_ident_data = nwg::InsertListViewItem {
            index: Some(0),
            column_index: 2,
            text: Some(utils::raw_to_hex(elf.hdr.e_ident.raw.get()))
        };
        self.list.insert_item(e_ident_field);
        self.list.insert_item(e_ident_data);

        // populate e_ident view
        self.e_ident_view.populate(&elf.hdr.e_ident);

        // insert e_type field
        let e_type_field = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 0,
            text: Some("e_type".to_owned())
        };
        let e_type_value = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 1,
            text: Some(elf.hdr.e_type.to_str())
        };
        let e_type_data = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 2,
            text: Some(format!("{:X}", elf.hdr.e_type.val))
        };
        self.list.insert_item(e_type_field);
        self.list.insert_item(e_type_value);
        self.list.insert_item(e_type_data);

        // insert e_machine field
        let e_machine_field = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 0,
            text: Some("e_machine".to_owned())
        };
        let e_machine_value = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 1,
            text: Some(elf.hdr.e_machine.to_str())
        };
        let e_machine_data = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 2,
            text: Some(format!("{:X}", elf.hdr.e_machine.val))
        };
        self.list.insert_item(e_machine_field);
        self.list.insert_item(e_machine_value);
        self.list.insert_item(e_machine_data);

        // insert e_version field
        let e_version_field = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 0,
            text: Some("e_version".to_owned())
        };
        let e_version_value = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 1,
            text: Some(format!("0x{:x}", elf.hdr.e_version))
        };
        let e_version_data = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 2,
            text: Some(format!("{:X}", elf.hdr.e_version))
        };
        self.list.insert_item(e_version_field);
        self.list.insert_item(e_version_value);
        self.list.insert_item(e_version_data);

        // insert e_entry field
        let e_entry_field = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 0,
            text: Some("e_entry".to_owned())
        };
        let text = match elf.hdr.e_entry {
            crate::elf::ElfNAddr::Elf32Addr(val) => format!("0x{:x}", val),
            crate::elf::ElfNAddr::Elf64Addr(val) => format!("0x{:x}", val)
        };
        let e_entry_value = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 1,
            text: Some(text)
        };
        let text = match elf.hdr.e_entry {
            crate::elf::ElfNAddr::Elf32Addr(val) => utils::u32_to_hex(val, is_little_endian),
            crate::elf::ElfNAddr::Elf64Addr(val) => utils::u64_to_hex(val, is_little_endian)
        };
        let e_entry_data = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 2,
            text: Some(text)
        };
        self.list.insert_item(e_entry_field);
        self.list.insert_item(e_entry_value);
        self.list.insert_item(e_entry_data);

        // insert e_phoff field
        let e_phoff_field = nwg::InsertListViewItem {
            index: Some(5),
            column_index: 0,
            text: Some("e_phoff".to_owned())
        };
        let text = match elf.hdr.e_phoff {
            crate::elf::ElfNOff::Elf32Off(val) => format!("0x{:x}", val),
            crate::elf::ElfNOff::Elf64Off(val) => format!("0x{:x}", val)
        };
        let e_phoff_value = nwg::InsertListViewItem {
            index: Some(5),
            column_index: 1,
            text: Some(text)
        };
        let text = match elf.hdr.e_phoff {
            crate::elf::ElfNOff::Elf32Off(val) => utils::u32_to_hex(val, is_little_endian),
            crate::elf::ElfNOff::Elf64Off(val) => utils::u64_to_hex(val, is_little_endian)
        };
        let e_phoff_data = nwg::InsertListViewItem {
            index: Some(5),
            column_index: 2,
            text: Some(text)
        };
        self.list.insert_item(e_phoff_field);
        self.list.insert_item(e_phoff_value);
        self.list.insert_item(e_phoff_data);

        // insert e_shoff field
        let e_shoff_field = nwg::InsertListViewItem {
            index: Some(6),
            column_index: 0,
            text: Some("e_shoff".to_owned())
        };
        let text = match elf.hdr.e_shoff {
            crate::elf::ElfNOff::Elf32Off(val) => format!("0x{:x}", val),
            crate::elf::ElfNOff::Elf64Off(val) => format!("0x{:x}", val)
        };
        let e_shoff_value = nwg::InsertListViewItem {
            index: Some(6),
            column_index: 1,
            text: Some(text)
        };
        let text = match elf.hdr.e_shoff {
            crate::elf::ElfNOff::Elf32Off(val) => utils::u32_to_hex(val, is_little_endian),
            crate::elf::ElfNOff::Elf64Off(val) => utils::u64_to_hex(val, is_little_endian)
        };
        let e_shoff_data = nwg::InsertListViewItem {
            index: Some(6),
            column_index: 2,
            text: Some(text)
        };
        self.list.insert_item(e_shoff_field);
        self.list.insert_item(e_shoff_value);
        self.list.insert_item(e_shoff_data);
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
        let ei_class_field = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 0,
            text: Some("EI_CLASS".to_owned())
        };
        let ei_class_value = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 1,
            text: Some(e_ident.ei_class.to_str())
        };
        let ei_class_data = nwg::InsertListViewItem {
            index: Some(1),
            column_index: 2,
            text: Some(format!("{}", e_ident.ei_class.val))
        };
        self.list.insert_item(ei_class_field);
        self.list.insert_item(ei_class_value);
        self.list.insert_item(ei_class_data);

        // insert EI_DATA field
        let ei_data_field = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 0,
            text: Some("EI_DATA".to_owned())
        };
        let ei_data_value = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 1,
            text: Some(e_ident.ei_data.to_str())
        };
        let ei_data_data = nwg::InsertListViewItem {
            index: Some(2),
            column_index: 2,
            text: Some(format!("{}", e_ident.ei_data.val))
        };
        self.list.insert_item(ei_data_field);
        self.list.insert_item(ei_data_value);
        self.list.insert_item(ei_data_data);

        // insert EI_VERSION field
        let ei_version_field = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 0,
            text: Some("EI_VERSION".to_owned())
        };
        let ei_version_value = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 1,
            text: Some(e_ident.ei_version.to_str())
        };
        let ei_version_data = nwg::InsertListViewItem {
            index: Some(3),
            column_index: 2,
            text: Some(format!("{}", e_ident.ei_version.val))
        };
        self.list.insert_item(ei_version_field);
        self.list.insert_item(ei_version_value);
        self.list.insert_item(ei_version_data);

        // insert EI_OSABI field
        let ei_osabi_field = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 0,
            text: Some("EI_OSABI".to_owned())
        };
        let ei_osabi_value = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 1,
            text: Some(e_ident.ei_osabi.to_str())
        };
        let ei_osabi_data = nwg::InsertListViewItem {
            index: Some(4),
            column_index: 2,
            text: Some(format!("{}", e_ident.ei_osabi.val))
        };
        self.list.insert_item(ei_osabi_field);
        self.list.insert_item(ei_osabi_value);
        self.list.insert_item(ei_osabi_data);

        // insert EI_ABIVERSION field
        let ei_abiversion_field = nwg::InsertListViewItem {
            index: Some(5),
            column_index: 0,
            text: Some("EI_ABIVERSION".to_owned())
        };
        let ei_abiversion_data = nwg::InsertListViewItem {
            index: Some(5),
            column_index: 2,
            text: Some(format!("{}", e_ident.ei_abi_version))
        };
        self.list.insert_item(ei_abiversion_field);
        self.list.insert_item(ei_abiversion_data);

        // insert EI_PAD field
        let ei_pad_field = nwg::InsertListViewItem {
            index: Some(6),
            column_index: 0,
            text: Some("EI_PAD".to_owned())
        };
        let ei_pad_data = nwg::InsertListViewItem {
            index: Some(6),
            column_index: 2,
            text: Some(utils::raw_to_hex(e_ident.ei_pad.get()))
        };
        self.list.insert_item(ei_pad_field);
        self.list.insert_item(ei_pad_data);
    }
}