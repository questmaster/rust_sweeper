use super::GameUi;
use crate::game_area::{EvaluationResult, GameArea};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use std::collections::HashMap;

pub struct TextureManager<'a> {
    tc: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'a>>,
}

impl<'s> TextureManager<'s> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> TextureManager {
        return TextureManager {
            tc: &texture_creator,
            textures: HashMap::new(),
        };
    }

    pub fn load(&mut self, resource_id: &str, surface: Surface) -> &Texture {
        {
            if self.textures.contains_key(resource_id) {
                return self.textures.get(resource_id).unwrap();
            }
        }

        let new_texture = self
            .tc
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())
            .unwrap();

        self.textures
            .insert(resource_id.clone().to_string(), new_texture);

        return self.textures.get(resource_id).unwrap().clone();
    }

    fn get(&self, resource_id: &str) -> &Texture {
        self.textures.get(resource_id).unwrap().clone()
    }

    fn unload_all(&mut self) {
        self.textures.clear();
    }
}

pub struct Sdl {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    SQUARE_SIZE: u32,
}

impl Sdl {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Rust Sweeper - SDL Edition", 600, 800)
            .position_centered()
            .opengl()
            .build()
            .expect("could not initialize video subsystem");
        let mut canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        Sdl {
            canvas,
            event_pump,
            ttf_context,
            SQUARE_SIZE: 50,
        }
    }

    fn render(&mut self, game_area: &GameArea) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        //let (width, height) = self.canvas.output_size()?;

        // -- init --------------------------------
        // Load a font
        let mut font = self
            .ttf_context
            .load_font("assets/ostrich-sans-rounded.ttf", 140)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);

        // render a surface, and convert it to a texture bound to the canvas
        for n in 1..9 {
            let number = &n.to_string();
            let left = &format!("{}{}", "left", number);
            let line = &format!(" {} |", number);

            Sdl::generate_and_store_texture(&mut font, &mut texture_manager, number, number);

            Sdl::generate_and_store_texture(&mut font, &mut texture_manager, left, line);
        }
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "*", "*");
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "M", "M");
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "-", "-");
        Sdl::generate_and_store_texture(
            &mut font,
            &mut texture_manager,
            "head1",
            "   X 1 2 3 4 5 6 7 8 9",
        );
        Sdl::generate_and_store_texture(
            &mut font,
            &mut texture_manager,
            "head2",
            " Y +------------------",
        );
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "leftE", "   |");
        // ------------------------------------------------------------

        Sdl::write_coordinatesystem(&mut self.canvas, &texture_manager);
        self.canvas.copy(
            &texture_manager.get("5"),
            None,
            Rect::new(100, 100, self.SQUARE_SIZE / 2, self.SQUARE_SIZE),
        )?;
        self.canvas
            .draw_line(Point::new(100, 100), Point::new(200, 200));

        self.canvas.present();

        Ok(())
    }

    fn write_coordinatesystem(canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        Sdl::write_texture(canvas, texture_manager.get("head1"), 0, 0, 550, 50);
        Sdl::write_texture(canvas, texture_manager.get("head2"), 0, 50, 550, 50);
        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 100, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("left1"), 0, 150, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 200, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("left2"), 0, 250, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 300, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("left3"), 0, 350, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 400, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("left4"), 0, 450, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 500, 100, 50);
        Sdl::write_texture(canvas, texture_manager.get("left5"), 0, 550, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 600, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("left6"), 0, 650, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 700, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("left7"), 0, 750, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 800, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("left8"), 0, 850, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("leftE"), 0, 900, 100, 50);
        //        Sdl::write_texture(canvas, texture_manager.get("left9"), 0, 950, 100, 50);
    }

    fn write_texture(
        canvas: &mut Canvas<Window>,
        texture: &Texture,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) {
        canvas
            .copy(&texture, None, Rect::new(x, y, width, height))
            .unwrap();
    }

    fn generate_and_store_texture(
        font: &mut Font,
        texture_manager: &mut TextureManager,
        id: &str,
        val: &str,
    ) {
        let surface = font
            .render(val)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())
            .unwrap();
        texture_manager.load(id, surface);
    }
}

impl GameUi for Sdl {
    fn input_coordinate(&mut self) -> Result<(usize, usize), ()> {
        let (x, y) = (0usize, 0usize);
        let mut result: Result<(usize, usize), ()> = Ok((0usize, 0usize));

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    result = Err(());
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    //todo!("This code is not adjusted to the game");
                    let x = (x as u32) / self.SQUARE_SIZE;
                    let y = (y as u32) / self.SQUARE_SIZE;
                    /*                    match game.get_mut(x as i32, y as i32) {
                                          Some(square) => {
                                              *square = !(*square);
                                          }
                                          None => unreachable!(),
                                      };
                    */
                }
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
        self.render(area)
    }
}
