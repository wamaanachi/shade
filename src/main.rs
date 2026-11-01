mod app;
mod shaders;
mod ui;

use std::{
    io::{Write, stdout},
    time::Instant,
};

use crate::shaders::{Shader, ShaderParams, plasma::NeonPlasma};
use crate::shaders::mountains::Mountains;
pub mod config;
pub mod cli;

fn main() -> std::io::Result<()> {
    cli::parse::parse();
  
    let demo = std::env::args().any(|argument| argument == "--demo");

    if demo {
        run_demo();
    } else {
        app::run()?;
    }

    Ok(())
}

fn run_demo() {
    let shader = Mountains;
    let params = ShaderParams::default();
    let start = Instant::now();
    let mut stdout = stdout();

    let (width, height) = (60, 30);

    print!("\x1b[?25l\x1b[2J");

    loop {
        let time = start.elapsed().as_secs_f32();

        print!("\x1b[H");

        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                let u =
                    ((x as f32 / width as f32) * 2.0 - 1.0) * (width as f32 / height as f32) * 0.5;
                let v = -((y as f32 / height as f32) * 2.0 - 1.0);

                let color = shader.evaluate((u, v), time, &params);

                line.push_str(&format!("\x1b[48;2;{};{};{}m ", color.r, color.g, color.b));
            }

            line.push_str("\x1b[0m\n");

            print!("{line}");
        }

        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(33)); // loop only runs every 30 times per second
    }
}
