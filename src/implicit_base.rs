use std::rc::Rc;
use std::cell::RefCell;

pub const MAX_SOURCES: usize = 20; 

pub trait ImplicitModule {
	fn set_seed(&mut self, seed: u32);

	fn get_2d(&mut self, x: f64, y: f64) -> f64;
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64;
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64;
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64;
	
	fn spacing(&self) -> f64;
}

/*pub enum ImplicitModule {
	ImplicitCache,
}

impl ImplicitModule {
	fn set_seed(&mut self, seed: u32) {
		
	}

	fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		
	}
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		
	}
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		
	}
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		
	}
	
	fn spacing(&self) -> f64;
}*/

pub struct ImplicitModuleBase {
	pub spacing: f64
}

impl Default for ImplicitModuleBase {
	fn default() -> Self {
		ImplicitModuleBase {
			spacing: 0.0
		}
	}
}

pub fn get_dx_2(module: &mut ImplicitModule, x: f64, y: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_2d(x-spacing, y);
	let maxval = module.get_2d(x+spacing, y);
	return (minval - maxval) / spacing
}

pub fn get_dy_2(module: &mut ImplicitModule, x: f64, y: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_2d(x, y-spacing);
	let maxval = module.get_2d(x, y+spacing);
	return (minval - maxval) / spacing
}

pub fn get_dx_3(module: &mut ImplicitModule, x: f64, y: f64, z: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_3d(x-spacing, y, z);
	let maxval = module.get_3d(x+spacing, y, z);
	return (minval - maxval) / spacing
}

pub fn get_dy_3(module: &mut ImplicitModule, x: f64, y: f64, z: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_3d(x, y-spacing, z);
	let maxval = module.get_3d(x, y+spacing, z);
	return (minval - maxval) / spacing
}

pub fn get_dz_3(module: &mut ImplicitModule, x: f64, y: f64, z: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_3d(x, y, z-spacing);
	let maxval = module.get_3d(x, y, z+spacing);
	return (minval - maxval) / spacing
}

pub fn get_dx_4(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_4d(x-spacing, y, z, w);
	let maxval = module.get_4d(x+spacing, y, z, w);
	return (minval - maxval) / spacing
}

pub fn get_dy_4(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_4d(x, y-spacing, z, w);
	let maxval = module.get_4d(x, y+spacing, z, w);
	return (minval - maxval) / spacing
}

pub fn get_dz_4(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_4d(x, y, z-spacing, w);
	let maxval = module.get_4d(x, y, z+spacing, w);
	return (minval - maxval) / spacing
}

pub fn get_dw_4(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_4d(x, y, z, w-spacing);
	let maxval = module.get_4d(x, y, z, w+spacing);
	return (minval - maxval) / spacing
}

pub fn get_dx_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x-spacing, y, z, w, u, v);
	let maxval = module.get_6d(x+spacing, y, z, w, u, v);
	return (minval - maxval) / spacing
}

pub fn get_dy_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x, y-spacing, z, w, u, v);
	let maxval = module.get_6d(x, y+spacing, z, w, u, v);
	return (minval - maxval) / spacing
}

pub fn get_dz_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x, y, z-spacing, w, u, v);
	let maxval = module.get_6d(x, y, z+spacing, w, u, v);
	return (minval - maxval) / spacing
}

pub fn get_dw_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x, y, z, w-spacing, u, v);
	let maxval = module.get_6d(x, y, z, w+spacing, u, v);
	return (minval - maxval) / spacing
}

pub fn get_du_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x, y, z, w, u-spacing, v);
	let maxval = module.get_6d(x, y, z, w, u+spacing, v);
	return (minval - maxval) / spacing
}

pub fn get_dv_6(module: &mut ImplicitModule, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
	let spacing = module.spacing();
	let minval = module.get_6d(x, y, z, w, u, v-spacing);
	let maxval = module.get_6d(x, y, z, w, u, v+spacing);
	return (minval - maxval) / spacing
}

pub enum ScalarParameter {
	Value(f64),
	Source(Rc<RefCell<ImplicitModule>>),
}

impl ScalarParameter {
	pub fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		match self {
			&mut ScalarParameter::Value(val) => val,
			&mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_2d(x, y),
		}
	}
	
	pub fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		match self {
			&mut ScalarParameter::Value(val) => val,
			&mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_3d(x, y, z),
		}
	}
	
	pub fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		match self {
			&mut ScalarParameter::Value(val) => val,
			&mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_4d(x, y, z, w),
		}
	}
	
	pub fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		match self {
			&mut ScalarParameter::Value(val) => val,
			&mut ScalarParameter::Source(ref mut s) => s.borrow_mut().get_6d(x, y, z, w, u, v),
		}
	}
}
