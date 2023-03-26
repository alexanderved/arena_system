use crate::Index;
use crate::{Handle, RawHandle};
use crate::{ArenaResult, ArenaError};

use vec_cell::{ElementRef, ElementRefMut, VecCell};

#[derive(Debug)]
pub struct Arena<T> {
    data: VecCell<Option<T>>,
    free: Vec<Index>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { data: VecCell::new(), free: vec![] }
    }

    pub fn handle<'arena, H: Handle<'arena, Type = T>>(
        &'arena self,
        index: Index,
        userdata: H::Userdata,
    ) -> H {
        let raw_handle = RawHandle::new(self, index);

        H::from_raw(raw_handle, userdata)
    }

    pub fn add(&mut self, value: T) {
        match self.free.pop() {
            Some(free_index) => {
                let mut free_place = self.try_borrow_mut(free_index).unwrap();
                *free_place = Some(value);
            },
            None => self.data.push(Some(value)),
        }
    }

    pub fn remove(&mut self, index: Index) -> ArenaResult<T> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        let mut element = self.try_borrow_mut(index).unwrap();
        match element.take() {
            Some(element) => Ok(element),
            None => Err(ArenaError::RemovedElementAccess),
        }
    }

    pub(crate) fn try_borrow(&self, index: Index) -> ArenaResult<ElementRef<'_, Option<T>>> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        Ok(self.data.try_borrow(index.into())?)
    }

    pub(crate) fn try_borrow_mut(&self, index: Index) -> ArenaResult<ElementRefMut<'_, Option<T>>> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        Ok(self.data.try_borrow_mut(index.into())?)
    }
}
