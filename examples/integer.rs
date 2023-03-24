use arena_system::*;

struct I32Handle<'arena> {
    raw: RawHandle<'arena, i32>,

    neighbours: Vec<Index>,
}

impl<'arena> I32Handle<'arena> {
    fn get_neighbours(&self) -> Vec<I32Handle<'arena>> {
        self.neighbours
            .iter()
            .map(|index| self.arena().handle(*index, self.neighbours.clone()))
            .collect()
    }
}

impl<'arena> Handle<'arena> for I32Handle<'arena> {
    type Type = i32;
    type Userdata = Vec<Index>;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> I32Handle<'arena> {
        Self {
            raw,
            neighbours: userdata
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

}