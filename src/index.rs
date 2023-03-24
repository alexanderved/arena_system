use std::convert;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Index<T> {
    index: usize,
    _p: PhantomData<T>,
}

impl<T> Index<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _p: PhantomData,
        }
    }
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Index<T> {}

impl<T> convert::Into<usize> for Index<T> {
    fn into(self) -> usize {
        self.index
    }
}

impl<T> convert::From<usize> for Index<T> {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}