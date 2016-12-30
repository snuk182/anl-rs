use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaBlend {
    low: RgbaParameter,
    high: RgbaParameter,
    control: ScalarParameter,
}

impl RgbaBlend {
    pub fn new() -> RgbaBlend {
        RgbaBlend {
            low: RgbaParameter::with_grey(0.0),
            high: RgbaParameter::with_grey(0.0),
            control: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_control_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.control = ScalarParameter::Source(m);
    }
    pub fn set_control_value(&mut self, v: f64) {
        self.control = ScalarParameter::Value(v);
    }

    pub fn set_low_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.low = RgbaParameter::Module(m);
    }
    pub fn set_low_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.low = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_high_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.high = RgbaParameter::Module(m);
    }
    pub fn set_high_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.high = RgbaParameter::with_color(r, g, b, a);
    }
}

impl RgbaModule for RgbaBlend {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let low = self.low.get_2d(x, y);
        let high = self.high.get_2d(x, y);
        let control = self.control.get_2d(x, y) as f32;

        Rgba::with_all((low[0] + control * (high[0] - low[0])),
                       (low[1] + control * (high[1] - low[1])),
                       (low[2] + control * (high[2] - low[2])),
                       (low[3] + control * (high[3] - low[3])))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let low = self.low.get_3d(x, y, z);
        let high = self.high.get_3d(x, y, z);
        let control = self.control.get_3d(x, y, z) as f32;

        Rgba::with_all((low[0] + control * (high[0] - low[0])),
                       (low[1] + control * (high[1] - low[1])),
                       (low[2] + control * (high[2] - low[2])),
                       (low[3] + control * (high[3] - low[3])))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let low = self.low.get_4d(x, y, z, w);
        let high = self.high.get_4d(x, y, z, w);
        let control = self.control.get_4d(x, y, z, w) as f32;

        Rgba::with_all((low[0] + control * (high[0] - low[0])),
                       (low[1] + control * (high[1] - low[1])),
                       (low[2] + control * (high[2] - low[2])),
                       (low[3] + control * (high[3] - low[3])))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let low = self.low.get_6d(x, y, z, w, u, v);
        let high = self.high.get_6d(x, y, z, w, u, v);
        let control = self.control.get_6d(x, y, z, w, u, v) as f32;

        Rgba::with_all((low[0] + control * (high[0] - low[0])),
                       (low[1] + control * (high[1] - low[1])),
                       (low[2] + control * (high[2] - low[2])),
                       (low[3] + control * (high[3] - low[3])))
    }
}
