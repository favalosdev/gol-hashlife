#[macro_use(c)]
extern crate cute;
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
const SQUARE_SIZE: usize = (2 * SQUARE_FACTOR) as usize;
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

    for y in 0..N {
        for x in 0..M {
            if grid.retrieve(x, y) {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            let a: i32 = (x * SQUARE_SIZE) as i32;
            let b: i32 = (y * SQUARE_SIZE) as i32;
            canvas.fill_rect(Rect::new(a, b, SQUARE_SIZE as u32,SQUARE_SIZE as u32));
        }
    }
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    grid.evolve();
                    canvas.clear();
                    for y in 0..N {
                        for x in 0..M {
                            if grid.retrieve(x, y) {
                                canvas.set_draw_color(Color::RGB(255, 255, 255));
                            } else {
                                canvas.set_draw_color(Color::RGB(0, 0, 0));
                            }
                            let a: i32 = (x * SQUARE_SIZE) as i32;
                            let b: i32 = (y * SQUARE_SIZE) as i32;
                            canvas.fill_rect(Rect::new(a, b, SQUARE_SIZE as u32,SQUARE_SIZE as u32));
                        }
                    }
                    canvas.present();
                }
                _ => {}
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
