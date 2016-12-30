use super::rgba_module_base::{RgbaModule, Rgba};

pub struct RgbaConstant {
    rgba: Rgba,
}

impl RgbaConstant {
    pub fn with_colors(r: f32, g: f32, b: f32, a: f32) -> RgbaConstant {
        RgbaConstant { rgba: Rgba::with_all(r, g, b, a) }
    }

    pub fn with_rgba(rgba: &Rgba) -> RgbaConstant {
        RgbaConstant { rgba: rgba.clone() }
    }

    pub fn set_colors(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.rgba = Rgba::with_all(r, g, b, a);
    }

    pub fn set_rgba(&mut self, rgba: &Rgba) {
        self.rgba = rgba.clone();
    }
}

impl RgbaModule for RgbaConstant {
    fn get_2d(&mut self, _: f64, _: f64) -> Rgba {
        self.rgba
    }
    fn get_3d(&mut self, _: f64, _: f64, _: f64) -> Rgba {
        self.rgba
    }
    fn get_4d(&mut self, _: f64, _: f64, _: f64, _: f64) -> Rgba {
        self.rgba
    }
    fn get_6d(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64, _: f64) -> Rgba {
        self.rgba
    }
}
