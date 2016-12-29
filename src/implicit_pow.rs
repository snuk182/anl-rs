use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitPow {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    power: ScalarParameter,
}

impl ImplicitPow {
    pub fn new() -> ImplicitPow {
        ImplicitPow {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            power: ScalarParameter::Value(1.0),
        }
    }

    pub fn set_power_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.power = ScalarParameter::Source(m);
    }
    pub fn set_power_value(&mut self, v: f64) {
        self.power = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitPow {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x, y).powf(self.power.get_2d(x, y))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x, y, z).powf(self.power.get_3d(x, y, z))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x, y, z, w).powf(self.power.get_4d(x, y, z, w))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x, y, z, w, u, v).powf(self.power.get_6d(x, y, z, w, u, v))
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
