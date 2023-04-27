mod a {
    use arena_system::Arena;

    pub mod b {
        #[derive(arena_system_proc_macro::Handleable, Debug)]
        pub struct Test<const TEST: usize, T: Default>
        where
            T: Clone + Copy,
        {
            #[handle_getter(name(test), vis(pub(in crate::a)), return_type(copy))]
            #[handle_setter(name(set_test), vis(pub(in crate::a)), input_type(value))]
            pub test_t: T,
            #[handle_getter(return_type(copy))]
            pub test_i32: i32,
            #[handle_getter(return_type(handle(tests: Arena<Test<24, u32>>)))]
            pub test_index: usize,
        }
    }

    pub fn test() {
        use b::*;
        use arena_system::Handle;

        let mut test_arena: Arena<Test<42, u32>> = Arena::new();
        test_arena.add(Test { test_t: 1, test_i32: 42, test_index: 0 });

        let test_handle = test_arena.handle(0i64.into(), None);

        println!("Test: {:?}", test_handle.test().unwrap());
        test_handle.set_test(100);
        println!("Test: {:?}", test_handle.test().unwrap());

        println!("Test i32: {:?}", test_handle.test_i32().unwrap());
        test_handle.set_test_i32(-100);
        println!("Test i32: {:?}", test_handle.test_i32().unwrap());

        println!("Test index: {:?}", test_handle.test_index().unwrap().get());
    }
}

fn main() {
    a::test();
}
