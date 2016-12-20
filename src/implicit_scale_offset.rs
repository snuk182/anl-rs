use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitScaleOffset {
	base: ImplicitModuleBase,
	source: ScalarParameter, scale: ScalarParameter, offset: ScalarParameter,
}

impl ImplicitScaleOffset {
	pub fn with_scale_offset(scale: f64, offset: f64) -> ImplicitScaleOffset {
		ImplicitScaleOffset {
			base: Default::default(),
			source: ScalarParameter::Value(0.0), 
			scale: ScalarParameter::Value(scale), 
			offset: ScalarParameter::Value(offset),
		}
	}
	
	pub fn set_source_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
		self.source = ScalarParameter::Source(b);
	}
	pub fn set_source_value(&mut self, v: f64) {
		self.source = ScalarParameter::Value(v);
	}
	
	pub fn set_scale_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
		self.scale = ScalarParameter::Source(b);
	}
	pub fn set_scale_value(&mut self, v: f64) {
		self.scale = ScalarParameter::Value(v);
	}
	
	pub fn set_offset_module(&mut self, b: Rc<RefCell<ImplicitModule>>) {
		self.offset = ScalarParameter::Source(b);
	}
	pub fn set_offset_value(&mut self, v: f64) {
		self.offset = ScalarParameter::Value(v);
	}
}

impl ImplicitModule for ImplicitScaleOffset {
	fn set_seed(&mut self, _: u32) {}

	fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		self.source.get_2d(x, y)*self.scale.get_2d(x, y) + self.offset.get_2d(x, y)
	}
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		self.source.get_3d(x, y, z)*self.scale.get_3d(x, y, z) + self.offset.get_3d(x, y, z)
	}
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		self.source.get_4d(x, y, z, w)*self.scale.get_4d(x, y, z, w) + self.offset.get_4d(x, y, z, w)
	}
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		self.source.get_6d(x, y, z, w, u, v)*self.scale.get_6d(x, y, z, w, u, v) + self.offset.get_6d(x, y, z, w, u, v)
	}
	
	fn spacing(&self) -> f64 {
		self.base.spacing
	}
}