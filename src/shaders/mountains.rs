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
        // and finally, small sakura fall.. would look fabulous.
        //
        // Let's get started!!
        //

        // v from (u, v) dictates values on y axis, let's make modify color of that part
        let mut t = ((v + 1.0) / 2.0).clamp(0.0, 1.0);
        t = t.powf(1.5);
        let ctop = [0.98, 0.53, 0.50];
        let cbot = [0.30, 0.37, 0.43];

        let r = ctop[0] * (1.0 - t) + cbot[0] * t;
        let g = ctop[1] * (1.0 - t) + cbot[1] * t;
        let b = ctop[2] * (1.0 - t) + cbot[2] * t;

        //now colors look so much better!!
        //let's get that sun in the place now...

        Color::from_normalized(r, g, b)
    }
}
