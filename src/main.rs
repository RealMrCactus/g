use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::controller::Button;

use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_gfx::primitives::DrawRenderer;


use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_gfx::primitives::DrawRenderer;

struct ColoredRect {
    rect: Rect,
    color: Color,
    stroke_color: Option<Color>,
    radius: Option<i16>,
}

impl ColoredRect {
    fn new(x: i32, y: i32, width: u32, height: u32, color: Color, stroke_color: Option<Color>, radius: Option<i16>) -> ColoredRect {
        ColoredRect {
            rect: Rect::new(x, y, width, height),
            color,
            stroke_color,
            radius,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let radius = self.radius.unwrap_or(0);
        let (r, g, b, a) = self.color.rgba();
        canvas.rounded_box(self.rect.x(), self.rect.y(), self.rect.width() as i16, self.rect.height() as i16, radius, (r, g, b, a)).unwrap();

        if let Some(stroke_color) = self.stroke_color {
            let (r, g, b, a) = stroke_color.rgba();
            canvas.rounded_rectangle(self.rect.x(), self.rect.y(), self.rect.width() as i16, self.rect.height() as i16, radius, (r, g, b, a)).unwrap();
        }
    }
}

struct Bt {
    rect: ColoredRect,
}

impl Bt {
    fn new(x: i32, y: i32, width: u32, height: u32, color: Color, stroke_color: Option<Color>, radius: Option<i16>) -> Bt {
        Bt {
            rect: ColoredRect::new(x, y, width, height, color, stroke_color, radius),
        }
    }

    fn is_clicked(&self, x: i32, y: i32) -> bool {
        self.rect.rect.contains_point(Point::new(x, y))
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.rect.draw(canvas);
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2 Red Square", 1000, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    

    let button1 = Bt::new(50, 50, 100, 100);
    let button2 = Bt::new(200, 200, 100, 100);
    let tbar = ColoredRect::new(0, 0, 1000, 20, Color::RGB(26, 26, 25), Some(Color::RGB(0, 0, 255)), Some(10));    



    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown { x, y, .. } => {
                    // Check if the click is within the button's rectangle
                    if button1.is_clicked(x, y) {
                        println!("Button 1 clicked!");
                    }
                    if button2.is_clicked(x, y) {
                        println!("Button 2 clicked!");
                    }
                },
                _ => {}
            }
        }
        canvas.clear();  
        
        tbar.draw(&mut canvas);
        //button1.draw(&mut canvas);
        //button2.draw(&mut canvas);

          
        canvas.set_draw_color(Color::RGB(10, 5, 5));
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
