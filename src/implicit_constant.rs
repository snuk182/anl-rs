use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;

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
