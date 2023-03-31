use crate::Arena;
use crate::Index;

use std::fmt;
use std::cmp;

use vec_cell::{ElementRef, ElementRefMut};

pub trait Handle<'arena>
where
    Self: 'arena,
{
    type Type;
    type Userdata: Clone;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self;
    fn as_raw(&self) -> &RawHandle<'arena, Self::Type>;

    fn get(&self) -> Option<ElementRef<'arena, Self::Type>> {
        self.as_raw().get()
    }

    fn get_mut(&self) -> Option<ElementRefMut<'arena, Self::Type>> {
        self.as_raw().get_mut()
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.as_raw().arena()
    }

    fn index(&self) -> Index {
        self.as_raw().index()
    }
}

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

    fn get(&self) -> Option<ElementRef<'arena, Self::Type>> {
        self.arena().try_borrow(self.index())
    }

    fn get_mut(&self) -> Option<ElementRefMut<'arena, Self::Type>> {
        self.arena().try_borrow_mut(self.index())
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.arena
    }

    fn index(&self) -> Index {
        self.index
    }
}

impl<T> fmt::Debug for RawHandle<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Handle({})", <Index as Into<i64>>::into(self.index)))
    }
}

impl<T> Clone for RawHandle<'_, T> {
    fn clone(&self) -> Self {
        Self::new(self.arena(), self.index())
    }
}

impl<T> Copy for RawHandle<'_, T> {}

impl<T> cmp::PartialEq for RawHandle<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl<T> cmp::Eq for RawHandle<'_, T> {}

impl<T> cmp::PartialOrd for RawHandle<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T> cmp::Ord for RawHandle<'_, T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.index.cmp(&other.index)
    }
}