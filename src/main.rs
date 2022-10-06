mod states;

fn main() {
    arche::ContextBuilder::new("software renderer".to_owned(), 800, 600)
        .vsync(false)
        .grab_mouse(true)
        .show_mouse(false)
        .build()
        .unwrap()
        .run(|ctx| states::Main::new(ctx));
}
