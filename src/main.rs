extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

mod gol;
use gol::grid::Grid;

const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const SQUARE_FACTOR: u8 = 10;
const SQUARE_SIZE: u8 = 2 * SQUARE_FACTOR;
const N: usize = (WINDOW_HEIGHT / (SQUARE_SIZE as u32)) as usize;
const M: usize = (WINDOW_WIDTH / (SQUARE_SIZE as u32)) as usize;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::<N, M>::new();

    grid.set(2, 3, true);
    grid.set(2, 4, true);
    grid.set(2, 5, true);
    grid.set(3, 4, true);

    canvas.present();

    'running: loop {
        // Main event loop to intervene later on
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
