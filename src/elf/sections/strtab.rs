use crate::utils::{self, RcSlice};

pub struct StrtabSection {
    pub strings: Vec<(usize, String)>,
}

impl StrtabSection {
    pub fn from(data: Option<RcSlice<u8>>) -> Self {
        let orig_data = match data {
            None => {
                return Self {
                    strings: Vec::<(usize, String)>::new(),
                }
            }
            Some(data) => data,
        };
        //return Self { strings: Vec::<(usize, String)>::new() };
        let data = orig_data.get();

        let mut strings = Vec::new();
        let mut offset: usize = 0;
        let mut string;
        while offset <= data.len() {
            let old_offset = offset;
            let tup = utils::raw_to_str(&data[offset..]);
            offset += tup.0 + 1;
            string = tup.1;
            if let Ok(string) = string {
                strings.push((old_offset, string.to_owned()));
            } else {
                strings.push((old_offset, "ERROR PARSING STRING".to_owned()));
            }
        }
        Self { strings }
    }
}
