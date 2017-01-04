/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::{ScalarParameter, ImplicitModule};
use super::utility::lerp;

use std::rc::Rc;
use std::cell::RefCell;

/// Blend has the effect of blending the value from `lowSource` with the value of `highSource` by linearly interpolating from one to the other using the value of `controlSource`. For best results, `controlSource` should output in the range of (0,1). All three inputs are scalar parameters that may accept either a constant value or a functional input as a source.
pub struct ImplicitBlend {
    base: ImplicitModuleBase,
    low: ScalarParameter,
    high: ScalarParameter,
    control: ScalarParameter,
}

impl ImplicitBlend {
    pub fn new() -> ImplicitBlend {
        ImplicitBlend {
            base: Default::default(),
            low: ScalarParameter::Value(0.0),
            high: ScalarParameter::Value(0.0),
            control: ScalarParameter::Value(0.0),
        }
    }

	pub fn set_low_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.low = ScalarParameter::Source(m);
    }
    pub fn set_low_value(&mut self, v: f64) {
        self.low = ScalarParameter::Value(v);
    }

    pub fn set_high_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.high = ScalarParameter::Source(m);
    }
    pub fn set_high_value(&mut self, v: f64) {
        self.high = ScalarParameter::Value(v);
    }
    pub fn set_control_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.control = ScalarParameter::Source(m);
    }
    pub fn set_control_value(&mut self, v: f64) {
        self.control = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitBlend {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let v1 = self.low.get_2d(x, y);
        let v2 = self.high.get_2d(x, y);
        let mut blend = self.control.get_2d(x, y);
        blend = (blend + 1.0) * 0.5;
        lerp(blend, v1, v2)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let v1 = self.low.get_3d(x, y, z);
        let v2 = self.high.get_3d(x, y, z);
        let blend = self.control.get_3d(x, y, z);
        lerp(blend, v1, v2)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let v1 = self.low.get_4d(x, y, z, w);
        let v2 = self.high.get_4d(x, y, z, w);
        let blend = self.control.get_4d(x, y, z, w);
        lerp(blend, v1, v2)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let v1 = self.low.get_6d(x, y, z, w, u, v);
        let v2 = self.high.get_6d(x, y, z, w, u, v);
        let blend = self.control.get_6d(x, y, z, w, u, v);
        lerp(blend, v1, v2)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
