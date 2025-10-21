extern crate cute;
extern crate sdl2;
extern crate memoize;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::{Duration,Instant};

mod gol;
use gol::grid::Grid;

const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const SQUARE_FACTOR: u8 = 10;
const SQUARE_SIZE: usize = (2 * SQUARE_FACTOR) as usize;
const N: usize = (WINDOW_HEIGHT / (SQUARE_SIZE as u32)) as usize;
const M: usize = (WINDOW_WIDTH / (SQUARE_SIZE as u32)) as usize;
const GAME_FREQ: u64 = 20;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::<N, M>::new();

    let draw_squares = |canvas: &mut Canvas<Window>, grid: &mut Grid::<N,M>| {
        for y in 0..N {
            for x in 0..M {
                if grid.retrieve(x, y) {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                let a: i32 = (x * SQUARE_SIZE) as i32;
                let b: i32 = (y * SQUARE_SIZE) as i32;
                let _ = canvas.fill_rect(Rect::new(a, b, SQUARE_SIZE as u32,SQUARE_SIZE as u32));
            }
        }
    };

    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / GAME_FREQ);

    'running: loop {
        let  now = Instant::now();

        if now.duration_since(last_game_tick) >= game_interval {
                last_game_tick = now;
                canvas.clear();
                draw_squares(&mut canvas, &mut grid);
                canvas.present();
                grid.evolve();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
