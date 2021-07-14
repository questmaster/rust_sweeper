use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

use crate::game_area::square::Square;
use crate::game_area::{EvaluationResult, GameArea};

use super::GameUi;

pub struct TextureManager<'a> {
    tc: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'a>>,
}

impl<'s> TextureManager<'s> {
    pub fn new(texture_creator: &'s TextureCreator<WindowContext>) -> Self {
        Self {
            tc: &texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn from_surface(&mut self, resource_id: &str, surface: Surface) -> &Texture {
        if self.textures.contains_key(resource_id) {
            return self.textures.get(resource_id).unwrap();
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
    square_size: u32,
}

impl Sdl {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Rust Sweeper - SDL Edition", 650, 650)
            .position_centered()
            .opengl()
            .build()
            .expect("could not initialize video subsystem");
        let canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas,
            event_pump,
            ttf_context,
            square_size: 50,
        }
    }

    fn render(
        &mut self,
        game_area: &GameArea,
        evaluation: &EvaluationResult,
    ) -> Result<(), String> {
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
        for n in 0..=9 {
            let number = &n.to_string();
            let left = &format!("{}{}", "left", number);
            let line = &format!(" {}", number);

            Sdl::generate_and_store_texture(&mut font, &mut texture_manager, number, number);

            Sdl::generate_and_store_texture(&mut font, &mut texture_manager, left, line);
        }
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "*", "*");
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "M", "M");
        Sdl::generate_and_store_texture(
            &mut font,
            &mut texture_manager,
            "head1",
            "   X 0 1 2 3 4 5 6 7 8 9",
        );
        Sdl::generate_and_store_texture(&mut font, &mut texture_manager, "head2", " Y");
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        Sdl::generate_and_store_texture(
            &mut font,
            &mut texture_manager,
            "win",
            "==> You WON !!! <==",
        );
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        Sdl::generate_and_store_texture(
            &mut font,
            &mut texture_manager,
            "loose",
            "BOOMM!! You lost!",
        );

        // ------------------------------------------------------------

        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        Sdl::write_coordinatesystem(&mut self.canvas, &texture_manager);

        for elem in 0..game_area.size_y() {
            for line in 0..game_area.size_x() {
                Sdl::write_square_value(
                    &mut self.canvas,
                    &texture_manager,
                    line as i32,
                    elem as i32,
                    &game_area.area()[line][elem],
                );
            }
        }

        match evaluation {
            EvaluationResult::Mine => {
                Sdl::write_texture(
                    &mut self.canvas,
                    texture_manager.get("loose"),
                    150,
                    50,
                    425,
                    50,
                );
            }
            EvaluationResult::Won => {
                Sdl::write_texture(
                    &mut self.canvas,
                    texture_manager.get("win"),
                    100,
                    50,
                    475,
                    50,
                );
            }
            _ => {
                // nothing to do
            }
        }

        self.canvas.present();

        Ok(())
    }

    fn write_square_value(
        canvas: &mut Canvas<Window>,
        texture_manager: &TextureManager,
        x: i32,
        y: i32,
        square: &Square,
    ) {
        if square.visible == false {
            Sdl::write_texture(
                canvas,
                texture_manager.get(&"*"),
                (x * 50) + 118,
                (y * 50) + 104,
                25,
                50,
            );
        } else if square.mine == true {
            Sdl::write_texture(
                canvas,
                texture_manager.get(&"M"),
                (x * 50) + 118,
                (y * 50) + 104,
                25,
                50,
            );
        } else if square.value == 0 {
        } else {
            Sdl::write_texture(
                canvas,
                texture_manager.get(&square.value.to_string()),
                (x * 50) + 118,
                (y * 50) + 104,
                25,
                50,
            );
        }
    }

    fn write_coordinatesystem(canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        Sdl::write_texture(canvas, texture_manager.get("head1"), 0, 0, 600, 50);
        Sdl::write_texture(canvas, texture_manager.get("head2"), 0, 50, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left0"), 0, 100, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left1"), 0, 150, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left2"), 0, 200, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left3"), 0, 250, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left4"), 0, 300, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left5"), 0, 350, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left6"), 0, 400, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left7"), 0, 450, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left8"), 0, 500, 50, 50);
        Sdl::write_texture(canvas, texture_manager.get("left9"), 0, 550, 50, 50);
        for row in 0..=9 {
            for column in 0..=9 {
                let rect = [Rect::new((column * 50) + 104, (row * 50) + 100, 50, 50)];
                canvas.draw_rects(&rect).unwrap();
            }
        }
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
        texture_manager.from_surface(id, surface);
    }
}

impl GameUi for Sdl {
    fn input_coordinate(&mut self) -> Result<(usize, usize), bool> {
        let mut result: Result<(usize, usize), bool> = Err(false);

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    result = Err(true);
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let x = ((x as usize) - 100) / self.square_size as usize;
                    let y = ((y as usize) - 100) / self.square_size as usize;
                    result = Ok((x, y));
                }
                _ => {
                    result = Err(false);
                }
            }
        }
        result
    }

    fn print_area(&mut self, area: &GameArea, evaluation: &EvaluationResult) -> Result<(), String> {
        self.render(area, evaluation)
    }
}
