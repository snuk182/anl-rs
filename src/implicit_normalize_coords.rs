use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitNormalizeCoords {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    length: ScalarParameter,
}

impl ImplicitNormalizeCoords {
    pub fn new() -> ImplicitNormalizeCoords {
        ImplicitNormalizeCoords {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            length: ScalarParameter::Value(1.0),
        }
    }

    pub fn with_length(l: f64) -> ImplicitNormalizeCoords {
        ImplicitNormalizeCoords {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            length: ScalarParameter::Value(l),
        }
    }

    pub fn set_length_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.length = ScalarParameter::Source(m);
    }
    pub fn set_length_value(&mut self, v: f64) {
        self.length = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitNormalizeCoords {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        if x == 0.0 && y == 0.0 {
            return self.source.get_2d(x, y);
        }

        let len = (x * x + y * y).sqrt();
        let r = self.length.get_2d(x, y);
        self.source.get_2d(x / len * r, y / len * r)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        if x == 0.0 && y == 0.0 && z == 0.0 {
            return self.source.get_3d(x, y, z);
        }

        let len = (x * x + y * y + z * z).sqrt();
        let r = self.length.get_3d(x, y, z);
        self.source.get_3d(x / len * r, y / len * r, z / len * r)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        if x == 0.0 && y == 0.0 && z == 0.0 && w == 0.0 {
            return self.source.get_4d(x, y, z, w);
        }

        let len = (x * x + y * y + z * z + w * w).sqrt();
        let r = self.length.get_4d(x, y, z, w);
        self.source.get_4d(x / len * r, y / len * r, z / len * r, w / len * r)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        if x == 0.0 && y == 0.0 && z == 0.0 && w == 0.0 && u == 0.0 && v == 0.0 {
            return self.source.get_6d(x, y, z, w, u, v);
        }

        let len = (x * x + y * y + z * z + w * w + u * u + v * v).sqrt();
        let r = self.length.get_6d(x, y, z, w, u, v);
        self.source.get_6d(x / len * r,
                           y / len * r,
                           z / len * r,
                           w / len * r,
                           u / len * r,
                           v / len * r)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
