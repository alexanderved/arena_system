use crate::Index;
use crate::{ArenaError, ArenaResult};
use crate::{Handle, RawHandle};

use std::{iter, convert};

use vec_cell::{ElementRef, ElementRefMut, Flatten, VecCell};

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
            },
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

    pub fn handle_iter<'arena, H: Handle<'arena, Type = T>>(
        &'arena self,
        userdata: H::Userdata,
    ) -> HandleIter<'arena, T, H> {
        HandleIter {
            arena: self,
            userdata,
            last_index: Index::new(0),
            _p: std::marker::PhantomData,
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

impl<T> convert::From<Vec<T>> for Arena<T> {
    fn from(data: Vec<T>) -> Self {
        data.into_iter().collect()
    }
}

impl<T> iter::FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arena = Arena::new();
        iter.into_iter().for_each(|value| { arena.add(value); });

        arena
    }
}

pub struct HandleIter<'arena, T, H: Handle<'arena, Type = T>> {
    arena: &'arena Arena<T>,
    userdata: H::Userdata,

    last_index: Index,

    _p: std::marker::PhantomData<H>,
}

impl<'arena, T, H: Handle<'arena, Type = T>> iter::Iterator for HandleIter<'arena, T, H> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        let last_index: usize = self.last_index.into();
        if last_index >= self.arena.data.len() {
            return None;
        }

        let mut handle = self.arena.handle::<Self::Item>(self.last_index, self.userdata.clone());
        self.last_index = Index::from(last_index as i64 + 1);

        Some(handle)
    }
}
