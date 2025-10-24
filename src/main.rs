extern crate cute;
extern crate sdl2;
extern crate memoize;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode,Scancode};
use std::time::{Duration,Instant};

mod gol;
use gol::grid::Grid;
use gol::utils::draw_squares;
use gol::constants;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of life", constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::new(constants::N, constants::M);

    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / constants::GAME_FREQ);

    'running: loop {
        let  now = Instant::now();

        if now.duration_since(last_game_tick) >= game_interval {
                last_game_tick = now;
                canvas.clear();
                draw_squares(&mut canvas, &mut grid);
                canvas.present();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    grid.shrink(1, 1);
                    println!("Zooming in! The grid's size is now: ({},{})", grid.rows, grid.cols);
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    grid.grow(1, 1);
                    println!("Zooming out! The grid's size is now: ({},{})", grid.rows, grid.cols);
                },
                _ => {}
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
