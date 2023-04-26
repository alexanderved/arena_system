use arena_system::{Arena, Handle, vec_cell::ElementRef};
use arena_system_proc_macro::Handleable;

#[derive(Handleable)]
struct Test<const TEST: usize, T: Default> where T: Clone {
    #[getter(name = "test")]
    test_i32: T,
    //_p: std::marker::PhantomData<&'a T>,
}

/* impl<'arena, const TEST: usize, T: Default> TestHandle<'arena, TEST, T>
    where T: Clone
{
    fn test_i32(&self) -> Option<ElementRef<'arena, T>> {
        self
            .get()
            .ok()
            .map(|this_ref| ElementRef::map(this_ref, |this| {
                &this.test_i32
            }))
    }
} */

fn main() {
    let mut test_arena: Arena<Test<42, i32>> = Arena::new();
    test_arena.add(Test { test_i32: 1 });

    println!("{}", *test_arena.handle::<TestHandle<'_, 42, i32>>(0i64.into(), ()).test().unwrap());
}