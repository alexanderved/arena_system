use crate::Index;
use crate::{Handle, RawHandle};

use std::cell::RefCell;

use vec_cell::{ElementRef, ElementRefMut, VecCell};

#[derive(Debug)]
pub struct Arena<T> {
    data: VecCell<Option<T>>,
    free: RefCell<Vec<usize>>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            data: VecCell::new(),
            free: RefCell::new(vec![]),
        }
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
        self.data.push(Some(value));
    }   

    pub(crate) fn try_borrow(&self, index: Index) -> Option<ElementRef<'_, Option<T>>> {
        self.data.try_borrow(index.into()).ok()
    }

    pub(crate) fn try_borrow_mut(&self, index: Index) -> Option<ElementRefMut<'_, Option<T>>> {
        self.data.try_borrow_mut(index.into()).ok()
    }
}