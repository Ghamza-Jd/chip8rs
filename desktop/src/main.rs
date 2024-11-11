mod params;

use anyhow::bail;
use chip8_core::emu::Emu;
use chip8_core::specs::SPECS;
use clap::Parser;
use params::CliParams;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs::File;
use std::io::Read;

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

    let Ok(window) = video_subsystem
        .window("Chip-8 Emulator", width, height)
        .position_centered()
        .opengl()
        .build()
    else {
        bail!("Failed to build window");
    };

    let Ok(mut canvas) = window.into_canvas().present_vsync().build() else {
        bail!("Failed to build the renderer");
    };

    canvas.clear();
    canvas.present();

    let Ok(mut event_pump) = sdl_context.event_pump() else {
        bail!("Failed to obtain the SDL event pump");
    };

    let mut chip8 = Emu::new();

    let mut rom = File::open(params.rom)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    chip8.load(&buffer);

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, false);
                    }
                }
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

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
