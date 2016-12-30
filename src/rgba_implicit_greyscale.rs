use super::rgba_module_base::{RgbaModule, Rgba};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaImplicitGreyscale {
	source: Option<Rc<RefCell<ImplicitModule>>>,
}

impl RgbaImplicitGreyscale {
	pub fn new() -> RgbaImplicitGreyscale {
		RgbaImplicitGreyscale {
			source: None,
		}
	}
	
	pub fn set_source(&mut self, m: Option<Rc<RefCell<ImplicitModule>>>) {
        self.source = m;
    }
}

impl RgbaModule for RgbaImplicitGreyscale {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
    	match self.source {
    		Some(ref s) => {
	    		let val = s.borrow_mut().get_2d(x,y);
	    		Rgba::with_all(val as f32, val as f32, val as f32, 1.0)
    		},
    		None => Rgba::with_value(0.0),
    	}     	
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
    	match self.source {
    		Some(ref s) => {
	    		let val = s.borrow_mut().get_3d(x,y,z);
	    		Rgba::with_all(val as f32, val as f32, val as f32, 1.0)
    		},
    		None => Rgba::with_value(0.0),
    	}     	
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
    	match self.source {
    		Some(ref s) => {
	    		let val = s.borrow_mut().get_4d(x,y,z,w);
	    		Rgba::with_all(val as f32, val as f32, val as f32, 1.0)
    		},
    		None => Rgba::with_value(0.0),
    	}     	
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
    	match self.source {
    		Some(ref s) => {
	    		let val = s.borrow_mut().get_6d(x,y,z,w,u,v);
	    		Rgba::with_all(val as f32, val as f32, val as f32, 1.0)
    		},
    		None => Rgba::with_value(0.0),
    	} 	
    }
}
