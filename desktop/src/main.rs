mod params;

use anyhow::bail;
use chip8_core::emu::Emu;
use chip8_core::specs::SPECS;
use clap::Parser;
use params::CliParams;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

const TICKS_PER_FRAME: usize = 10;

fn main() -> anyhow::Result<()> {
    let params = CliParams::try_parse()?;
    let Ok(sdl_context) = sdl2::init() else {
        bail!("Failed to intialize sdl2");
    };

    let Ok(video_subsystem) = sdl_context.video() else {
        bail!("Failed to initialize video subsystem");
    };

    let width = (SPECS.screen_w as u32) * params.scale;
    let height = (SPECS.screen_h as u32) * params.scale;

    let window = video_subsystem
        .window("Chip-8 Emulator", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Emu::new();

    let mut rom = File::open(params.rom)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    chip8.load(&buffer);

    let k = Arc::new(Mutex::new(String::from("Hello")));
    let guard = k.lock().unwrap();
    guard.borrow();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => (),
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timer();
        draw_screen(&chip8, &mut canvas, SPECS.screen_w, params.scale);
    }
    Ok(())
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>, width: usize, scale: u32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Iterate over the 1D screen buffer, if we find a white pixel, then it's worth getting its 2D coordinates and
    // and draw the pixel there
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % width) as u32;
            let y = (i / width) as u32;
            let rect = Rect::new((x * scale) as i32, (y * scale) as i32, scale, scale);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
