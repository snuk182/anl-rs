use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaNormalize {
    source: RgbaParameter,
}

impl RgbaNormalize {
    pub fn new() -> RgbaNormalize {
        RgbaNormalize { source: RgbaParameter::Constant(Rgba::with_value(0.0)) }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source = RgbaParameter::Module(m);
    }
    pub fn set_source_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source = RgbaParameter::with_color(r, g, b, a);
    }
}

impl RgbaModule for RgbaNormalize {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        normalize(&self.source.get_2d(x, y))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        normalize(&self.source.get_3d(x, y, z))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        normalize(&self.source.get_4d(x, y, z, w))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        normalize(&self.source.get_6d(x, y, z, w, u, v))
    }
}

#[inline(always)]
fn normalize(s: &Rgba) -> Rgba {
    let mut len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    if len == 0.0 {
        return Rgba::with_value(0.0);
    }
    len = len.sqrt();
    Rgba::with_all(s[0] / len, s[1] / len, s[2] / len, s[3])
}
