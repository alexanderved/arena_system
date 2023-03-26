use std::convert;

#[derive(Debug, Clone, Copy)]
pub struct Index {
    index: i64,
}

impl Index {
    pub fn new(index: i64) -> Self {
        Self {
            index
        }
    }

    pub fn invalid() -> Self {
        Self::new(-1)
    }
}

impl convert::Into<usize> for Index {
    fn into(self) -> usize {
        if self.index < 0 {
            panic!("Trying to convert invalid Index to usize");
        }

        self.index as usize
    }
}

impl convert::From<usize> for Index {
    fn from(index: usize) -> Self {
        Self::new(index as i64)
    }
}