use super::{Color, Shader, ShaderParams};

pub struct Mountains;

impl Shader for Mountains {
    fn name(&self) -> &'static str {
        "Misty Mountains"
    }

    fn evaluate(&self, uv: (f32, f32), time: f32, params: &ShaderParams) -> Color {
        let (u, v) = uv;

        //So first I think I would be writing a gradient on y axis, from dark indigo at top,
        //to warm-ish peach sunset at origin
        //
        // Then, maybe a sun.. a circular 2D disk behind mountain.
        //
        // And a GOATED silhoutte of mount fuji itself.. Maybe use assymetric slop formula
        // for that

        Color::WHITE
    }
}
