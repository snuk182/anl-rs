use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub enum FunctionGradientAxis {
    X,
    Y,
    Z,
    W,
    U,
    V,
}

pub struct ImplicitFunctionGradient {
    base: ImplicitModuleBase,
    axis: FunctionGradientAxis,
    source: ScalarParameter,
    spacing: f64,
}

impl ImplicitFunctionGradient {
    pub fn with_axis(a: FunctionGradientAxis) -> ImplicitFunctionGradient {
        ImplicitFunctionGradient {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            axis: a,
            spacing: 0.0,
        }
    }

    pub fn set_axis(&mut self, a: FunctionGradientAxis) {
        self.axis = a;
    }
    pub fn set_spacing(&mut self, s: f64) {
        self.spacing = s;
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitFunctionGradient {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        match self.axis {
            FunctionGradientAxis::X => (self.source.get_2d(x - self.spacing, y) - self.source.get_2d(x + self.spacing, y)) / self.spacing,
            FunctionGradientAxis::Y => (self.source.get_2d(x, y - self.spacing) - self.source.get_2d(x, y + self.spacing)) / self.spacing,
            FunctionGradientAxis::Z => 0.0,
            FunctionGradientAxis::W => 0.0,
            FunctionGradientAxis::U => 0.0,
            FunctionGradientAxis::V => 0.0,
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        match self.axis {
            FunctionGradientAxis::X => (self.source.get_3d(x - self.spacing, y, z) - self.source.get_3d(x + self.spacing, y, z)) / self.spacing,
            FunctionGradientAxis::Y => (self.source.get_3d(x, y - self.spacing, z) - self.source.get_3d(x, y + self.spacing, z)) / self.spacing,
            FunctionGradientAxis::Z => (self.source.get_3d(x, y, z - self.spacing) - self.source.get_3d(x, y, z + self.spacing)) / self.spacing,
            FunctionGradientAxis::W => 0.0,
            FunctionGradientAxis::U => 0.0,
            FunctionGradientAxis::V => 0.0,
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        match self.axis {
            FunctionGradientAxis::X => (self.source.get_4d(x - self.spacing, y, z, w) - self.source.get_4d(x + self.spacing, y, z, w)) / self.spacing,
            FunctionGradientAxis::Y => (self.source.get_4d(x, y - self.spacing, z, w) - self.source.get_4d(x, y + self.spacing, z, w)) / self.spacing,
            FunctionGradientAxis::Z => (self.source.get_4d(x, y, z - self.spacing, w) - self.source.get_4d(x, y, z + self.spacing, w)) / self.spacing,
            FunctionGradientAxis::W => (self.source.get_4d(x, y, z, w - self.spacing) - self.source.get_4d(x, y, z, w + self.spacing)) / self.spacing,
            FunctionGradientAxis::U => 0.0,
            FunctionGradientAxis::V => 0.0,
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        match self.axis {
            FunctionGradientAxis::X => (self.source.get_6d(x - self.spacing, y, z, w, u, v) - self.source.get_6d(x + self.spacing, y, z, w, u, v)) / self.spacing,
            FunctionGradientAxis::Y => (self.source.get_6d(x, y - self.spacing, z, w, u, v) - self.source.get_6d(x, y + self.spacing, z, w, u, v)) / self.spacing,
            FunctionGradientAxis::Z => (self.source.get_6d(x, y, z - self.spacing, w, u, v) - self.source.get_6d(x, y, z + self.spacing, w, u, v)) / self.spacing,
            FunctionGradientAxis::W => (self.source.get_6d(x, y, z, w - self.spacing, u, v) - self.source.get_6d(x, y, z, w + self.spacing, u, v)) / self.spacing,
            FunctionGradientAxis::U => (self.source.get_6d(x, y, z, w, u - self.spacing, v) - self.source.get_6d(x, y, z, w, u + self.spacing, v)) / self.spacing,
            FunctionGradientAxis::V => (self.source.get_6d(x, y, z, w, u, v - self.spacing) - self.source.get_6d(x, y, z, w, u, v + self.spacing)) / self.spacing,
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
