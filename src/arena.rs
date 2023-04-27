use crate::{ArenaError, ArenaResult};
use crate::{Handle, RawHandle};
use crate::{Handleable, Index};

use std::{convert, iter};

use vec_cell::{ElementRef, ElementRefMut, Flatten, VecCell};

#[derive(Debug)]
pub struct Arena<T> {
    data: VecCell<Option<T>>,
    free: Vec<Index>,
}

impl<'arena, T: Handleable<'arena>> Arena<T> {
    pub fn new() -> Self {
        Self { data: VecCell::new(), free: vec![] }
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.free.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn handle(
        &'arena self,
        index: Index,
        userdata: <T::Handle as Handle<'arena>>::Userdata,
    ) -> T::Handle {
        let raw_handle = RawHandle::new(self, index);

        T::Handle::from_raw(raw_handle, userdata)
    }

    pub fn add(&mut self, value: T) -> Index {
        match self.free.pop() {
            Some(free_index) => {
                let mut free_place = self.lookup_mut(free_index).unwrap();
                *free_place = value;

                free_index
            }
            None => {
                self.data.push(Some(value));

                Index::from(self.data.len() - 1)
            }
        }
    }

    pub fn remove(&mut self, index: Index) -> ArenaResult<T> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        let mut element = self.data.try_take(index.into()).unwrap();
        match element.take() {
            Some(element) => Ok(element),
            None => Err(ArenaError::RemovedElementAccess),
        }
    }

    pub fn handle_iter(
        &'arena self,
        userdata: <T::Handle as Handle<'arena>>::Userdata,
    ) -> HandleIter<'arena, T> {
        HandleIter {
            arena: self,
            userdata,
            last_index: Index::new(0),
        }
    }

    pub fn lookup(&self, index: Index) -> ArenaResult<ElementRef<'_, T>> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        self.data.try_borrow(index.into()).flatten().map_err(ArenaError::from)
    }

    pub fn lookup_mut(&self, index: Index) -> ArenaResult<ElementRefMut<'_, T>> {
        if index.is_invalid() {
            return Err(ArenaError::InvalidIndexUsage);
        }

        self.data.try_borrow_mut(index.into()).flatten().map_err(ArenaError::from)
    }
}

impl<'arena, T: Handleable<'arena>> convert::From<Vec<T>> for Arena<T> {
    fn from(data: Vec<T>) -> Self {
        data.into_iter().collect()
    }
}

impl<'arena, T: Handleable<'arena>> iter::FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arena = Arena::new();
        iter.into_iter().for_each(|value| {
            arena.add(value);
        });

        arena
    }
}

pub struct HandleIter<'arena, T: Handleable<'arena>> {
    arena: &'arena Arena<T>,
    userdata: <T::Handle as Handle<'arena>>::Userdata,

    last_index: Index,
}

impl<'arena, T: Handleable<'arena>> iter::Iterator for HandleIter<'arena, T> {
    type Item = T::Handle;

    fn next(&mut self) -> Option<Self::Item> {
        let last_index: usize = self.last_index.into();
        if last_index >= self.arena.data.len() {
            return None;
        }

        let handle = self.arena.handle(self.last_index, self.userdata.clone());
        self.last_index = Index::from(last_index as i64 + 1);

        Some(handle)
    }
}
