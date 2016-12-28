use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::rgba_module_base::{RgbaParameter, RgbaModule};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitRgbaDotProduct {
    base: ImplicitModuleBase,
    source_1: RgbaParameter,
    source_2: RgbaParameter,
}

impl ImplicitRgbaDotProduct {
    pub fn new() -> ImplicitRgbaDotProduct {
        ImplicitRgbaDotProduct {
            base: Default::default(),
            source_1: RgbaParameter::with_grey(0.0),
            source_2: RgbaParameter::with_grey(0.0),
        }
    }

    pub fn set_source_1_constant(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source_1 = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_source_1_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source_1 = RgbaParameter::with_module(m);
    }

    pub fn set_source_2_constant(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source_2 = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_source_2_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source_2 = RgbaParameter::with_module(m);
    }
}

impl ImplicitModule for ImplicitRgbaDotProduct {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let s1 = self.source_1.get_2d(x, y);
        let s2 = self.source_2.get_2d(x, y);
        (s1[0] * s2[0] + s1[1] * s2[1] + s1[2] * s2[2] + s1[3] * s2[3]) as f64
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let s1 = self.source_1.get_3d(x, y, z);
        let s2 = self.source_2.get_3d(x, y, z);
        (s1[0] * s2[0] + s1[1] * s2[1] + s1[2] * s2[2] + s1[3] * s2[3]) as f64
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let s1 = self.source_1.get_4d(x, y, z, w);
        let s2 = self.source_2.get_4d(x, y, z, w);
        (s1[0] * s2[0] + s1[1] * s2[1] + s1[2] * s2[2] + s1[3] * s2[3]) as f64
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let s1 = self.source_1.get_6d(x, y, z, w, u, v);
        let s2 = self.source_2.get_6d(x, y, z, w, u, v);
        (s1[0] * s2[0] + s1[1] * s2[1] + s1[2] * s2[2] + s1[3] * s2[3]) as f64
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
