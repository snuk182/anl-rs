/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::{ScalarParameter, ImplicitModule};
use super::utility::bias;

/// Bias has the effect of applying the following function to the output of its source:
///      `f(x)=pow(x, log(b)/log(0.5))`
/// When applied to a function in the range of (0,1), this has the effect of "pushing" the values of the function toward 1 if b is greater than 0.5, or pushing them toward 0 if b is less than 0.5. In effect, it "biases" the function toward one end of the range or the other.
/// `source` and `b` are scalar parameters. They default to 0 and 0.5, respectively, and may be overridden with either another constant or with a function source.
pub struct ImplicitBias {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    bias: ScalarParameter,
}

impl ImplicitBias {
    pub fn new(source: ScalarParameter, bias: f64) -> ImplicitBias {
        ImplicitBias {
            base: Default::default(),
            source: source,
            bias: ScalarParameter::Value(bias),
        }
    }

    pub fn set_source(&mut self, source: ScalarParameter) {
        self.source = source;
    }

    pub fn set_bias(&mut self, bias: f64) {
        self.bias = ScalarParameter::Value(bias);
    }

    pub fn set_bias_module(&mut self, bias: ScalarParameter) {
        self.bias = bias;
    }
}

impl ImplicitModule for ImplicitBias {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let va = self.source.get_2d(x, y);
        bias(self.bias.get_2d(x, y), va)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let va = self.source.get_3d(x, y, z);
        bias(self.bias.get_3d(x, y, z), va)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let va = self.source.get_4d(x, y, z, w);
        bias(self.bias.get_4d(x, y, z, w), va)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let va = self.source.get_6d(x, y, z, w, u, v);
        bias(self.bias.get_6d(x, y, z, w, u, v), va)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
