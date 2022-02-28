use crate::utils::RcSlice;

#[derive(Debug)]
pub struct StrtabSection {
    todo: usize
}

impl StrtabSection {
    pub fn from(data: Option<RcSlice<u8>>) -> Self {
        Self { todo: 0 }
    }
}