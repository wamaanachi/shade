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

        let r = length(u, v) * params.zoom;

        let speed = time * params.speed;
        let depth = 1.0 / r.max(0.01);
        let angle = v.atan2(u);

        let rings = (depth * 8.0 + speed).sin();
        let base = (angle * 8.0 + speed).sin();

        let pattern = rings * base;

        let fog = (r * 1.8).clamp(0.0, 1.0);

        let mut final_col = step(0.0, pattern);

        final_col *= fog * params.intensity;

        let (red, green, blue) = if final_col > 0.5 {
            (0.0 * r, 0.8 * r, 1.0 * r)
        } else {
            (0.4 * r, 0.05 * r, 0.6 * r)
        };

        Color::from_normalized(red, green, blue)
    }
}
