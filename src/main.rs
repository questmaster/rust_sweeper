extern crate rand;

use std::io;
// "self" imports the "image" module itself as well as everything else we listed
use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use game_area::GameArea;

use crate::game_area::EvaluationResult;

mod game_area;
mod game_area_ui;

struct Percent {
    value: f32,
}

impl Percent {
    fn new(value: usize) -> Percent {
        if value > 100 {
            panic!("Percentage out of range!");
        }

        Percent {
            value: (value as f32 / 100.0),
        }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

fn fill_mines_in_area(area: &mut GameArea, pct: Percent) {
    let mut rng = rand::thread_rng();

    let mine_cnt = ((area.size_x() * area.size_y()) as f32 * pct.value()) as usize;

    for _i in 0..mine_cnt {
        let x = rng.gen_range(0..area.size_x());
        let y = rng.gen_range(0..area.size_y());

        area.set_mine(x, y);
    }
}

fn input_coordinate() -> (usize, usize) {
    let mut x = String::new();
    let mut y = String::new();

    println!("Enter x coordinate:");
    io::stdin().read_line(&mut x).expect("Input failed.");
    println!("Enter y coordinate:");
    io::stdin().read_line(&mut y).expect("Input failed.");

    let x: usize = x.trim().parse().expect("x not a number.");
    let y: usize = y.trim().parse().expect("y not a number.");

    (x, y)
}

fn render(canvas: &mut WindowCanvas, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    println!("Prepairing game area...");
    let mut area = GameArea::new();
    fill_mines_in_area(&mut area, Percent::new(10));

    println!("Let's start!");
    //game_area_ui::print_area(&area);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust Sweeper - SDL Edition", 600, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        let (x, y) = input_coordinate(); // Terminal

        // Update
        i = (i + 1) % 255;
        let evaluation = area.evaluate_square(x, y); // Terminal
        match evaluation {
            EvaluationResult::Mine => {
                println!("BOOMM!! You lost!");
                break;
            }
            EvaluationResult::Nothing => {
                if area.all_mines_detected() {
                    println!("==> You  WON !!! <==");
                    break;
                }
            }
        } // Terminal


        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i))?;
        game_area_ui::print_area(&area); // Terminal

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
