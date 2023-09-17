mod life;

use life::{Cell, World};
use macroquad::{miniquad::conf::Platform, prelude::*};
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Default)]
struct Settings {
    paused: bool,
    click_mode: Cell,
    step: bool,
}

// shared by main loop and javascript iterop for user input.
// todo: switch to parking_lot?
static SETTINGS: Lazy<RwLock<Settings>> = Lazy::new(|| RwLock::new(Settings::default()));

// Anything marked with #[no_mangle] and pub extern "C"
// will be available in JS via wasm_exports
#[no_mangle]
pub extern "C" fn toggle_paused() {
    let mut settings = SETTINGS.write().unwrap();
    settings.paused = !settings.paused;
}

#[no_mangle]
pub extern "C" fn toggle_click_mode() {
    let mut settings = SETTINGS.write().unwrap();
    settings.click_mode = !settings.click_mode;
}

#[no_mangle]
pub extern "C" fn step() {
    let mut settings = SETTINGS.write().unwrap();
    settings.step = true;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    let mut frame: u64 = 0;

    loop {
        clear_background(BLACK);
        let click_mode = { SETTINGS.read().unwrap().click_mode };
        draw_ui(click_mode, frame);

        let screen_size = (screen_width() + screen_height()) / 2.0;
        let cell_size_ratio = (world.width as f32 + world.height as f32) / 2.0;
        let cell_size = screen_size / cell_size_ratio;

        for row in 0..world.height {
            for col in 0..world.width {
                let index = world.get_cell_index(row, col);
                if world.cells[index] == Cell::Alive {
                    let (x, y) = world_to_screen_pos(&world, row, col);
                    draw_rectangle(x, y, cell_size, cell_size, BLUE);
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            // don't ask me why these are flipped, mouse gives (y, x)
            let (y, x) = mouse_position();
            let (world_row, world_col) = screen_to_world_pos(&world, (x, y));
            let cell_index = world.get_cell_index(world_row, world_col);
            world.cells[cell_index] = click_mode;
        }

        let (paused, step) = {
            let settings = SETTINGS.read().unwrap();
            (settings.paused, settings.step)
        };

        if !paused || step {
            world.tick();
            frame += 1;
        }

        if step {
            // step is implemented via a bool in settings,
            // if it is set we will advance the frame in the block
            //  above and immediately set it to false here
            let mut settings = SETTINGS.write().unwrap();
            settings.step = false;
        }

        next_frame().await
    }
}

fn screen_to_world_pos(world: &World, pos: (f32, f32)) -> (usize, usize) {
    let width_ratio = pos.0 / screen_width();
    let height_ratio = pos.1 / screen_height();

    let width = world.width as f32;
    let height = world.height as f32;

    let world_col = width_ratio * width;
    let world_row = height_ratio * height;

    (world_col as usize, world_row as usize)
}

fn world_to_screen_pos(world: &World, row: usize, col: usize) -> (f32, f32) {
    let x_ratio = col as f32 / world.width as f32;
    let y_ratio = row as f32 / world.height as f32;

    let x = x_ratio * screen_width();
    let y = y_ratio * screen_height();

    (x, y)
}

fn draw_ui(mode: Cell, frame: u64) {
    let font_size = 45.0;
    draw_text(
        &format!("click mode: {mode:?}"),
        10.0,
        30.0,
        font_size,
        GREEN,
    );

    draw_text(&format!("frame: {frame}"), 10.0, 75.0, font_size, GREEN);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "GameOfLife".to_string(),
        window_width: 1500,
        window_height: 1500,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Platform::default(),
    }
}
