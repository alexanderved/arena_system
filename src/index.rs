use std::convert;

#[derive(Debug, Clone, Copy)]
pub struct Index {
    index: usize,
}

impl Index {
    pub fn new(index: usize) -> Self {
        Self {
            index
        }
    }
}

impl convert::Into<usize> for Index {
    fn into(self) -> usize {
        self.index
    }
}

impl convert::From<usize> for Index {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}