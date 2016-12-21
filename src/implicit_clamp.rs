use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::utility::clamp;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitClamp {
    base: ImplicitModuleBase,
    source: Rc<RefCell<ImplicitModule>>,
    low: f64,
    high: f64,
}

impl ImplicitClamp {
    pub fn new(low: f64, high: f64, source: Rc<RefCell<ImplicitModule>>) -> ImplicitClamp {
        ImplicitClamp {
            base: Default::default(),
            source: source,
            low: low,
            high: high,
        }
    }

    pub fn set_range(&mut self, low: f64, high: f64) {
        self.low = low;
        self.high = high;
    }

    pub fn set_source(&mut self, source: Rc<RefCell<ImplicitModule>>) {
        self.source = source;
    }
}

impl ImplicitModule for ImplicitClamp {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        clamp(self.source.borrow_mut().get_2d(x, y), self.low, self.high)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        clamp(self.source.borrow_mut().get_3d(x, y, z),
              self.low,
              self.high)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        clamp(self.source.borrow_mut().get_4d(x, y, z, w),
              self.low,
              self.high)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        clamp(self.source.borrow_mut().get_6d(x, y, z, w, u, v),
              self.low,
              self.high)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
