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
use std::fs::File;

use clap::Parser;

use ca_formats::rle::Rle;

mod gol;
use gol::grid::Grid;
use gol::camera::Camera;

const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const GAME_FREQ: u64 = 20;
const FPS: u32 = 200;
const ZOOM: i32 = 20;
const OFFSET_X: i32 = (WINDOW_WIDTH / 2) as i32;
const OFFSET_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File-path of the pattern to load
    #[arg(short = 'p', long)]
    pattern_path: Option<String>,
}

fn main() {
    // SDL-2 stuff 
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    // Game of life specific stuff
    let args = Args::parse();
    let mut grid = Grid::new();
    let mut camera= Camera::new(ZOOM, 0, 0);

    match args.pattern_path {
        Some(path) => {
            let file = File::open(path).unwrap();
            grid.load_pattern(Rle::new_from_file(file).unwrap());
        },
        _ => {}
    } 
    

    let draw_squares = |canvas: &mut Canvas<Window>, grid: &Grid, camera: &Camera| {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (x,y) in grid.cells.iter() {
            let (xo_w, yo_w) = (*x,-*y);
            let (xf_w, yf_w) = (xo_w + 1, yo_w + 1);

            let (xo_s, yo_s) = camera.from_world_coords(xo_w, yo_w);
            let (xf_s, _) = camera.from_world_coords(xf_w, 0);
            let (_, yf_s) = camera.from_world_coords(0, yf_w);

            let to_draw = Rect::new(xo_s + OFFSET_X, yo_s + OFFSET_Y, (xf_s - xo_s) as u32, (yf_s - yo_s) as u32);
            let _ = canvas.fill_rect(to_draw);
        }

        canvas.present();
    };

    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / GAME_FREQ);
    let mut is_paused = false;

    // Initial render
    draw_squares(&mut canvas, &grid, &camera);

    'running: loop {
        if !is_paused {
            let  now = Instant::now();
            if  now.duration_since(last_game_tick) >= game_interval {
                last_game_tick = now;
                draw_squares(&mut canvas, &grid, &camera);
                grid.evolve();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                    camera.y -= 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    camera.x -= 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    camera.y += 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                    camera.x += 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                // Zoom in
                Event::KeyDown { scancode: Some(Scancode::I), .. } => {
                    camera.zoom += 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                // Zoom out
                Event::KeyDown { scancode: Some(Scancode::O), .. } => {
                    if camera.zoom > 1 {
                        camera.zoom -= 1;
                    }

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::P), .. } => {
                    is_paused = true;
                    draw_squares(&mut canvas, &grid, &camera);
                },
                Event::KeyDown { scancode: Some(Scancode::R), .. } => {
                    is_paused = false;
                },
                Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                    if is_paused {
                        grid.evolve();
                        draw_squares(&mut canvas, &mut grid, &camera);
                    }
                },
                _ => {}
            }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }
}
