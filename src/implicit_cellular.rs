use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::cellular_gen::*;

use std::rc::Rc;
use std::cell::RefCell;

pub
struct ImplicitCellular {
	base: ImplicitModuleBase,
	generator: Option<Rc<RefCell<CellularGenerator>>>,
	coefficients: [f64; 4],
}

impl ImplicitCellular {
	pub fn new() -> ImplicitCellular {
		ImplicitCellular {
			base: Default::default(),
			generator: None,
			coefficients: [1.0, 0.0, 0.0, 0.0],
		}
	}
	
	pub fn with_coefficients(a: f64, b: f64, c: f64, d: f64) -> ImplicitCellular {
		ImplicitCellular {
			base: Default::default(),
			generator: None,
			coefficients: [a, b, c, d],
		}
	}
	
	pub fn set_coefficients(&mut self, a: f64, b: f64, c: f64, d: f64) {
		self.coefficients[0] = a;
		self.coefficients[1] = b;
		self.coefficients[2] = c;
		self.coefficients[3] = d;
	}
	
	pub fn set_cellular_source(&mut self, source: Option<Rc<RefCell<CellularGenerator>>>) {
		self.generator = source;
	}
}

impl ImplicitModule for ImplicitCellular {
	fn set_seed(&mut self, seed: u32) {
		if let Some(ref mut g) = self.generator {
			g.borrow_mut().set_seed(seed);
		}
	}

	fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		match self.generator {
			None => 0.0,
			Some(ref mut g) => {
				let mut b = g.borrow_mut();
				let c = b.get_2d(x, y);
				c.f[0]*self.coefficients[0] + c.f[1]*self.coefficients[1] + c.f[2]*self.coefficients[2] + c.f[3]*self.coefficients[3]
			}
		}
	}
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		match self.generator {
			None => 0.0,
			Some(ref mut g) => {
				let mut b = g.borrow_mut();
				let c = b.get_3d(x, y, z);
				c.f[0]*self.coefficients[0] + c.f[1]*self.coefficients[1] + c.f[2]*self.coefficients[2] + c.f[3]*self.coefficients[3]
			}
		}
	}
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		match self.generator {
			None => 0.0,
			Some(ref mut g) => {
				let mut b = g.borrow_mut();
				let c = b.get_4d(x, y, z, w);
				c.f[0]*self.coefficients[0] + c.f[1]*self.coefficients[1] + c.f[2]*self.coefficients[2] + c.f[3]*self.coefficients[3]
			}
		}
	}
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		match self.generator {
			None => 0.0,
			Some(ref mut g) => {
				let mut b = g.borrow_mut();
				let c = b.get_6d(x, y, z, w, u, v);
				c.f[0]*self.coefficients[0] + c.f[1]*self.coefficients[1] + c.f[2]*self.coefficients[2] + c.f[3]*self.coefficients[3]
			}
		}
	}
	
	fn spacing(&self) -> f64 {
		self.base.spacing
	}
}