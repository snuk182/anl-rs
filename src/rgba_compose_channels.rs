use super::rgba_module_base::{RgbaModule, Rgba};
use super::hsv::hsv_to_rgba;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub enum CompositeChannelsMode {
    Rgb,
    Hsv,
}

pub struct RgbaComposeChannels {
    mode: CompositeChannelsMode,
    c1: ScalarParameter,
    c2: ScalarParameter,
    c3: ScalarParameter,
    c4: ScalarParameter,
}

impl RgbaComposeChannels {
    pub fn with_mode(m: CompositeChannelsMode) -> RgbaComposeChannels {
        RgbaComposeChannels {
            mode: m,
            c1: ScalarParameter::Value(0.0),
            c2: ScalarParameter::Value(0.0),
            c3: ScalarParameter::Value(0.0),
            c4: ScalarParameter::Value(0.0),
        }
    }

    pub fn set_red_or_hue_value(&mut self, v: f64) {
        self.c1 = ScalarParameter::Value(v)
    }
    pub fn set_green_or_saturation_value(&mut self, v: f64) {
        self.c2 = ScalarParameter::Value(v)
    }
    pub fn set_blue_or_value_value(&mut self, v: f64) {
        self.c3 = ScalarParameter::Value(v)
    }
    pub fn set_alpha_value(&mut self, v: f64) {
        self.c4 = ScalarParameter::Value(v)
    }
    pub fn set_red_or_hue_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.c1 = ScalarParameter::Source(m)
    }
    pub fn set_green_or_saturation_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.c2 = ScalarParameter::Source(m)
    }
    pub fn set_blue_or_value_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.c3 = ScalarParameter::Source(m)
    }
    pub fn set_alpha_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.c4 = ScalarParameter::Source(m)
    }
    
    #[inline(always)]
    fn get(&self, r: f64, g: f64, b: f64, a: f64) -> Rgba {
    	let rgba = Rgba::with_all(r as f32, g as f32, b as f32, a as f32);
        match self.mode {
            CompositeChannelsMode::Rgb => rgba,
            CompositeChannelsMode::Hsv => hsv_to_rgba(&rgba),
        }
    }
}

impl RgbaModule for RgbaComposeChannels {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let r = self.c1.get_2d(x, y);
        let g = self.c2.get_2d(x, y);
        let b = self.c3.get_2d(x, y);
        let a = self.c4.get_2d(x, y);

        self.get(r, g, b, a)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let r = self.c1.get_3d(x, y, z);
        let g = self.c2.get_3d(x, y, z);
        let b = self.c3.get_3d(x, y, z);
        let a = self.c4.get_3d(x, y, z);

        self.get(r, g, b, a)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let r = self.c1.get_4d(x, y, z, w);
        let g = self.c2.get_4d(x, y, z, w);
        let b = self.c3.get_4d(x, y, z, w);
        let a = self.c4.get_4d(x, y, z, w);

        self.get(r, g, b, a)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let r = self.c1.get_6d(x, y, z, w, u, v);
        let g = self.c2.get_6d(x, y, z, w, u, v);
        let b = self.c3.get_6d(x, y, z, w, u, v);
        let a = self.c4.get_6d(x, y, z, w, u, v);

        self.get(r, g, b, a)
    }
}
