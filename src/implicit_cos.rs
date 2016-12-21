use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitCos {
    base: ImplicitModuleBase,
    source: ScalarParameter,
}

impl ImplicitCos {
    pub fn new() -> ImplicitCos {
        ImplicitCos {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitCos {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x, y).cos()
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x, y, z).cos()
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x, y, z, w).cos()
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x, y, z, w, u, v).cos()
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
