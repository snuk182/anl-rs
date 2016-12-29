use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};
use super::utility::quintic_blend;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitTiers {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    numtiers: usize,
    smooth: bool,
}

impl ImplicitTiers {
    pub fn new() -> ImplicitTiers {
        ImplicitTiers {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            numtiers: 0,
            smooth: true,
        }
    }

    pub fn with_num_tiers(tiers: usize, smooth: bool) -> ImplicitTiers {
        ImplicitTiers {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            numtiers: tiers,
            smooth: smooth,
        }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }

    pub fn set_num_tiers(&mut self, tiers: usize) {
        self.numtiers = tiers;
    }
    pub fn set_smooth(&mut self, v: bool) {
        self.smooth = v;
    }
}

impl ImplicitModule for ImplicitTiers {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut numsteps = self.numtiers;
        if self.smooth {
            numsteps -= 1;
        }
        let val = self.source.get_2d(x, y);
        let mut tb = (val * numsteps as f64).floor();
        let mut tt = tb + 1.0;
        let t = val * numsteps as f64 - tb;
        tb /= numsteps as f64;
        tt /= numsteps as f64;
        let u = if self.smooth { quintic_blend(t) } else { 0.0 };
        tb + u * (tt - tb)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut numsteps = self.numtiers;
        if self.smooth {
            numsteps -= 1;
        }
        let val = self.source.get_3d(x, y, z);
        let mut tb = (val * numsteps as f64).floor();
        let mut tt = tb + 1.0;
        let t = val * numsteps as f64 - tb;
        tb /= numsteps as f64;
        tt /= numsteps as f64;
        let u = if self.smooth { quintic_blend(t) } else { 0.0 };
        tb + u * (tt - tb)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut numsteps = self.numtiers;
        if self.smooth {
            numsteps -= 1;
        }
        let val = self.source.get_4d(x, y, z, w);
        let mut tb = (val * numsteps as f64).floor();
        let mut tt = tb + 1.0;
        let t = val * numsteps as f64 - tb;
        tb /= numsteps as f64;
        tt /= numsteps as f64;
        let u = if self.smooth { quintic_blend(t) } else { 0.0 };
        tb + u * (tt - tb)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut numsteps = self.numtiers;
        if self.smooth {
            numsteps -= 1;
        }
        let val = self.source.get_6d(x, y, z, w, u, v);
        let mut tb = (val * numsteps as f64).floor();
        let mut tt = tb + 1.0;
        let t = val * numsteps as f64 - tb;
        tb /= numsteps as f64;
        tt /= numsteps as f64;
        let s = if self.smooth { quintic_blend(t) } else { 0.0 };
        tb + s * (tt - tb)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
