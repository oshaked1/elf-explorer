#[macro_export]
macro_rules! descriptive_field {
    ($name:expr, $val:expr, $list:expr, $row:expr) => {
        let field = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 0,
            text: Some($name.to_owned())
        };
        let value = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 1,
            text: Some($val.to_str())
        };
        let data = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 2,
            text: Some(format!("0x{:x}", $val.0))
        };
        $list.insert_item(field);
        $list.insert_item(value);
        $list.insert_item(data);
    };
}

#[macro_export]
macro_rules! address_field {
    ($name:expr, $val:expr, $list:expr, $row:expr, $is_little_endian:expr) => {
        let field = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 0,
            text: Some($name.to_owned())
        };
        let text = match $val {
            ElfNAddr::Elf32Addr(val) => format!("0x{:x}", val),
            ElfNAddr::Elf64Addr(val) => format!("0x{:x}", val)
        };
        let value = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 1,
            text: Some(text)
        };
        let text = match $val {
            ElfNAddr::Elf32Addr(val) => utils::u32_to_hex(val, $is_little_endian),
            ElfNAddr::Elf64Addr(val) => utils::u64_to_hex(val, $is_little_endian)
        };
        let data = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 2,
            text: Some(text)
        };
        $list.insert_item(field);
        $list.insert_item(value);
        $list.insert_item(data);
    };
}

#[macro_export]
macro_rules! offset_field {
    ($name:expr, $val:expr, $list:expr, $row:expr, $is_little_endian:expr) => {
        let field = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 0,
            text: Some($name.to_owned())
        };
        let text = match $val {
            ElfNOff::Elf32Off(val) => format!("0x{:x}", val),
            ElfNOff::Elf64Off(val) => format!("0x{:x}", val)
        };
        let value = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 1,
            text: Some(text)
        };
        let text = match $val {
            ElfNOff::Elf32Off(val) => utils::u32_to_hex(val, $is_little_endian),
            ElfNOff::Elf64Off(val) => utils::u64_to_hex(val, $is_little_endian)
        };
        let data = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 2,
            text: Some(text)
        };
        $list.insert_item(field);
        $list.insert_item(value);
        $list.insert_item(data);
    };
}

#[macro_export]
macro_rules! size_field {
    ($name:expr, $val:expr, $list:expr, $row:expr) => {
        let field = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 0,
            text: Some($name.to_owned())
        };
        let value = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 1,
            text: Some(format!("{} bytes", $val))
        };
        let data = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 2,
            text: Some(format!("0x{:x}", $val))
        };
        $list.insert_item(field);
        $list.insert_item(value);
        $list.insert_item(data);
    };
}

#[macro_export]
macro_rules! raw_field {
    ($name:expr, $val:expr, $list:expr, $row:expr) => {
        let field = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 0,
            text: Some($name.to_owned())
        };
        let data = nwg::InsertListViewItem {
            index: Some($row),
            column_index: 2,
            text: Some(utils::raw_to_hex($val.get()))
        };
        $list.insert_item(field);
        $list.insert_item(data);
    };
}