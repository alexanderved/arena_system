use std::convert;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
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

    pub fn is_invalid(&self) -> bool {
        self.index < 0
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

impl convert::Into<i64> for Index {
    fn into(self) -> i64 {
        self.index
    }
}

impl convert::From<usize> for Index {
    fn from(index: usize) -> Self {
        Self::new(index as i64)
    }
}

impl convert::From<i64> for Index {
    fn from(index: i64) -> Self {
        Self::new(index)
    }
}