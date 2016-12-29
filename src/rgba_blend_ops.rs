use super::rgba_module_base::{RgbaModule, RgbaParameter};
use super::vector_types::Rgba;

use std::rc::Rc;
use std::cell::RefCell;

pub enum BlendOps {
    Src1Alpha,
    Src2Alpha,
    OneMinusSrc1Alpha,
    OneMinusSrc2Alpha,
    One,
    Zero,
}

pub struct RgbaBlendOps {
    source_1: RgbaParameter,
    source_2: RgbaParameter,
    blend_1: BlendOps,
    blend_2: BlendOps,
}

impl RgbaBlendOps {
    pub fn new() -> RgbaBlendOps {
        RgbaBlendOps {
            source_1: RgbaParameter::with_grey(0.0),
            source_2: RgbaParameter::with_grey(0.0),
            blend_1: BlendOps::Src1Alpha,
            blend_2: BlendOps::OneMinusSrc1Alpha,
        }
    }

    pub fn with_modes(m1: BlendOps, m2: BlendOps) -> RgbaBlendOps {
        RgbaBlendOps {
            source_1: RgbaParameter::with_grey(0.0),
            source_2: RgbaParameter::with_grey(0.0),
            blend_1: m1,
            blend_2: m2,
        }
    }

    pub fn set_source_1_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source_1 = RgbaParameter::Module(m);
    }
    pub fn set_source_1_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source_1 = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_source_2_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source_2 = RgbaParameter::Module(m);
    }
    pub fn set_source_2_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source_2 = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_source_1_blend(&mut self, b: BlendOps) {
        self.blend_1 = b;
    }
    pub fn set_source_2_blend(&mut self, b: BlendOps) {
        self.blend_2 = b;
    }

    fn blend_rgbas(&self, s1: &Rgba, s2: &Rgba) -> Rgba {
        let srcfactor = match self.blend_1 {
            BlendOps::Src1Alpha => s1[3],
            BlendOps::Src2Alpha => s2[3],
            BlendOps::OneMinusSrc1Alpha => 1.0 - s1[3],
            BlendOps::OneMinusSrc2Alpha => 1.0 - s2[3],
            BlendOps::One => 1.0,
            BlendOps::Zero => 0.0,
        };

        let dstfactor = match self.blend_2 {
            BlendOps::Src1Alpha => s1[3],
            BlendOps::Src2Alpha => s2[3],
            BlendOps::OneMinusSrc1Alpha => 1.0 - s1[3],
            BlendOps::OneMinusSrc2Alpha => 1.0 - s2[3],
            BlendOps::One => 1.0,
            BlendOps::Zero => 0.0,//1.0, // ???
        };

        Rgba::with_all(s1[0] * srcfactor + s2[0] * dstfactor,
                       s1[1] * srcfactor + s2[1] * dstfactor,
                       s1[2] * srcfactor + s2[2] * dstfactor,
                       s2[3])
    }
}

impl RgbaModule for RgbaBlendOps {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let s1 = self.source_1.get_2d(x, y);
        let s2 = self.source_2.get_2d(x, y);

        self.blend_rgbas(&s1, &s2)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let s1 = self.source_1.get_3d(x, y, z);
        let s2 = self.source_2.get_3d(x, y, z);

        self.blend_rgbas(&s1, &s2)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let s1 = self.source_1.get_4d(x, y, z, w);
        let s2 = self.source_2.get_4d(x, y, z, w);

        self.blend_rgbas(&s1, &s2)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let s1 = self.source_1.get_6d(x, y, z, w, u, v);
        let s2 = self.source_2.get_6d(x, y, z, w, u, v);

        self.blend_rgbas(&s1, &s2)
    }
}
