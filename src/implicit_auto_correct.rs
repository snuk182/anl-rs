use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::utility::clamp;
use super::random_gen::{LCG, get_01};

use std::rc::Rc;
use std::cell::RefCell;

pub
struct ImplicitAutoCorrect {
	base: ImplicitModuleBase,
	source: Option<Rc<RefCell<ImplicitModule>>>,
	low: f64, 
	high: f64,
	scale2: f64, 
	offset2: f64,
	scale3: f64, 
	offset3: f64,
	scale4: f64, 
	offset4: f64,
	scale6: f64, 
	offset6: f64,
}

impl ImplicitAutoCorrect {
	pub fn with_source(source: Option<Rc<RefCell<ImplicitModule>>>) -> ImplicitAutoCorrect {
		ImplicitAutoCorrect {
			base: Default::default(),
			source: source,
			low: 0.0, 
			high: 0.0,
			scale2: 0.0, 
			offset2: 0.0,
			scale3: 0.0, 
			offset3: 0.0,
			scale4: 0.0, 
			offset4: 0.0,
			scale6: 0.0, 
			offset6: 0.0,
		}
	}
	
	pub fn with_range(low: f64, high: f64) -> ImplicitAutoCorrect {
		ImplicitAutoCorrect {
			base: Default::default(),
			source: None,
			low: low, 
			high: high,
			scale2: 0.0, 
			offset2: 0.0,
			scale3: 0.0, 
			offset3: 0.0,
			scale4: 0.0, 
			offset4: 0.0,
			scale6: 0.0, 
			offset6: 0.0,
		}
	}
	
	pub fn new() -> ImplicitAutoCorrect {
		ImplicitAutoCorrect::with_range(-1.0, 1.0)
	}
	
	pub fn calculate(&mut self) {
		match self.source {
			None => {},
			Some(ref mut s) => {
				let mut mn: f64;
				let mut mx: f64;
				
				let mut lcg = LCG::new();
				//lcg.setSeedTime();
			
				// Calculate 2D
				mn = 10000.0;
				mx = -10000.0;
				for _ in 0..10000 {
					let nx = get_01(&mut lcg)*4.0 - 2.0;
					let ny = get_01(&mut lcg)*4.0 - 2.0;
			
					let v = s.borrow_mut().get_2d(nx, ny);
					if v < mn {
						mn = v
					}
					if v > mx {
						mx = v
					}
				}
				self.scale2 = (self.high - self.low) / (mx - mn);
				self.offset2 = self.low - mn * self.scale2;
			
				// Calculate 3D
				mn = 10000.0;
				mx = -10000.0;
				for _ in 0..10000 {
					let nx = get_01(&mut lcg)*4.0 - 2.0;
					let ny = get_01(&mut lcg)*4.0 - 2.0;
					let nz = get_01(&mut lcg)*4.0 - 2.0;
			
					let v = s.borrow_mut().get_3d(nx, ny, nz);
					if v < mn {
						mn = v
					}
					if v > mx {
						mx = v
					}
				}
				self.scale3 = (self.high - self.low) / (mx - mn);
				self.offset3 = self.low - mn*self.scale3;
			
				// Calculate 4D
				let mut mn = 10000.0;
				let mut mx = -10000.0;
				for _ in 0..10000 {
					let nx = get_01(&mut lcg)*4.0 - 2.0;
					let ny = get_01(&mut lcg)*4.0 - 2.0;
					let nz = get_01(&mut lcg)*4.0 - 2.0;
					let nw = get_01(&mut lcg)*4.0 - 2.0;
			
					let v = s.borrow_mut().get_4d(nx, ny, nz, nw);
					if v < mn {
						mn = v
					}
					if v > mx {
						mx = v
					}
				}
				self.scale4 = (self.high - self.low) / (mx - mn);
				self.offset4 = self.low - mn*self.scale4;
			
				// Calculate 6D
				mn = 10000.0;
				mx = -10000.0;
				for _ in 0..10000 {
					let nx = get_01(&mut lcg)*4.0 - 2.0;
					let ny = get_01(&mut lcg)*4.0 - 2.0;
					let nz = get_01(&mut lcg)*4.0 - 2.0;
					let nw = get_01(&mut lcg)*4.0 - 2.0;
					let nu = get_01(&mut lcg)*4.0 - 2.0;
					let nv = get_01(&mut lcg)*4.0 - 2.0;
			
					let v = s.borrow_mut().get_6d(nx, ny, nz, nw, nu, nv);
					if v < mn {
						mn = v
					}
					if v > mx {
						mx = v
					}
				}
				self.scale6 = (self.high - self.low) / (mx - mn);
				self.offset6 = self.low - mn*self.scale6;
			}
		}
	}
	
	pub fn set_source(&mut self, m: Option<Rc<RefCell<ImplicitModule>>>) {
		self.source = m;
		self.calculate()
	}
	
	pub fn set_range(&mut self, low: f64, high: f64) {
		self.low = low;
		self.high = high;
		self.calculate();
	}
}

impl ImplicitModule for ImplicitAutoCorrect {
	fn set_seed(&mut self, _: u32) {}

	fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		match self.source {
			Some(ref mut s) => {
				let v = s.borrow_mut().get_2d(x, y);
				clamp(v*self.scale2+self.offset2, self.low, self.high)
			},
			None => 0.0
		}
	}
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		match self.source {
			Some(ref mut s) => {
				let v = s.borrow_mut().get_3d(x, y, z);
				clamp(v*self.scale3+self.offset3, self.low, self.high)
			},
			None => 0.0
		}
	}
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		match self.source {
			Some(ref mut s) => {
				let v = s.borrow_mut().get_4d(x, y, z, w);
				clamp(v*self.scale4+self.offset4, self.low, self.high)
			},
			None => 0.0
		}
	}
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		match self.source {
			Some(ref mut s) => {
				let val = s.borrow_mut().get_6d(x, y, z, w, u, v);
				clamp(val*self.scale6+self.offset6, self.low, self.high)
			},
			None => 0.0
		}
	}
	
	fn spacing(&self) -> f64 {
		self.base.spacing
	}
}