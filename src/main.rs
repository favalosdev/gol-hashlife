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
use sdl2::mouse::MouseState;

use std::cmp;
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
const GRID_COLOR: Color = Color::RGB(255, 255, 255);
const CELL_COLOR: Color = Color::RGB(255, 255, 255);
const FEEDBACK_COLOR: Color = Color::RGB(255, 255, 255);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File-path of the pattern (in .rle format) to load
    #[arg(short = 'p', long)]
    pattern_path: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
struct MouseCoords {
    x: isize,
    y: isize
}

#[derive(Clone, PartialEq, Eq)]
struct Feedback {
    mouse_coords: MouseCoords,
    generation: i32
}

// Stolen macros to handle annoying Rects
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_rect(camera: &Camera, x_raw: isize, y_raw: isize) -> Rect {
    let (xo_w, yo_w) = (x_raw, -y_raw);
    let (xf_w, yf_w) = (xo_w + 1, yo_w + 1);

    let (xo_s, yo_s) = camera.from_world_coords(xo_w, yo_w);
    let (xf_s, _) = camera.from_world_coords(xf_w, 0);
    let (_, yf_s) = camera.from_world_coords(0, yf_w);

    rect!(xo_s + OFFSET_X, yo_s + OFFSET_Y, xf_s - xo_s, yf_s - yo_s)
}

fn draw_squares(canvas: &mut Canvas<Window>, grid: &Grid, camera: &Camera, show_grid: bool) {
    canvas.set_draw_color(CELL_COLOR);

    let mut min_x_s = WINDOW_WIDTH as i32;
    let mut min_y_s = WINDOW_HEIGHT as i32;

    for (x,y) in grid.cells.iter() {
        let to_fill = get_rect(camera, *x, *y);
        let _ = canvas.fill_rect(to_fill);
        
        if to_fill.x >= 0 && to_fill.y >= 0 {
            min_x_s = cmp::min(min_x_s, to_fill.x);
            min_y_s = cmp::min(min_y_s, to_fill.y);
        }
    }

    if show_grid {
        draw_grid(canvas, camera, min_x_s, min_y_s)
    } 
}

fn draw_grid(canvas: &mut Canvas<Window>, camera: &Camera, min_x_s: i32, min_y_s: i32) {
    canvas.set_draw_color(GRID_COLOR);

    let dummy = get_rect(camera, 0, 0);
    let width = dummy.width() as i32;
    let height = dummy.height() as i32;

    let start_x = min_x_s % width;
    let start_y = min_y_s % height;

    let mut x = start_x;

    while x <= WINDOW_WIDTH as i32 {
        let _ = canvas.draw_line((x, 0), (x, WINDOW_HEIGHT as i32));
        x += width;
    }

    // Draw Horizontal Lines
    let mut y = start_y;

    while y <= WINDOW_HEIGHT as i32 {
        let _ = canvas.draw_line((0, y), (WINDOW_WIDTH as i32, y));
        y += height;
    } 
}

fn draw_feedback(canvas: &mut Canvas<Window>, feedback: &Feedback) {
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let padding = 10;

    // Load a font
    let mut font = ttf_context.load_font(Path::new("assets/IBM_Plex_Mono/IBMPlexMono-Regular.ttf"), 20).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mx = feedback.mouse_coords.x;
    let my = feedback.mouse_coords.y;
    let generation = feedback.generation;
    let text = format!("X: {mx}, Y: {my}, gen: {generation}");

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(&text)
        .blended(FEEDBACK_COLOR)
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width: t_width, height: t_height, .. } = texture.query();

    let target = rect!(WINDOW_WIDTH - t_width - padding, WINDOW_HEIGHT - t_height - padding, t_width, t_height); 

    canvas.copy(&texture, None, Some(target)).unwrap();
}

fn draw_all(canvas: &mut Canvas<Window>, grid: &Grid, camera: &Camera, feedback: &Feedback, show_grid: bool) {
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    draw_squares(canvas, grid, camera, show_grid);
    draw_feedback(canvas, feedback);
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
    
    let mut feedback = Feedback {
        mouse_coords: MouseCoords {
            x: 0,
            y: 0
        },
        generation: 0 
    };

    grid.load_pattern(Rle::new_from_file(file).unwrap());

    let mut last_game_tick = Instant::now();
    let game_interval = Duration::from_nanos(1_000_000_000 / GAME_FREQ);
    let mut is_paused = false;
    let mut show_grid = false;

    // Initial render
    draw_all(&mut canvas, &grid, &camera, &feedback, show_grid);

    'running: loop {
        let  now = Instant::now();
        if  now.duration_since(last_game_tick) >= game_interval {
            last_game_tick = now;
            draw_all(&mut canvas, &grid, &camera, &feedback, show_grid);
            if !is_paused {
                grid.evolve();
                feedback.generation += 1;
            }
        }

        let mouse_state: MouseState = event_pump.mouse_state();
        let (x_w, y_w) = camera.from_screen_coords(mouse_state.x() - OFFSET_X, mouse_state.y() - OFFSET_Y);
        feedback.mouse_coords = MouseCoords { x: x_w, y: -y_w };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                    camera.y -= CAMERA_DELTA;
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    camera.x -= CAMERA_DELTA;
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    camera.y += CAMERA_DELTA;
                },
                Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                    camera.x += CAMERA_DELTA;
                },
                // Zoom in
                Event::KeyDown { scancode: Some(Scancode::I), .. } => {
                    camera.zoom += 1;
                },
                // Zoom out
                Event::KeyDown { scancode: Some(Scancode::O), .. } => {
                    if camera.zoom > 1 {
                        camera.zoom -= 1;
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::P), .. } => {
                    is_paused = !is_paused;
                },
                Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                    if is_paused {
                        grid.evolve();
                        feedback.generation += 1;
                    }
                },
                Event::KeyDown { scancode: Some(Scancode::G), .. } => {
                    show_grid = !show_grid;
                },
                _ => {}
            }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }
}
