mod a {
    use arena_system::Arena;

    pub mod b {
        use arena_system::Handle;
        use arena_system_proc_macro::Handleable;

        #[derive(Handleable)]
        pub struct Test<const TEST: usize, T: Default>
        where
            T: Clone + Copy,
        {
            #[getter(name(test), vis(pub(in crate::a)), copy)]
            pub test_i32: T,
        }
    }

    pub fn test() {
        use b::*;

        let mut test_arena: Arena<Test<42, i32>> = Arena::new();
        test_arena.add(Test { test_i32: 1 });

        println!(
            "{}",
            test_arena
                .handle::<TestHandle<'_, 42, i32>>(0i64.into(), ())
                .test()
                .unwrap()
        );
    }
}

fn main() {
    a::test();
}
