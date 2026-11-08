use std::io::{stdout, Write};
use std::time::Instant;
use crate::shaders::{Shader, ShaderParams};

pub(crate) fn show(shader: impl Shader, params: ShaderParams) {
    let (width, height) = (60, 30);

    print!("\x1b[?25l\x1b[2J");

    print_shader(shader, params, (width, height));
}


fn print_shader(shader: impl Shader, params: ShaderParams, (width, height): (u8, u8)) {
    let mut stdout = stdout();
    let start = Instant::now();

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
