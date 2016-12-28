use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::curve::Curve;
use super::utility::clamp;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitModifier {
    base: ImplicitModuleBase,
    source: Option<Rc<RefCell<ImplicitModule>>>,
    curve: Curve<f64>,
}

impl ImplicitModifier {
    pub fn new() -> ImplicitModifier {
        ImplicitModifier {
            base: Default::default(),
            source: None,
            curve: Curve::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> ImplicitModifier {
        ImplicitModifier {
            base: Default::default(),
            source: None,
            curve: Curve::with_capacity(capacity),
        }
    }

    pub fn set_source(&mut self, source: Option<Rc<RefCell<ImplicitModule>>>) {
        self.source = source;
    }

    pub fn add_control_point(&mut self, v: f64, p: f64) {
        self.curve.push_point(v, p);
    }

    pub fn clear_control_points(&mut self) {
        self.curve.clear();
    }
}

impl ImplicitModule for ImplicitModifier {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        if let Some(ref s) = self.source {
            let mut v = s.borrow_mut().get_2d(x, y);
            // Should clamp; make sure inputs are in range
            // v=(v+1.0 ) * 0.5;
            v = clamp(v, 0.0, 1.0);
            self.curve.linear_interp(v)
        } else {
            0.0
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        if let Some(ref s) = self.source {
            let mut v = s.borrow_mut().get_3d(x, y, z);
            // v=(v+1)*0.5;
            v = clamp(v, 0.0, 1.0);
            self.curve.linear_interp(v)
        } else {
            0.0
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        if let Some(ref s) = self.source {
            let mut v = s.borrow_mut().get_4d(x, y, z, w);
            // Should clamp; make sure inputs are in range
            // v=(v+1.0 ) * 0.5;
            v = clamp(v, 0.0, 1.0);
            self.curve.linear_interp(v)
        } else {
            0.0
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        if let Some(ref s) = self.source {
            let mut val = s.borrow_mut().get_6d(x, y, z, w, u, v);
            // Should clamp; make sure inputs are in range
            // val=(val+1.0 ) * 0.5;
            val = clamp(val, 0.0, 1.0);
            self.curve.linear_interp(val)
        } else {
            0.0
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
