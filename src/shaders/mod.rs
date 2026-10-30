pub mod mountains;
pub mod plasma;
pub mod tunnel;

/// The RGB Channel Color bytes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Converts from normalized floats (0.0 to 1.0) to 0-255 RGB bytes
    pub fn from_normalized(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

/// Paramter's that TUI can adjust on sliders
/// Yall can also prolly store some presets
/// of values in this struct
#[derive(Clone, Copy)]
pub struct ShaderParams {
    pub speed: f32,       //like, from 0.5x to 3.0x
    pub zoom: f32,        //like fromm 0.5x to 5.0x
    pub color_shift: f32, //will change the phase in pallete
    pub intensity: f32,   // to adjust brightness/contrast
}

impl Default for ShaderParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            zoom: 1.0,
            color_shift: 1.0,
            intensity: 1.0,
        }
    }
}

pub trait Shader {
    fn name(&self) -> &'static str;

    /// The real rendering logic, in openGL, this would be mainImage()
    fn evaluate(&self, uv: (f32, f32), time: f32, params: &ShaderParams) -> Color;
}
