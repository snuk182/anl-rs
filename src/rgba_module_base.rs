use super::vector_types::Rgba;

use std::rc::Rc;
use std::cell::RefCell;

pub trait RgbaModule {
	fn set_seed(&mut self, _seed: u32) {}
	
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba;
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba;
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba;
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba;
}

pub enum RgbaParameter {
	Constant(Rgba),
	Module(Rc<RefCell<RgbaModule>>),
}

impl RgbaParameter {
	pub fn with_module(m: Rc<RefCell<RgbaModule>>) -> RgbaParameter {
		RgbaParameter::Module(m)
	}
	
	pub fn with_color(r: f32, g: f32, b: f32, a: f32) -> RgbaParameter {
		RgbaParameter::Constant(Rgba::with_all(r,g,b,a))
	}
	
	pub fn with_grey(c: f32) -> RgbaParameter {
		RgbaParameter::Constant(Rgba::with_value(c))
	}
	
	pub fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
		match self {
			&mut RgbaParameter::Constant(c) => c,
			&mut RgbaParameter::Module(ref m) => m.borrow_mut().get_2d(x, y),
		}
	}
    pub fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
    	match self {
			&mut RgbaParameter::Constant(c) => c,
			&mut RgbaParameter::Module(ref m) => m.borrow_mut().get_3d(x, y, z),
		}
    }
    pub fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
    	match self {
			&mut RgbaParameter::Constant(c) => c,
			&mut RgbaParameter::Module(ref m) => m.borrow_mut().get_4d(x, y, z, w),
		}
    }
    pub fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
    	match self {
			&mut RgbaParameter::Constant(c) => c,
			&mut RgbaParameter::Module(ref m) => m.borrow_mut().get_6d(x, y, z, w, u, v),
		}
    }
}