use crate::Arena;
use crate::Index;

use vec_cell::{ElementRef, ElementRefMut};

pub trait Handle<'arena>
    where Self: 'arena
{
    type Type;
    type Userdata;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self;
    fn as_raw(&self) -> &RawHandle<'arena, Self::Type>;
    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type>;

    fn get(&self) -> Option<ElementRef<'arena, Self::Type>> {
        self.as_raw().get()
    }

    fn get_mut(&mut self) -> Option<ElementRefMut<'arena, Self::Type>> {
        self.as_mut_raw().get_mut()
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.as_raw().arena()
    }

    fn index(&self) -> Index<Self::Type> {
        self.as_raw().index()
    }
}

#[derive(Debug)]
pub struct RawHandle<'arena, T> {
    arena: &'arena Arena<T>,
    index: Index<T>,
}

impl<'arena, T> RawHandle<'arena, T> {
    pub fn get(&self) -> Option<ElementRef<'arena, T>> {
        self.arena.try_borrow(self.index)
    }

    pub fn get_mut(&mut self) -> Option<ElementRefMut<'arena, T>> {
        self.arena.try_borrow_mut(self.index)
    }

    pub fn arena(&self) -> &'arena Arena<T> {
        self.arena
    }

    pub fn index(&self) -> Index<T> {
        self.index
    }

    pub(crate) fn new(arena: &'arena Arena<T>, index: Index<T>) -> Self {
        Self { arena, index }
    }
}
