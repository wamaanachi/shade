use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use crate::app::App;
use crate::shaders::{Color, ShaderParams};

pub struct ShaderPreview<'a> {
    pub app: &'a App,
}

impl<'a> Widget for ShaderPreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 4 {
            return; // Too small to render
        }

        let preview_width = 40;
        let preview_height = 20;
        
        // Center the preview in the available area
        let start_x = area.x + (area.width.saturating_sub(preview_width as u16)) / 2;
        let start_y = area.y + (area.height.saturating_sub(preview_height as u16)) / 2;
        
        // Calculate aspect ratio from available space
        let aspect_ratio = area.width as f32 / area.height as f32;
        
        let shader_params = ShaderParams {
            speed: self.app.values[0],
            zoom: self.app.values[1],
            color_shift: self.app.values[2],
            intensity: self.app.values[3],
            aspect_ratio,
        };

        // Render shader to buffer
        render_shader_to_buffer(
            &self.app.current_shader,
            shader_params,
            (preview_width, preview_height),
            (start_x, start_y),
            buf,
            self.app.time,
            self.app.paused,
        );
    }
}

fn render_shader_to_buffer(
    shader: &crate::app::CurrentShader,
    params: ShaderParams,
    (width, height): (u16, u16),
    (start_x, start_y): (u16, u16),
    buf: &mut Buffer,
    time: f32,
    paused: bool,
) {
    let time = if paused { time } else { time };
    
    for y in 0..height {
        for x in 0..width {
            // Convert pixel coordinates to shader UV coordinates
            let u = ((x as f32 / width as f32) * 2.0 - 1.0) * (width as f32 / height as f32) * 0.5;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0);
            
            let color = shader.evaluate((u, v), time, &params);
            
            // Convert to terminal color
            let cell = &mut buf[(start_x + x, start_y + y)];
            cell.bg = ratatui::style::Color::Rgb(color.r, color.g, color.b);
            cell.set_symbol(" ");
        }
    }
}