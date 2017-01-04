/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;

/// Constant is simply that: a constant value. Note: This may be going away, as the increasing use of ScalarParameters (parameters that can either be a constant or a noise source) is making it irrelevant. Even in the case of something like a combiner, which still takes pointers to modules rather than a scalar parameter, it is just as easy to use a [`Cache`](struct.ImplicitCache.html) with a constant source.
pub struct ImplicitConstant {
    base: ImplicitModuleBase,
    constant: f64,
}

impl ImplicitConstant {
    pub fn new(constant: f64) -> ImplicitConstant {
        ImplicitConstant {
            base: Default::default(),
            constant: constant,
        }
    }

    pub fn set_constant(&mut self, constant: f64) {
        self.constant = constant;
    }
}

impl ImplicitModule for ImplicitConstant {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, _: f64, _: f64) -> f64 {
        self.constant
    }
    fn get_3d(&mut self, _: f64, _: f64, _: f64) -> f64 {
        self.constant
    }
    fn get_4d(&mut self, _: f64, _: f64, _: f64, _: f64) -> f64 {
        self.constant
    }
    fn get_6d(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64, _: f64) -> f64 {
        self.constant
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
