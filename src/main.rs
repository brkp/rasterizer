#![allow(
    dead_code,
    unused_imports,
    unused_variables)]

mod states;
mod renderer;

fn main() {
    arche::ContextBuilder::new("software renderer".to_owned(), 640, 480)
        .vsync(false)
        // .grab_mouse(true)
        // .show_mouse(false)
        .build()
        .unwrap()
        .run(|ctx| states::Main::new(ctx));
}
