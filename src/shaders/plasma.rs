use core::f32;

use crate::shaders::{Color, Shader, ShaderParams};

#[derive(Debug)]
pub struct NeonPlasma;

fn palette(t: f32) -> [f32; 3] {
    let a: [f32; 3] = [0.500, 0.500, 0.500];
    let b: [f32; 3] = [0.500, 0.500, 0.500];
    let c: [f32; 3] = [1.000, 1.000, 1.000];
    let d: [f32; 3] = [0.300, 0.423, 0.637];

    let mut col = [0.0; 3];

    for i in 0..3 {
        col[i] = a[i] + b[i] * (f32::consts::TAU * c[i] * t + d[i]);
    }

    col
}

fn length(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

fn fract(x: f32) -> f32 {
    x - x.floor()
}

impl Shader for NeonPlasma {
    fn name(&self) -> &'static str {
        "Plasma Neon"
    }

    fn evaluate(&self, uv: (f32, f32), time: f32, params: &ShaderParams) -> Color {
        let t = time * params.speed;
        let mut uv = (uv.0 * params.zoom, uv.1 * params.zoom);
        let uv0 = uv;
        let mut final_color = [0.0, 0.0, 0.0];

        for i in 0..3 {
            let i_f = i as f32;

            uv = (fract(uv.0 * 1.5) - 0.5, fract(uv.1 * 1.5) - 0.5);

            let d0 = length(uv0.0, uv0.1);
            let mut d = length(uv.0, uv.1) * (-d0).exp();

            let col = palette(d0 + i_f * 0.4 + params.color_shift);

            d = ((d * 8.0 + t).sin() / 8.0).abs();

            d = (0.02 / d.max(0.001)).powf(1.2);

            final_color[0] += col[0] + d * params.intensity;
            final_color[1] += col[1] + d * params.intensity;
            final_color[2] += col[2] + d * params.intensity;
        }

        Color::from_normalized(final_color[0], final_color[1], final_color[2])
    }
}
