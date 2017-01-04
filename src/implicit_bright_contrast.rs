/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::{ScalarParameter, ImplicitModule};

use std::rc::Rc;
use std::cell::RefCell;

/// The BrightContrast function provides a method for adjusting the brightness and contrast of the image. Contrast is adjusted relative to a specified threshold. Values greater than the threshold are adjust upward (brighter), while values below the threshold are adjusted downward (darker). (Note: The brightness and contrast correction provided by this function are additive, meaning that all values are brightened or darkened across the applicable range, unlike the [`Bias`](struct.ImplicitBias.html) function, which lightens or darkens, but preserves the total range of the function.
pub struct ImplicitBrightContrast {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    bright: ScalarParameter,
    threshold: ScalarParameter,
    factor: ScalarParameter,
}

impl ImplicitBrightContrast {
    pub fn new() -> ImplicitBrightContrast {
        ImplicitBrightContrast {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            bright: ScalarParameter::Value(0.0),
            threshold: ScalarParameter::Value(0.0),
            factor: ScalarParameter::Value(1.0),
        }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }

    pub fn set_bright_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.bright = ScalarParameter::Source(m);
    }
    pub fn set_bright_value(&mut self, v: f64) {
        self.bright = ScalarParameter::Value(v);
    }
    pub fn set_threshold_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.threshold = ScalarParameter::Source(m);
    }
    pub fn set_threshold_value(&mut self, v: f64) {
        self.threshold = ScalarParameter::Value(v);
    }

    pub fn set_factor_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.factor = ScalarParameter::Source(m);
    }
    pub fn set_factor_value(&mut self, v: f64) {
        self.factor = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitBrightContrast {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut v = self.source.get_2d(x, y);
        // Apply brightness
        v += self.bright.get_2d(x, y);

        // Subtract threshold, scale by factor, add threshold
        let threshold = self.threshold.get_2d(x, y);
        v -= threshold;
        v *= self.factor.get_2d(x, y);
        v += threshold;
        v
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut v = self.source.get_3d(x, y, z);
        // Apply brightness
        v += self.bright.get_3d(x, y, z);

        // Subtract threshold, scale by factor, add threshold
        let threshold = self.threshold.get_3d(x, y, z);
        v -= threshold;
        v *= self.factor.get_3d(x, y, z);
        v += threshold;
        v
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut v = self.source.get_4d(x, y, z, w);
        // Apply brightness
        v += self.bright.get_4d(x, y, z, w);

        // Subtract threshold, scale by factor, add threshold
        let threshold = self.threshold.get_4d(x, y, z, w);
        v -= threshold;
        v *= self.factor.get_4d(x, y, z, w);
        v += threshold;
        v
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut c = self.source.get_6d(x, y, z, w, u, v);
        // Apply brightness
        c += self.bright.get_6d(x, y, z, w, u, v);

        // Subtract threshold, scale by factor, add threshold
        let threshold = self.threshold.get_6d(x, y, z, w, u, v);
        c -= threshold;
        c *= self.factor.get_6d(x, y, z, w, u, v);
        c += threshold;
        c
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
