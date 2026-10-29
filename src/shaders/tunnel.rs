use super::{Color, Shader, ShaderParams};

pub struct CyberTunnel;

fn step(edge: f32, x: f32) -> f32 {
    if x < edge { 0.0 } else { 1.0 }
}

fn length(u: f32, v: f32) -> f32 {
    (u * u + v * v).sqrt()
}

impl Shader for CyberTunnel {
    fn name(&self) -> &'static str {
        "Cyber Tunnel"
    }

    fn evaluate(&self, uv: (f32, f32), time: f32, params: &ShaderParams) -> Color {
        let (u, v) = uv;

        let r = length(u, v);

        let depth = 1.0 / r;
        let angle = v.atan2(u);

        let rings = (depth * 8.0 + time).sin();
        let base = (angle * 8.0 + time).sin();

        let pattern = rings * base;

        let final_col = step(0.1, pattern);

        Color::from_normalized(final_col, final_col, final_col)
    }
}
