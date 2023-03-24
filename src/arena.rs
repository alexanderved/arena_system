use crate::Index;
use crate::{Handle, RawHandle};

use vec_cell::{ElementRef, ElementRefMut, VecCell};

pub struct Arena<T> {
    data: VecCell<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            data: VecCell::new(),
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

    pub(crate) fn try_borrow(&self, index: Index) -> Option<ElementRef<'_, T>> {
        self.data.try_borrow(index.into()).ok()
    }

    pub(crate) fn try_borrow_mut(&self, index: Index) -> Option<ElementRefMut<'_, T>> {
        self.data.try_borrow_mut(index.into()).ok()
    }
}
