use crate::Arena;
use crate::ArenaResult;
use crate::Index;

use std::cmp;
use std::fmt;

use vec_cell::{ElementRef, ElementRefMut};

#[derive(Debug, Clone, Copy)]
pub enum Void {}

pub type EmptyUserdata = Option<Void>;

pub trait Handleable<'arena> {
    type Handle: Handle<'arena, Type = Self>;
}

pub trait Handle<'arena>
where
    Self: 'arena,
{
    type Type: Handleable<'arena>;
    type Userdata: Clone;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self;
    fn to_raw(&self) -> RawHandle<'arena, Self::Type>;

    fn get(&self) -> ArenaResult<ElementRef<'arena, Self::Type>> {
        self.to_raw().get()
    }

    fn get_mut(&self) -> ArenaResult<ElementRefMut<'arena, Self::Type>> {
        self.to_raw().get_mut()
    }

    fn exists(&self) -> bool {
        self.to_raw().get().is_ok()
    }

    fn arena(&self) -> &'arena Arena<Self::Type> {
        self.to_raw().arena()
    }

    fn index(&self) -> Index {
        self.to_raw().index()
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

impl<'arena, T: Handleable<'arena>> Handle<'arena> for RawHandle<'arena, T> {
    type Type = T;
    type Userdata = EmptyUserdata;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> Self {
        raw
    }

    fn to_raw(&self) -> RawHandle<'arena, Self::Type> {
        *self
    }

    fn get(&self) -> ArenaResult<ElementRef<'arena, Self::Type>> {
        self.arena().lookup(self.index())
    }

    fn get_mut(&self) -> ArenaResult<ElementRefMut<'arena, Self::Type>> {
        self.arena().lookup_mut(self.index())
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

impl<'arena, T: Handleable<'arena>> Clone for RawHandle<'arena, T> {
    fn clone(&self) -> Self {
        Self::new(self.arena(), self.index())
    }
}

impl<'arena, T: Handleable<'arena>> Copy for RawHandle<'arena, T> {}

impl<'arena, T: Handleable<'arena>> cmp::PartialEq for RawHandle<'arena, T> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl<'arena, T: Handleable<'arena>> cmp::Eq for RawHandle<'arena, T> {}

impl<'arena, T: Handleable<'arena>> cmp::PartialOrd for RawHandle<'arena, T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<'arena, T: Handleable<'arena>> cmp::Ord for RawHandle<'arena, T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.index.cmp(&other.index)
    }
}
