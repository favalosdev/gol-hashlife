extern crate cute;
extern crate sdl2;
extern crate memoize;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode,Scancode};
use sdl2::rect::Rect;
use std::time::{Duration,Instant};

mod gol;
use gol::grid::Grid;
use gol::screen::ScreenState;

const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const SQUARE_FACTOR: u8 = 10;
const SQUARE_SIZE: usize = (2 * SQUARE_FACTOR) as usize;
const R: usize = (WINDOW_HEIGHT / (SQUARE_SIZE as u32)) as usize;
const C: usize = (WINDOW_WIDTH / (SQUARE_SIZE as u32)) as usize;
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
    let mut grid = Grid::<R, C>::new();

    let mut screen = ScreenState::new(1, 0, 0);

    /*
     The way I imagine it is as follows:
     (1) We render every piece of the world that's currently in the viewport.
     * Per each object, we determine whether at least one vertex is included in the screen.
     * (We would need to perform world_to_screen conversion)
     * If at least one vertex is included, render the whole object.
     */

    let draw_squares = |canvas: &mut Canvas<Window>, grid: &Grid::<R,C>, screen: &ScreenState| {
        canvas.set_draw_color(Color::RGB(255,255, 255));
        for (x_w,y_w) in grid.cells.iter() {
            let (x_s, y_s)= screen.world_to_screen(*x_w, *y_w, SQUARE_SIZE as u32);
            let _ = canvas.fill_rect(Rect::new(x_s, y_s,SQUARE_SIZE as u32,SQUARE_SIZE as u32));
        }
    };

    /*
    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / GAME_FREQ);
    */
    draw_squares(&mut canvas, &mut grid, &screen);
    canvas.present();
    grid.evolve();
    'running: loop {
        /*
        let  now = Instant::now();

        if now.duration_since(last_game_tick) >= game_interval {
                last_game_tick = now;
                canvas.clear();
                draw_squares(&mut canvas, &mut grid, &screen);
                canvas.present();
                grid.evolve();
        }
        */

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    println!("Zooming in!");
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    println!("Zooming out!");
                },
                Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                    grid.evolve();
                    canvas.clear();
                    draw_squares(&mut canvas, &mut grid, &screen);
                    canvas.present();
                },
                _ => {}
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
