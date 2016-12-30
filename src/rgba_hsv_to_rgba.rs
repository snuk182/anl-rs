use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};
use super::hsv::hsv_to_rgba;

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaHsvToRgba {
    source: RgbaParameter,
}

impl RgbaHsvToRgba {
    pub fn new() -> RgbaHsvToRgba {
        RgbaHsvToRgba { source: RgbaParameter::Constant(Rgba::with_value(0.0)) }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source = RgbaParameter::Module(m);
    }
    pub fn set_source_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source = RgbaParameter::with_color(r, g, b, a);
    }
}

impl RgbaModule for RgbaHsvToRgba {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        hsv_to_rgba(&self.source.get_2d(x, y))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        hsv_to_rgba(&self.source.get_3d(x, y, z))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        hsv_to_rgba(&self.source.get_4d(x, y, z, w))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        hsv_to_rgba(&self.source.get_6d(x, y, z, w, u, v))
    }
}
