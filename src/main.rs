#![allow(
    dead_code,
    unused_imports,
    unused_variables)]

mod states;
mod renderer;

mod rand {
    extern "C" {
        fn rand() -> i32;
        fn srand(seed: u32);
    }

    pub fn seed(seed: u32) {
        unsafe { srand(seed) }
    }

    pub fn u32() -> u32 {
        unsafe { rand() as u32 }
    }

    pub fn range(a: i32, b: i32) -> i32 {
        return (u32() as i32 % (b - a + 1)) + a;
    }
}

use std::time::SystemTime;

fn main() {
    rand::seed(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    );

    arche::ContextBuilder::new("software renderer".to_owned(), 1024, 768)
        .vsync(false)
        .grab_mouse(true)
        // .show_mouse(false)
        .build()
        .unwrap()
        .run(|ctx| states::Main::new(ctx));
}
