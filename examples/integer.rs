use arena_system::*;

#[derive(Debug)]
struct I32Handle<'arena> {
    raw: RawHandle<'arena, i32>,
}

impl<'arena> Handle<'arena> for I32Handle<'arena> {
    type Type = i32;
    type Userdata = ();

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> I32Handle<'arena> {
        Self {
            raw,
        }
    }

    fn as_raw(&self) -> &RawHandle<'arena, Self::Type> {
        &self.raw
    }

    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type> {
        &mut self.raw
    }
}

fn main() {
    let arena: Arena<i32> = Arena::from(vec![0, 1, 2, 3, 4, 5]);
    let handle: I32Handle = arena.handle(1.into(), ());

    println!("{:#?}", handle);
}