use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use super::grid::Grid;
use super::constants;

// It doesn't matter if it gets really really small as long as it works
pub fn get_square_size(height: u32, width: u32, N: usize, M: usize) {
}

pub fn draw_squares(canvas: &mut Canvas<Window>, grid: &mut Grid) {
    for y in 0..(grid.rows) {
        for x in 0..(grid.cols) {
            if grid.cells[y][x] {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            let a: i32 = (x * constants::SQUARE_SIZE) as i32;
            let b: i32 = (y * constants::SQUARE_SIZE) as i32;
            let _ = canvas.fill_rect(Rect::new(a, b, constants::SQUARE_SIZE as u32,constants::SQUARE_SIZE as u32));
        }
    }
}
