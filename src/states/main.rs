use arche::prelude::*;
use arche::winit::event::VirtualKeyCode;
use arche::winit_input_helper::WinitInputHelper;

use crate::renderer::scene::Scene;

pub struct MainState {
    scene: Scene,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Box<dyn State> {
        Box::new(MainState {
            scene: Scene::new(ctx),
        })
    }
}

impl State for MainState {
    fn on_start(&mut self, ctx: &mut Context) {
        self.scene
            .push_object(obj::Obj::load("assets/teapot.obj").unwrap());
    }

    fn handle_events(&mut self, _ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        Trans::None
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, Color::rgb(0x6495ed));

        self.scene.render(ctx);
    }
}
