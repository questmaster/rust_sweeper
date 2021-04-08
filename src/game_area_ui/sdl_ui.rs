use crate::game_area::{GameArea, EvaluationResult};
use crate::ui::GameUi;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Sdl {
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Sdl {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rust Sweeper - SDL Edition", 600, 600)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        let mut event_pump = sdl_context.event_pump().unwrap();

        Sdl {canvas, event_pump}
    }

    fn render(&mut self, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.clear();

        let (width, height) = self.canvas.output_size()?;

        self.canvas.present();

        Ok(())
    }
}

impl GameUi for Sdl {
    fn input_coordinate(&mut self) -> Result<(usize, usize), ()> {
        let (x, y) = (0usize,0usize);
        let mut result :Result<(usize, usize), ()> = Ok((0usize,0usize));

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    result = Err(());
                },
                _ => {
                    // Extract x and y from UI
                    //(x, y) = todo!();
                    result = Ok((x, y));
                }
            }
        }
        result
    }

    fn output_game_finished(&self, evaluation: EvaluationResult, all_mines_detected: bool) -> bool {
        //todo!();
        false
    }

    fn print_area(&mut self, area: &GameArea) -> Result<(), String> {
        self.render(Color::RGB(0, 64, 255 - 0))
    }
}