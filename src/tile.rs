use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Tile {
    width: u32,
    height: u32,
    symbol: char,
    scolor: Color,
    bgcolor: Color
}

impl Tile {
    fn new(width: u32, height: u32, symbol: char) -> Tile {
        Tile { 
            width: width,
            height: height,
            symbol: symbol,
            scolor: Color::RGB(255, 0, 0),
            bgcolor: Color::RGB(0, 255, 0) }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.scolor);
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height));
    }
}