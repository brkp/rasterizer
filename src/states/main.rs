use arche::prelude::*;
use arche::winit::event::VirtualKeyCode;
use arche::winit_input_helper::WinitInputHelper;

pub struct MainState;

impl MainState {
    pub fn new(_ctx: &mut Context) -> Box<dyn State> {
        Box::new(MainState {})
    }
}

impl State for MainState {
    fn handle_events(&mut self, _ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        Trans::None
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, Color::rgb(0x6495ed));
    }
}
