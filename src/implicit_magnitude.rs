use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitMagnitude {
    base: ImplicitModuleBase,
    x: ScalarParameter,
    y: ScalarParameter,
    z: ScalarParameter,
    w: ScalarParameter,
    u: ScalarParameter,
    v: ScalarParameter,
}

impl ImplicitMagnitude {
    pub fn new() -> ImplicitMagnitude {
        ImplicitMagnitude {
            base: Default::default(),
            x: ScalarParameter::Value(0.0),
            y: ScalarParameter::Value(0.0),
            z: ScalarParameter::Value(0.0),
            w: ScalarParameter::Value(0.0),
            u: ScalarParameter::Value(0.0),
            v: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_x_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.x = ScalarParameter::Source(m);
    }
    pub fn set_x_value(&mut self, v: f64) {
        self.x = ScalarParameter::Value(v);
    }

    pub fn set_y_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.y = ScalarParameter::Source(m);
    }
    pub fn set_y_value(&mut self, v: f64) {
        self.y = ScalarParameter::Value(v);
    }
    pub fn set_z_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.z = ScalarParameter::Source(m);
    }
    pub fn set_z_value(&mut self, v: f64) {
        self.z = ScalarParameter::Value(v);
    }

    pub fn set_w_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.w = ScalarParameter::Source(m);
    }
    pub fn set_w_value(&mut self, v: f64) {
        self.w = ScalarParameter::Value(v);
    }

    pub fn set_u_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.u = ScalarParameter::Source(m);
    }
    pub fn set_u_value(&mut self, v: f64) {
        self.u = ScalarParameter::Value(v);
    }

    pub fn set_v_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.v = ScalarParameter::Source(m);
    }
    pub fn set_v_value(&mut self, v: f64) {
        self.v = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitMagnitude {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let xx = self.x.get_2d(x, y);
        let yy = self.y.get_2d(x, y);
        (xx * xx + yy * yy).sqrt()
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let xx = self.x.get_3d(x, y, z);
        let yy = self.y.get_3d(x, y, z);
        let zz = self.z.get_3d(x, y, z);
        (xx * xx + yy * yy + zz * zz).sqrt()
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let xx = self.x.get_4d(x, y, z, w);
        let yy = self.y.get_4d(x, y, z, w);
        let zz = self.z.get_4d(x, y, z, w);
        let ww = self.w.get_4d(x, y, z, w);
        (xx * xx + yy * yy + zz * zz + ww * ww).sqrt()
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let xx = self.x.get_6d(x, y, z, w, u, v);
        let yy = self.y.get_6d(x, y, z, w, u, v);
        let zz = self.z.get_6d(x, y, z, w, u, v);
        let ww = self.w.get_6d(x, y, z, w, u, v);
        let uu = self.u.get_6d(x, y, z, w, u, v);
        let vv = self.v.get_6d(x, y, z, w, u, v);
        (xx * xx + yy * yy + zz * zz + ww * ww + uu * uu + vv * vv).sqrt()
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
