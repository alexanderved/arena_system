use crate::ArenaResult;
use crate::Arena;
use crate::Index;

use vec_cell::{ElementRef, ElementRefMut};

pub trait Handle<'arena>
where
    Self: 'arena,
{
    type Type;
    type Userdata;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self;
    fn as_raw(&self) -> &RawHandle<'arena, Self::Type>;
    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type>;

    fn get(&self) -> ArenaResult<ElementRef<'arena, Option<Self::Type>>> {
        self.as_raw().get()
    }

    fn get_mut(&mut self) -> ArenaResult<ElementRefMut<'arena, Option<Self::Type>>> {
        self.as_mut_raw().get_mut()
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.as_raw().arena()
    }

    fn index(&self) -> Index {
        self.as_raw().index()
    }
}

#[derive(Debug)]
pub struct RawHandle<'arena, T> {
    arena: &'arena Arena<T>,
    index: Index,
}

impl<'arena, T> RawHandle<'arena, T> {
    pub(crate) fn new(arena: &'arena Arena<T>, index: Index) -> Self {
        Self { arena, index }
    }
}

impl<'arena, T> Handle<'arena> for RawHandle<'arena, T> {
    type Type = T;
    type Userdata = ();

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> Self {
        raw
    }

    fn as_raw(&self) -> &RawHandle<'arena, Self::Type> {
        self
    }

    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type> {
        self
    }

    fn get(&self) -> ArenaResult<ElementRef<'arena, Option<Self::Type>>> {
        self.arena().try_borrow(self.index())
    }

    fn get_mut(&mut self) -> ArenaResult<ElementRefMut<'arena, Option<Self::Type>>> {
        self.arena().try_borrow_mut(self.index())
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.arena
    }

    fn index(&self) -> Index {
        self.index
    }
}
