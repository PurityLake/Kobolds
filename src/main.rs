extern crate sdl2;

mod tile;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut font = ttf.load_font("font.ttf", 32).unwrap();

    let surface = font.render("Hello World!")
        .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    canvas.copy(&texture, None, Some(Rect::new(100, 100, width, height)));
    canvas.present();
    
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    }
}