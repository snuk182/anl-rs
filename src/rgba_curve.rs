use super::rgba_module_base::{RgbaModule, Rgba};
use super::{ImplicitModule, ScalarParameter, InterpType};
use super::curve::Curve;

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaCurve {
    curve: Curve<Rgba>,
    source: ScalarParameter,
    itype: InterpType,
}

impl RgbaCurve {
    pub fn with_interp_type(it: InterpType) -> RgbaCurve {
        RgbaCurve {
            curve: Curve::new(),
            source: ScalarParameter::Value(0.0),
            itype: it,
        }
    }

    pub fn with_interp_type_capacity(it: InterpType, capacity: usize) -> RgbaCurve {
        RgbaCurve {
            curve: Curve::with_capacity(capacity),
            source: ScalarParameter::Value(0.0),
            itype: it,
        }
    }

    pub fn push_point(&mut self, t: f64, r: f32, g: f32, b: f32, a: f32) {
        self.curve.push_point(t, Rgba::with_all(r, g, b, a));
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

    pub fn clear_curve(&mut self) {
        self.curve.clear();
    }
    
    #[inline(always)]
    fn get(&mut self, t: f64) -> Rgba {
    	match self.itype {
            InterpType::None => self.curve.no_interp(t),
            InterpType::Linear => self.curve.linear_interp(t),
            InterpType::Cubic => self.curve.cubic_interp(t),
            InterpType::Quintic => self.curve.quintic_interp(t),
        }
    }
}

impl RgbaModule for RgbaCurve {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let t = self.source.get_2d(x, y);
        self.get(t)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let t = self.source.get_3d(x, y, z);
        self.get(t)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let t = self.source.get_4d(x, y, z, w);
        self.get(t)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let t = self.source.get_6d(x, y, z, w, u, v);
        self.get(t)
    }
}
