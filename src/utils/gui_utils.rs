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