use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::utility::{lerp, quintic_blend};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitSelect {
    base: ImplicitModuleBase,
    low: ScalarParameter,
    high: ScalarParameter,
    control: ScalarParameter,
    threshold: ScalarParameter,
    falloff: ScalarParameter,
}

impl ImplicitSelect {
    pub fn new() -> ImplicitSelect {
        ImplicitSelect {
            base: Default::default(),
            low: ScalarParameter::Value(0.0),
            high: ScalarParameter::Value(0.0),
            control: ScalarParameter::Value(0.0),
            threshold: ScalarParameter::Value(0.0),
            falloff: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_low_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
        self.low = ScalarParameter::Source(b);
    }

    pub fn set_low_source_value(&mut self, v: f64) {
        self.low = ScalarParameter::Value(v);
    }

    pub fn set_high_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
        self.high = ScalarParameter::Source(b);
    }

    pub fn set_high_source_value(&mut self, v: f64) {
        self.high = ScalarParameter::Value(v);
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

impl ImplicitModule for ImplicitSelect {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let control = self.control.get_2d(x, y);
        let falloff = self.falloff.get_2d(x, y);
        let threshold = self.threshold.get_2d(x, y);

        if falloff > 0.0 {
            if control < (threshold - falloff) {
                self.low.get_2d(x, y)
            } else if control > (threshold + falloff) {
                self.high.get_2d(x, y)
            } else {
                let lower = threshold - falloff;
                let upper = threshold + falloff;
                let blend = quintic_blend((control - lower) / (upper - lower));
                lerp(blend, self.low.get_2d(x, y), self.high.get_2d(x, y))
            }
        } else {
            if control < threshold {
                self.low.get_2d(x, y)
            } else {
                self.high.get_2d(x, y)
            }
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let control = self.control.get_3d(x, y, z);
        let falloff = self.falloff.get_3d(x, y, z);
        let threshold = self.threshold.get_3d(x, y, z);

        if falloff > 0.0 {
            if control < (threshold - falloff) {
                self.low.get_3d(x, y, z)
            } else if control > (threshold + falloff) {
                self.high.get_3d(x, y, z)
            } else {
                let lower = threshold - falloff;
                let upper = threshold + falloff;
                let blend = quintic_blend((control - lower) / (upper - lower));
                lerp(blend, self.low.get_3d(x, y, z), self.high.get_3d(x, y, z))
            }
        } else {
            if control < threshold {
                self.low.get_3d(x, y, z)
            } else {
                self.high.get_3d(x, y, z)
            }
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let control = self.control.get_4d(x, y, z, w);
        let falloff = self.falloff.get_4d(x, y, z, w);
        let threshold = self.threshold.get_4d(x, y, z, w);

        if falloff > 0.0 {
            if control < (threshold - falloff) {
                self.low.get_4d(x, y, z, w)
            } else if control > (threshold + falloff) {
                self.high.get_4d(x, y, z, w)
            } else {
                let lower = threshold - falloff;
                let upper = threshold + falloff;
                let blend = quintic_blend((control - lower) / (upper - lower));
                lerp(blend,
                     self.low.get_4d(x, y, z, w),
                     self.high.get_4d(x, y, z, w))
            }
        } else {
            if control < threshold {
                self.low.get_4d(x, y, z, w)
            } else {
                self.high.get_4d(x, y, z, w)
            }
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let control = self.control.get_6d(x, y, z, w, u, v);
        let falloff = self.falloff.get_6d(x, y, z, w, u, v);
        let threshold = self.threshold.get_6d(x, y, z, w, u, v);

        if falloff > 0. {
            if control < (threshold - falloff) {
                self.low.get_6d(x, y, z, w, u, v)
            } else if control > (threshold + falloff) {
                self.high.get_6d(x, y, z, w, u, v)
            } else {
                let lower = threshold - falloff;
                let upper = threshold + falloff;
                let blend = quintic_blend((control - lower) / (upper - lower));
                lerp(blend,
                     self.low.get_6d(x, y, z, w, u, v),
                     self.high.get_6d(x, y, z, w, u, v))
            }
        } else {
            if control < threshold {
                self.low.get_6d(x, y, z, w, u, v)
            } else {
                self.high.get_6d(x, y, z, w, u, v)
            }
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
}
