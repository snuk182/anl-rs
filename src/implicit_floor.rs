use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitFloor {
	base: ImplicitModuleBase,
	source: ScalarParameter,
}

impl ImplicitFloor {
    pub fn new() -> ImplicitFloor {
        ImplicitFloor {
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

impl ImplicitModule for ImplicitFloor {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x, y).floor()
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x, y, z).floor()
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x, y, z, w).floor()
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x, y, z, w, u, v).floor()
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}