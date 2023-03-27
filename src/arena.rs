use crate::Index;
use crate::{ArenaError, ArenaResult};
use crate::{Handle, RawHandle};

use std::iter;

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

    pub fn len(&self) -> usize {
        self.data.len() - self.free.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
            }
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

    pub fn iter<'arena>(&'arena self) -> HandleIter<'arena, T> {
        HandleIter { arena: self, last_index: Index::new(0) }
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

impl<T> iter::FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arena = Arena::new();
        iter.into_iter().for_each(|value| arena.add(value));

        arena
    }
}

pub struct HandleIter<'arena, T> {
    arena: &'arena Arena<T>,
    last_index: Index,
}

impl<'arena, T> iter::Iterator for HandleIter<'arena, T> {
    type Item = RawHandle<'arena, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let last_index: usize = self.last_index.into();
        if last_index >= self.arena.data.len() {
            return None;
        }

        let handle = self.arena.handle::<Self::Item>(self.last_index, ());

        self.last_index = Index::from(last_index as i64 + 1);

        Some(handle)
    }
}