extern crate cute;
extern crate sdl2;
extern crate memoize;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode,Scancode};
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use std::time::{Duration,Instant};
use std::fs::File;
use std::path::Path;

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
const CAMERA_DELTA: i32 = 2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File-path of the pattern (in .rle format) to load
    #[arg(short = 'p', long)]
    pattern_path: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Info {
    mouse_x: i32,
    mouse_y: i32,
    generation: i32,
}

// Stolen macros to handle annoying Rects
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn render_info(canvas: &mut Canvas<Window>, text: &str, padding: u32) {
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    // Load a font
    let font = ttf_context.load_font(Path::new("assets/IBM_Plex_Mono/IBMPlexMono-Regular.ttf"), 20).unwrap();
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width: t_width, height: t_height, .. } = texture.query();

    let target = rect!(WINDOW_WIDTH - t_width - padding, WINDOW_HEIGHT - t_height - padding, t_width, t_height); 

    canvas.copy(&texture, None, Some(target)).unwrap();
    canvas.present();
}

fn main() {
    // SDL-2 stuff 
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    // Game of life specific stuff
    let args = Args::parse();
    let mut grid = Grid::new();
    let mut camera= Camera::new(ZOOM, 0, 0);

    let file;
 
    match args.pattern_path {
        Some(path) => {
            file = File::open(path).unwrap();
            
        },
        None => {
            file = File::open("assets/patterns/hwss.rle").unwrap();
        }
    }

    let mut info = Info {
        mouse_x: 0,
        mouse_y: 0,
        generation: 0
    };

    let mut prev_info =  info;

    grid.load_pattern(Rle::new_from_file(file).unwrap());
    
    let draw_squares = |canvas: &mut Canvas<Window>, grid: &Grid, camera: &Camera, info: &mut Info, prev_info: &mut Info| {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (x,y) in grid.cells.iter() {
            let (xo_w, yo_w) = (*x,-*y);
            let (xf_w, yf_w) = (xo_w + 1, yo_w + 1);

            let (xo_s, yo_s) = camera.from_world_coords(xo_w, yo_w);
            let (xf_s, _) = camera.from_world_coords(xf_w, 0);
            let (_, yf_s) = camera.from_world_coords(0, yf_w);

            let to_draw = rect!(xo_s + OFFSET_X, yo_s + OFFSET_Y, xf_s - xo_s, yf_s - yo_s);
            let _ = canvas.fill_rect(to_draw);
        }

        canvas.present();
        *prev_info = *info;
        *info = Info {
            mouse_x: prev_info.mouse_x,
            mouse_y: prev_info.mouse_y,
            generation: prev_info.generation + 1
        }
    };

    let format_info = |info: &Info| {
        let mx = info.mouse_x;
        let my = info.mouse_y;
        let g = info.generation;
        return format!("x: {mx}, y: {my}, gen: {g}");
    };

    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / GAME_FREQ);
    let mut is_paused = false;

    // Initial render
    draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
    render_info(&mut canvas, &format_info(&info), 10);

    'running: loop {
        let  now = Instant::now();
        if  now.duration_since(last_game_tick) >= game_interval {
            if !is_paused {
                last_game_tick = now;
                draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                grid.evolve();
            }

            if info != prev_info {
                render_info(&mut canvas, &format_info(&info), 10);
                prev_info = info;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                    camera.y -= CAMERA_DELTA;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    camera.x -= CAMERA_DELTA;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    camera.y += CAMERA_DELTA;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                    camera.x += CAMERA_DELTA;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                // Zoom in
                Event::KeyDown { scancode: Some(Scancode::I), .. } => {
                    camera.zoom += 1;

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                // Zoom out
                Event::KeyDown { scancode: Some(Scancode::O), .. } => {
                    if camera.zoom > 1 {
                        camera.zoom -= 1;
                    }

                    if is_paused {
                        draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::P), .. } => {
                    is_paused = true;
                    draw_squares(&mut canvas, &grid, &camera, &mut info, &mut prev_info);
                },
                Event::KeyDown { scancode: Some(Scancode::R), .. } => {
                    is_paused = false;
                },
                Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                    if is_paused {
                        grid.evolve();
                        draw_squares(&mut canvas, &mut grid, &camera, &mut info, &mut prev_info);
                    }
                },
                _ => {}
            }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }
}
