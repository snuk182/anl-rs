use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter, InterpType};
use super::curve::Curve;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitCurve {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    curve: Curve<f64>,
    itype: InterpType,
}

impl ImplicitCurve {
    pub fn new() -> ImplicitCurve {
        ImplicitCurve {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            itype: InterpType::Linear,
            curve: Curve::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> ImplicitCurve {
        ImplicitCurve {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            itype: InterpType::Linear,
            curve: Curve::with_capacity(capacity),
        }
    }

    pub fn push_point(&mut self, t: f64, v: f64) {
        self.curve.push_point(t, v);
    }

    pub fn set_interp_type(&mut self, t: InterpType) {
        self.itype = t;
    }

    pub fn set_source_value(&mut self, t: f64) {
        self.source = ScalarParameter::Value(t);
    }
    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
}

impl ImplicitModule for ImplicitCurve {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let t = self.source.get_2d(x, y);
        match self.itype {
            InterpType::None => self.curve.no_interp(t),
            InterpType::Linear => self.curve.linear_interp(t),
            InterpType::Cubic => self.curve.cubic_interp(t),
            InterpType::Quintic => self.curve.quintic_interp(t),
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let t = self.source.get_3d(x, y, z);
        match self.itype {
            InterpType::None => self.curve.no_interp(t),
            InterpType::Linear => self.curve.linear_interp(t),
            InterpType::Cubic => self.curve.cubic_interp(t),
            InterpType::Quintic => self.curve.quintic_interp(t),
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let t = self.source.get_4d(x, y, z, w);
        match self.itype {
            InterpType::None => self.curve.no_interp(t),
            InterpType::Linear => self.curve.linear_interp(t),
            InterpType::Cubic => self.curve.cubic_interp(t),
            InterpType::Quintic => self.curve.quintic_interp(t),
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let t = self.source.get_6d(x, y, z, w, u, v);
        match self.itype {
            InterpType::None => self.curve.no_interp(t),
            InterpType::Linear => self.curve.linear_interp(t),
            InterpType::Cubic => self.curve.cubic_interp(t),
            InterpType::Quintic => self.curve.quintic_interp(t),
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
