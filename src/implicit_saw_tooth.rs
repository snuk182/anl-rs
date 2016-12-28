use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitSawTooth {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    period: ScalarParameter,
}

impl ImplicitSawTooth {
    pub fn with_period(period: f64) -> ImplicitSawTooth {
        ImplicitSawTooth {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            period: ScalarParameter::Value(period),
        }
    }

    pub fn set_period_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.period = ScalarParameter::Source(m);
    }
    pub fn set_period_value(&mut self, v: f64) {
        self.period = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitSawTooth {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let val = self.source.get_2d(x, y);
        let p = self.period.get_2d(x, y);
        2.0 * (val / p - (0.5 + val / p).floor())
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let val = self.source.get_3d(x, y, z);
        let p = self.period.get_3d(x, y, z);
        2.0 * (val / p - (0.5 + val / p).floor())
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let val = self.source.get_4d(x, y, z, w);
        let p = self.period.get_4d(x, y, z, w);
        2.0 * (val / p - (0.5 + val / p).floor())
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let val = self.source.get_6d(x, y, z, w, u, v);
        let p = self.period.get_6d(x, y, z, w, u, v);
        2.0 * (val / p - (0.5 + val / p).floor())
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
