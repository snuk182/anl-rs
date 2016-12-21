use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitScaleDomain {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    sx: ScalarParameter,
    sy: ScalarParameter,
    sz: ScalarParameter,
    sw: ScalarParameter,
    su: ScalarParameter,
    sv: ScalarParameter,
}

impl ImplicitScaleDomain {
    pub fn new(x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> ImplicitScaleDomain {
        ImplicitScaleDomain {
            sx: ScalarParameter::Value(x),
            sy: ScalarParameter::Value(y),
            sz: ScalarParameter::Value(z),
            sw: ScalarParameter::Value(w),
            su: ScalarParameter::Value(u),
            sv: ScalarParameter::Value(v),
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
        }
    }

    pub fn empty() -> ImplicitScaleDomain {
        ImplicitScaleDomain::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0)
    }

    pub fn set_scale(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) {
        self.sx = ScalarParameter::Value(x);
        self.sy = ScalarParameter::Value(y);
        self.sz = ScalarParameter::Value(z);
        self.sw = ScalarParameter::Value(w);
        self.su = ScalarParameter::Value(u);
        self.sv = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }

    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }

    pub fn set_x_scale_value(&mut self, x: f64) {
        self.sx = ScalarParameter::Value(x)
    }
    pub fn set_y_scale_value(&mut self, y: f64) {
        self.sy = ScalarParameter::Value(y)
    }
    pub fn set_z_scale_value(&mut self, z: f64) {
        self.sz = ScalarParameter::Value(z)
    }
    pub fn set_w_scale_value(&mut self, w: f64) {
        self.sw = ScalarParameter::Value(w)
    }
    pub fn set_u_scale_value(&mut self, u: f64) {
        self.su = ScalarParameter::Value(u)
    }
    pub fn set_v_scale_value(&mut self, v: f64) {
        self.sv = ScalarParameter::Value(v)
    }
    pub fn set_x_scale(&mut self, x: Rc<RefCell<ImplicitModule>>) {
        self.sx = ScalarParameter::Source(x)
    }
    pub fn set_y_scale(&mut self, y: Rc<RefCell<ImplicitModule>>) {
        self.sy = ScalarParameter::Source(y)
    }
    pub fn set_z_scale(&mut self, z: Rc<RefCell<ImplicitModule>>) {
        self.sz = ScalarParameter::Source(z)
    }
    pub fn set_w_scale(&mut self, w: Rc<RefCell<ImplicitModule>>) {
        self.sw = ScalarParameter::Source(w)
    }
    pub fn set_u_scale(&mut self, u: Rc<RefCell<ImplicitModule>>) {
        self.su = ScalarParameter::Source(u)
    }
    pub fn set_v_scale(&mut self, v: Rc<RefCell<ImplicitModule>>) {
        self.sv = ScalarParameter::Source(v)
    }
}

impl ImplicitModule for ImplicitScaleDomain {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x * self.sx.get_2d(x, y), y * self.sy.get_2d(x, y))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x * self.sx.get_3d(x, y, z),
                           y * self.sy.get_3d(x, y, z),
                           z * self.sz.get_3d(x, y, z))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x * self.sx.get_4d(x, y, z, w),
                           y * self.sy.get_4d(x, y, z, w),
                           z * self.sz.get_4d(x, y, z, w),
                           w * self.sw.get_4d(x, y, z, w))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x * self.sx.get_6d(x, y, z, w, u, v),
                           y * self.sy.get_6d(x, y, z, w, u, v),
                           z * self.sz.get_6d(x, y, z, w, u, v),
                           w * self.sw.get_6d(x, y, z, w, u, v),
                           u * self.su.get_6d(x, y, z, w, u, v),
                           v * self.sv.get_6d(x, y, z, w, u, v))
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
