use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};
use super::utility::gain;

use std::rc::Rc;
use std::cell::RefCell; 

pub struct ImplicitGain {
	base: ImplicitModuleBase,
	source: ScalarParameter,
	gain: ScalarParameter,
}

impl ImplicitGain {
	pub fn with_gain(g: f64) -> ImplicitGain {
		ImplicitGain {
			base: Default::default(),
			source: ScalarParameter::Value(0.0),
			gain: ScalarParameter::Value(g),
		}
	}
	
	pub fn with_module(m: Rc<RefCell<ImplicitModule>>) -> ImplicitGain {
		ImplicitGain {
			base: Default::default(),
			source: ScalarParameter::Value(0.0),
			gain: ScalarParameter::Source(m),
		}
	}
	
    pub fn set_gain_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.gain = ScalarParameter::Source(m);
    }
    pub fn set_gain_value(&mut self, v: f64) {
        self.gain = ScalarParameter::Value(v);
    }
    
    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitGain {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        gain(self.gain.get_2d(x,y), self.source.get_2d(x,y))    	
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        gain(self.gain.get_3d(x,y,z), self.source.get_3d(x,y,z))  	
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        gain(self.gain.get_4d(x,y,z,w), self.source.get_4d(x,y,z,w))   	
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        gain(self.gain.get_6d(x,y,z,w,u,v), self.source.get_6d(x,y,z,w, u, v))    	
    }

    fn spacing(&self) -> f64 {
    	self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
    	self.base.spacing = s;
    }
}
