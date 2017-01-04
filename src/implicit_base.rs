/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html). 

use std::rc::Rc;
use std::cell::RefCell;

pub const MAX_SOURCES: usize = 20;

/// Implicit modules output double-precision float values. Implicit functions are derived from ImplicitModule trait.
/// Noise values are obtained by calling one of the get() methods provided, with the appropriate number of coordinates. Note that the performance of the system as a hold is affected by the dimensionality of the function called, so a 6D function will take significantly longer than a 2D function. Typical applications will probably stick with 2D or 3D versions; the higher orders are provided for the purpose of [seamless mapping](http://accidentalnoise.sourceforge.net/seamlessnoise.html). 
pub trait ImplicitModule {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64;
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64;
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64;
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64;

    fn spacing(&self) -> f64;
    fn set_deriv_spacing(&mut self, s: f64);

    fn get_dx_2(&mut self, x: f64, y: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_2d(x - spacing, y);
        let maxval = self.get_2d(x + spacing, y);
        return (minval - maxval) / spacing;
    }

    fn get_dy_2(&mut self, x: f64, y: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_2d(x, y - spacing);
        let maxval = self.get_2d(x, y + spacing);
        return (minval - maxval) / spacing;
    }

    fn get_dx_3(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_3d(x - spacing, y, z);
        let maxval = self.get_3d(x + spacing, y, z);
        return (minval - maxval) / spacing;
    }

    fn get_dy_3(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_3d(x, y - spacing, z);
        let maxval = self.get_3d(x, y + spacing, z);
        return (minval - maxval) / spacing;
    }

    fn get_dz_3(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_3d(x, y, z - spacing);
        let maxval = self.get_3d(x, y, z + spacing);
        return (minval - maxval) / spacing;
    }

    fn get_dx_4(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_4d(x - spacing, y, z, w);
        let maxval = self.get_4d(x + spacing, y, z, w);
        return (minval - maxval) / spacing;
    }

    fn get_dy_4(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_4d(x, y - spacing, z, w);
        let maxval = self.get_4d(x, y + spacing, z, w);
        return (minval - maxval) / spacing;
    }

    fn get_dz_4(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_4d(x, y, z - spacing, w);
        let maxval = self.get_4d(x, y, z + spacing, w);
        return (minval - maxval) / spacing;
    }

    fn get_dw_4(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_4d(x, y, z, w - spacing);
        let maxval = self.get_4d(x, y, z, w + spacing);
        return (minval - maxval) / spacing;
    }

    fn get_dx_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x - spacing, y, z, w, u, v);
        let maxval = self.get_6d(x + spacing, y, z, w, u, v);
        return (minval - maxval) / spacing;
    }

    fn get_dy_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x, y - spacing, z, w, u, v);
        let maxval = self.get_6d(x, y + spacing, z, w, u, v);
        return (minval - maxval) / spacing;
    }

    fn get_dz_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x, y, z - spacing, w, u, v);
        let maxval = self.get_6d(x, y, z + spacing, w, u, v);
        return (minval - maxval) / spacing;
    }

    fn get_dw_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x, y, z, w - spacing, u, v);
        let maxval = self.get_6d(x, y, z, w + spacing, u, v);
        return (minval - maxval) / spacing;
    }

    fn get_du_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x, y, z, w, u - spacing, v);
        let maxval = self.get_6d(x, y, z, w, u + spacing, v);
        return (minval - maxval) / spacing;
    }

    fn get_dv_6(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let spacing = self.spacing();
        let minval = self.get_6d(x, y, z, w, u, v - spacing);
        let maxval = self.get_6d(x, y, z, w, u, v + spacing);
        return (minval - maxval) / spacing;
    }
}

pub struct ImplicitModuleBase {
    pub spacing: f64,
}

impl Default for ImplicitModuleBase {
    fn default() -> Self {
        ImplicitModuleBase { spacing: 0.0001 }
    }
}

/// Many functions have "parameters" that affect their functionality, aside from any "source" function they might have. An example is the ImplicitSelect function. This function has a control source, a low source, a high source, a threshold parameter, and a falloff parameter. The threshold parameter is used to select between the output of lowSource and highSource, depending on if the value of controlSource is above or below threshold. All five of these inputs are instances of what ANL calls a "scalar parameter". A scalar parameter can be set to either a constant (double-precision) value, or to another function. Most will default to some sane double-precision value (ie, 0), but if desired they can be overridden with any constant or any implicit functional output. In this way, complex behaviors can be obtained through a relatively simple interface.
pub enum ScalarParameter {
    Value(f64),
    Source(Rc<RefCell<ImplicitModule>>),
}

impl ScalarParameter {
    pub fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        match self {
            &mut ScalarParameter::Value(val) => val,
            &mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_2d(x, y),
        }
    }

    pub fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        match self {
            &mut ScalarParameter::Value(val) => val,
            &mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_3d(x, y, z),
        }
    }

    pub fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        match self {
            &mut ScalarParameter::Value(val) => val,
            &mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_4d(x, y, z, w),
        }
    }

    pub fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        match self {
            &mut ScalarParameter::Value(val) => val,
            &mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_6d(x, y, z, w, u, v),
        }
    }
}
