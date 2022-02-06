use std::rc::Rc;

pub struct RcSlice<T> {
    rc: Rc<Vec<T>>,
    start: usize,
    end: usize,
}

impl<T> RcSlice<T> {
    pub fn new(rc: Rc<Vec<T>>, start: usize, end: usize) -> Self {
        assert!(end <= rc.len());
        Self { rc, start, end }
    }

    pub fn from(rcslice: &RcSlice<T>, start: usize, end: usize) -> Self {
        assert!(rcslice.start + end <= rcslice.end);
        Self {
            rc: rcslice.rc.clone(),
            start: rcslice.start + start,
            end: rcslice.start + end,
        }
    }

    pub fn get(&self) -> &[T] {
        &self.rc[self.start..self.end]
    }
}
