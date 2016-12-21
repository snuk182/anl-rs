use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitTranslateDomain {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    ax: ScalarParameter,
    ay: ScalarParameter,
    az: ScalarParameter,
    aw: ScalarParameter,
    au: ScalarParameter,
    av: ScalarParameter,
}

impl ImplicitTranslateDomain {
    pub fn new() -> ImplicitTranslateDomain {
        ImplicitTranslateDomain {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            ax: ScalarParameter::Value(0.0),
            ay: ScalarParameter::Value(0.0),
            az: ScalarParameter::Value(0.0),
            aw: ScalarParameter::Value(0.0),
            au: ScalarParameter::Value(0.0),
            av: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_x_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.ax = ScalarParameter::Source(m);
    }

    pub fn set_x_axis_source_value(&mut self, v: f64) {
        self.ax = ScalarParameter::Value(v);
    }

    pub fn set_y_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.ay = ScalarParameter::Source(m);
    }

    pub fn set_y_axis_source_value(&mut self, v: f64) {
        self.ay = ScalarParameter::Value(v);
    }

    pub fn set_z_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.az = ScalarParameter::Source(m);
    }

    pub fn set_z_axis_source_value(&mut self, v: f64) {
        self.az = ScalarParameter::Value(v);
    }

    pub fn set_w_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.aw = ScalarParameter::Source(m);
    }

    pub fn set_w_axis_source_value(&mut self, v: f64) {
        self.aw = ScalarParameter::Value(v);
    }

    pub fn set_u_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.au = ScalarParameter::Source(m);
    }

    pub fn set_u_axis_source_value(&mut self, v: f64) {
        self.au = ScalarParameter::Value(v);
    }

    pub fn set_v_axis_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.av = ScalarParameter::Source(m);
    }

    pub fn set_v_axis_source_value(&mut self, v: f64) {
        self.av = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }

    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitTranslateDomain {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x + self.ax.get_2d(x, y), y + self.ay.get_2d(x, y))
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x + self.ax.get_3d(x, y, z),
                           y + self.ay.get_3d(x, y, z),
                           z + self.az.get_3d(x, y, z))
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x + self.ax.get_4d(x, y, z, w),
                           y + self.ay.get_4d(x, y, z, w),
                           z + self.az.get_4d(x, y, z, w),
                           w + self.aw.get_4d(x, y, z, w))
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x + self.ax.get_6d(x, y, z, w, u, v),
                           y + self.ay.get_6d(x, y, z, w, u, v),
                           z + self.az.get_6d(x, y, z, w, u, v),
                           w + self.aw.get_6d(x, y, z, w, u, v),
                           u + self.au.get_6d(x, y, z, w, u, v),
                           v + self.av.get_6d(x, y, z, w, u, v))
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
