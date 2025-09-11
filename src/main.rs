extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

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
    let mut squares: [[bool; M]; N] = [[false; M]; N];

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for i in 0..N {
        for j in 0..M {
            if squares[i][j] {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(((j*(SQUARE_SIZE as usize)) as i32),((i*(SQUARE_SIZE as usize)) as i32),SQUARE_SIZE as u32,SQUARE_SIZE as u32));
            }
        }
    }
    
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
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
