/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};
use super::utility::clamp;

use std::rc::Rc;
use std::cell::RefCell;

/// Generates a hyper sphere as a distance field. Values between the center and the radius are interpolated between 0 and 1, while values beyond radius are 0. The individual axis components of the center point, as well as the radius, are specifiable as either constants or as noise functions, making this an extremely flexible sphere indeed. Following are images taken as cross-sections of the sphere in the 2D plane, the one on the left with all constants for the components, the one in the center with constants for center and a fractal function for radius, and the one on the right with a fractal for the x component, and a constant radius.
pub struct ImplicitSphere {
    base: ImplicitModuleBase,
    cx: ScalarParameter,
    cy: ScalarParameter,
    cz: ScalarParameter,
    cw: ScalarParameter,
    cu: ScalarParameter,
    cv: ScalarParameter,
    radius: ScalarParameter,
}

impl Default for ImplicitSphere {
    fn default() -> Self {
        ImplicitSphere {
            base: Default::default(),
            cx: ScalarParameter::Value(0.0),
            cy: ScalarParameter::Value(0.0),
            cz: ScalarParameter::Value(0.0),
            cw: ScalarParameter::Value(0.0),
            cu: ScalarParameter::Value(0.0),
            cv: ScalarParameter::Value(0.0),
            radius: ScalarParameter::Value(1.0),
        }
    }
}

impl ImplicitSphere {
    pub fn new_6d(x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> ImplicitSphere {
        ImplicitSphere {
            cx: ScalarParameter::Value(x),
            cy: ScalarParameter::Value(y),
            cz: ScalarParameter::Value(z),
            cw: ScalarParameter::Value(w),
            cu: ScalarParameter::Value(u),
            cv: ScalarParameter::Value(v),
            ..Default::default()
        }
    }
    pub fn new_4d(x: f64, y: f64, z: f64, w: f64) -> ImplicitSphere {
        ImplicitSphere {
            cx: ScalarParameter::Value(x),
            cy: ScalarParameter::Value(y),
            cz: ScalarParameter::Value(z),
            cw: ScalarParameter::Value(w),
            ..Default::default()
        }
    }
    pub fn new_3d(x: f64, y: f64, z: f64) -> ImplicitSphere {
        ImplicitSphere {
            cx: ScalarParameter::Value(x),
            cy: ScalarParameter::Value(y),
            cz: ScalarParameter::Value(z),
            ..Default::default()
        }
    }
    pub fn new_2d(x: f64, y: f64) -> ImplicitSphere {
        ImplicitSphere {
            cx: ScalarParameter::Value(x),
            cy: ScalarParameter::Value(y),
            ..Default::default()
        }
    }

    pub fn set_cx_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cx = ScalarParameter::Source(m);
    }
    pub fn set_cx_value(&mut self, v: f64) {
        self.cx = ScalarParameter::Value(v);
    }

    pub fn set_cy_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cy = ScalarParameter::Source(m);
    }
    pub fn set_cy_value(&mut self, v: f64) {
        self.cy = ScalarParameter::Value(v);
    }

    pub fn set_cz_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cz = ScalarParameter::Source(m);
    }
    pub fn set_cz_value(&mut self, v: f64) {
        self.cz = ScalarParameter::Value(v);
    }

    pub fn set_cw_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cw = ScalarParameter::Source(m);
    }
    pub fn set_cw_value(&mut self, v: f64) {
        self.cw = ScalarParameter::Value(v);
    }

    pub fn set_cu_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cu = ScalarParameter::Source(m);
    }
    pub fn set_cu_value(&mut self, v: f64) {
        self.cu = ScalarParameter::Value(v);
    }

    pub fn set_cv_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.cv = ScalarParameter::Source(m);
    }
    pub fn set_cv_value(&mut self, v: f64) {
        self.cv = ScalarParameter::Value(v);
    }

    pub fn set_radius_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.radius = ScalarParameter::Source(m);
    }
    pub fn set_radius_value(&mut self, v: f64) {
        self.radius = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitSphere {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let dx = x - self.cx.get_2d(x, y);
        let dy = y - self.cy.get_2d(x, y);
        let len = (dx * dx + dy * dy).sqrt();
        let radius = self.radius.get_2d(x, y);
        let i = (radius - len) / radius;
        clamp(i, 0.0, 1.0)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let dx = x - self.cx.get_3d(x, y, z);
        let dy = y - self.cy.get_3d(x, y, z);
        let dz = z - self.cz.get_3d(x, y, z);
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        let radius = self.radius.get_3d(x, y, z);
        let i = (radius - len) / radius;
        clamp(i, 0.0, 1.0)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let dx = x - self.cx.get_4d(x, y, z, w);
        let dy = y - self.cy.get_4d(x, y, z, w);
        let dz = z - self.cz.get_4d(x, y, z, w);
        let dw = w - self.cw.get_4d(x, y, z, w);
        let len = (dx * dx + dy * dy + dz * dz + dw * dw).sqrt();
        let radius = self.radius.get_4d(x, y, z, w);
        let i = (radius - len) / radius;
        clamp(i, 0.0, 1.0)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let dx = x - self.cx.get_6d(x, y, z, w, u, v);
        let dy = y - self.cy.get_6d(x, y, z, w, u, v);
        let dz = z - self.cz.get_6d(x, y, z, w, u, v);
        let dw = w - self.cw.get_6d(x, y, z, w, u, v);
        let du = u - self.cu.get_6d(x, y, z, w, u, v);
        let dv = v - self.cv.get_6d(x, y, z, w, u, v);
        let len = (dx * dx + dy * dy + dz * dz + dw * dw + du * du + dv * dv).sqrt();
        let radius = self.radius.get_6d(x, y, z, w, u, v);
        let i = (radius - len) / radius;
        clamp(i, 0.0, 1.0)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
