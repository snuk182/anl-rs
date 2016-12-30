use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};
use super::implicit_base::ScalarParameter;
use super::utility::quintic_blend;
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaSelect {
    low: RgbaParameter,
    high: RgbaParameter,
    control: ScalarParameter,
    threshold: ScalarParameter,
    falloff: ScalarParameter,
}

impl RgbaSelect {
    pub fn new() -> RgbaSelect {
        RgbaSelect {
            low: RgbaParameter::Constant(Rgba::with_value(0.0)),
            high: RgbaParameter::Constant(Rgba::with_value(0.0)),
            control: ScalarParameter::Value(0.0),
            threshold: ScalarParameter::Value(0.0),
            falloff: ScalarParameter::Value(0.0),
        }
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

    pub fn set_control_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
        self.control = ScalarParameter::Source(b);
    }

    pub fn set_control_source_value(&mut self, v: f64) {
        self.control = ScalarParameter::Value(v);
    }

    pub fn set_threshold_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
        self.threshold = ScalarParameter::Source(b);
    }

    pub fn set_threshold_source_value(&mut self, v: f64) {
        self.threshold = ScalarParameter::Value(v);
    }

    pub fn set_falloff_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
        self.falloff = ScalarParameter::Source(b);
    }

    pub fn set_falloff_source_value(&mut self, v: f64) {
        self.falloff = ScalarParameter::Value(v);
    }
}

impl RgbaModule for RgbaSelect {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let s1 = self.low.get_2d(x, y);
        let s2 = self.high.get_2d(x, y);
        let control = self.control.get_2d(x, y);
        let threshold = self.threshold.get_2d(x, y);
        let falloff = self.falloff.get_2d(x, y);
        get(&s1, &s2, control, threshold, falloff)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let s1 = self.low.get_3d(x, y, z);
        let s2 = self.high.get_3d(x, y, z);
        let control = self.control.get_3d(x, y, z);
        let threshold = self.threshold.get_3d(x, y, z);
        let falloff = self.falloff.get_3d(x, y, z);
        get(&s1, &s2, control, threshold, falloff)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let s1 = self.low.get_4d(x, y, z, w);
        let s2 = self.high.get_4d(x, y, z, w);
        let control = self.control.get_4d(x, y, z, w);
        let threshold = self.threshold.get_4d(x, y, z, w);
        let falloff = self.falloff.get_4d(x, y, z, w);
        get(&s1, &s2, control, threshold, falloff)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let s1 = self.low.get_6d(x, y, z, w, u, v);
        let s2 = self.high.get_6d(x, y, z, w, u, v);
        let control = self.control.get_6d(x, y, z, w, u, v);
        let threshold = self.threshold.get_6d(x, y, z, w, u, v);
        let falloff = self.falloff.get_6d(x, y, z, w, u, v);
        get(&s1, &s2, control, threshold, falloff)
    }
}

#[inline(always)]
fn get(s1: &Rgba, s2: &Rgba, control: f64, threshold: f64, falloff: f64) -> Rgba {
    if falloff > 0.0 {
        if control < (threshold - falloff) {
            *s1
        } else if control > (threshold + falloff) {
            *s2
        } else {
            let lower = threshold - falloff;
            let upper = threshold + falloff;
            let t = quintic_blend((control - lower) / (upper - lower));
            Rgba::with_all((s1[0] + t as f32 * (s2[0] - s1[0])) as f32,
                           (s1[1] + t as f32 * (s2[1] - s1[1])) as f32,
                           (s1[2] + t as f32 * (s2[2] - s1[2])) as f32,
                           (s1[3] + t as f32 * (s2[3] - s1[3])) as f32)
        }
    } else {
        if control < threshold { *s1 } else { *s2 }
    }
}
