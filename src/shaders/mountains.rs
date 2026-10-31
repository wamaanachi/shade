use super::{Color, Shader, ShaderParams};

pub struct Mountains;

fn lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let mut out = [0.0; 3];

    // let r = ctop[0] * (1.0 - t) + cbot[0] * t;
    for i in 0..3 {
        out[i] = a[i] * (1.0 - t) + b[i] * t;
    }

    out
}

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

        // v from (u, v) dictates values on y axis, let's make modify color of that part
        //
        // So What I tried to do is called lerp (hehe), like linear interpolation
        // and hence I will jst move lerp-ing into separate function
        let mut t = ((v + 1.0) / 2.0).clamp(0.0, 1.0);
        t = t.powf(1.5);
        let ctop = [0.98, 0.53, 0.50];
        let cbot = [0.30, 0.37, 0.43];

        let sky_color = lerp(cbot, ctop, t);

        let mut col = sky_color;

        //now colors look so much better!!
        //let's get that sun in the place now...
        //

        //Defining a random coordinate for sun
        let x0: f32 = 0.35;
        let y0: f32 = 0.15;

        let dx = u - x0;
        let dy = v - y0;

        let dist = (dx * dx + dy * dy).sqrt();

        //0.35 here is radius of sun
        if dist < 0.35 {
            col = [0.85, 0.20, 0.20];
        }

        let x = u.abs().max(0.08);

        let mountain_h = 0.35 - (x - 0.08).powf(0.85) * 0.8;

        if v < mountain_h {
            let snowline = -0.25 + (u * 25.0).sin() * 0.03;
            if v > snowline {
                col = [0.95, 0.95, 0.98];
            } else {
                col = [0.45, 0.15, 0.12];
            }
        }

        // The mountain is looking awesome! We have snow on the fuji too!
        // Now time for petals! I will try with figuring out a single petal
        //
        // We need the petals to have no teleportation if widescreen, hence dynamic parameter
        let max_x = params.aspect_ratio + 0.2;
        let span_x = max_x * 2.0;
        let span_y = 2.2;

        let num_petals = 7.0;

        for i in 0..7 {
            let i_f = i as f32;

            let p_time = time + i_f * (6.0 / num_petals);

            // -1.0 is jst starting point, reducing it from 1 to a slowed down time, and wrapping it
            // with span_y, a dynamic value
            let petal_y = 1.0 - (p_time * 0.35) % span_y;

            //now we want natural drift to the petals, so I am gonna do some calculations
            //previously we would jst calculate flutter and drift in one place, now I wanna
            //separate them out

            let lanes = (i_f / num_petals) * span_x;
            let drift = (p_time * 0.2 + lanes) % span_x - max_x;
            let flutter = (p_time * 2.5 + i_f).sin() * 0.04;
            let petal_x = drift + flutter;

            let dx = u - petal_x;
            let dy = v - petal_y;

            let p_dist = (dx * dx + dy * dy).sqrt();

            if p_dist < 0.025 {
                col = [0.98, 0.72, 0.80];
            }
        }

        Color::from_normalized(col[0], col[1], col[2])
    }
}
