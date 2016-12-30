use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};
use super::utility::{min, max};

use std::rc::Rc;
use std::cell::RefCell;

pub enum ColorOperation {
    ColorMultiply,
    ColorAdd,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    Dodge,
    Burn,
    LinearDodge,
    LinearBurn,
}

pub struct RgbaColorOps {
    source_1: RgbaParameter,
    source_2: RgbaParameter,
    op: ColorOperation,
}

impl RgbaColorOps {
    pub fn with_op(o: ColorOperation) -> RgbaColorOps {
        RgbaColorOps {
            source_1: RgbaParameter::with_grey(0.0),
            source_2: RgbaParameter::with_grey(0.0),
            op: o,
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

    pub fn set_operation(&mut self, op: ColorOperation) {
        self.op = op;
    }
    
    #[inline(always)]
    fn get(&self, s1: &Rgba, s2: &Rgba) -> Rgba {
    	match self.op {
            ColorOperation::ColorMultiply => multiply(s1, s2),
            ColorOperation::ColorAdd => add(s1, s2),
            ColorOperation::Screen => screen(s1, s2),
            ColorOperation::Overlay => overlay(s1, s2),
            ColorOperation::SoftLight => soft_light(s1, s2),
            ColorOperation::HardLight => hard_light(s1, s2),
            ColorOperation::Dodge => dodge(s1, s2),
            ColorOperation::Burn => burn(s1, s2),
            ColorOperation::LinearDodge => linear_dodge(s1, s2),
            ColorOperation::LinearBurn => linear_burn(s1, s2),
        }
    }
}

impl RgbaModule for RgbaColorOps {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let s1 = self.source_1.get_2d(x, y);
        let s2 = self.source_2.get_2d(x, y);
        self.get(&s1, &s2)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let s1 = self.source_1.get_3d(x, y, z);
        let s2 = self.source_2.get_3d(x, y, z);
        self.get(&s1, &s2)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let s1 = self.source_1.get_4d(x, y, z, w);
        let s2 = self.source_2.get_4d(x, y, z, w);
        self.get(&s1, &s2)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let s1 = self.source_1.get_6d(x, y, z, w, u, v);
        let s2 = self.source_2.get_6d(x, y, z, w, u, v);
        self.get(&s1, &s2)
    }
}

fn multiply(s1: &Rgba, s2: &Rgba) -> Rgba {
    Rgba::with_all(s1[0] * s2[0], s1[1] * s2[1], s1[2] * s2[2], s1[3])
}
fn add(s1: &Rgba, s2: &Rgba) -> Rgba {
    Rgba::with_all(s1[0] + s2[0], s1[1] + s2[1], s1[2] + s2[2], s1[3])
}
fn screen(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = s1[0] + s2[0] - s1[0] * s2[0];
    let g = s1[1] + s2[1] - s1[1] * s2[1];
    let b = s1[2] + s2[2] - s1[2] * s2[2];
    Rgba::with_all(r, g, b, s1[3])
}
fn overlay(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if s2[0] < 0.5 {
        (2.0 * s1[0] * s2[0])
    } else {
        (1.0 - 2.0 * (1.0 - s1[0]) * (1.0 - s2[0]))
    };
    let g = if s2[1] < 0.5 {
        (2.0 * s1[1] * s2[1])
    } else {
        (1.0 - 2.0 * (1.0 - s1[1]) * (1.0 - s2[1]))
    };
    let b = if s2[2] < 0.5 {
        (2.0 * s1[2] * s2[2])
    } else {
        (1.0 - 2.0 * (1.0 - s1[2]) * (1.0 - s2[2]))
    };
    let a = s1[3];
    Rgba::with_all(r, g, b, a)
}

fn soft_light(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if s1[0] > 0.5 {
        s2[0] + (1.0 - s2[0]) * ((s1[0] - 0.5) / 0.5) * (0.5 - (s2[0] - 0.5).abs())
    } else {
        s2[0] - s2[0] * ((0.5 - s1[0]) / 0.5) * (0.5 - (s2[0] - 0.5).abs())
    };

    let g = if s1[1] > 0.5 {
        s2[1] + (1.0 - s2[1]) * ((s1[1] - 0.5) / 0.5) * (0.5 - (s2[1] - 0.5).abs())
    } else {
        s2[1] - s2[1] * ((0.5 - s1[1]) / 0.5) * (0.5 - (s2[1] - 0.5).abs())
    };

    let b = if s1[2] > 0.5 {
        s2[2] + (1.0 - s2[2]) * ((s1[2] - 0.5) / 0.5) * (0.5 - (s2[2] - 0.5).abs())
    } else {
        s2[2] - s2[2] * ((0.5 - s1[2]) / 0.5) * (0.5 - (s2[2] - 0.5).abs())
    };

    Rgba::with_all(r, g, b, s1[3])
}

fn hard_light(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if s1[0] > 0.5 {
        s2[0] + (1.0 - s2[0]) * ((s1[0] - 0.5) / 0.5)
    } else {
        s2[0] * s1[0] / 0.5
    };

    let g = if s1[1] > 0.5 {
        s2[1] + (1.0 - s2[1]) * ((s1[1] - 0.5) / 0.5)
    } else {
        s2[1] * s1[1] / 0.5
    };

    let b = if s1[2] > 0.5 {
        s2[2] + (1.0 - s2[2]) * ((s1[2] - 0.5) / 0.5)
    } else {
        s2[2] * s1[2] / 0.5
    };

    Rgba::with_all(r, g, b, s1[3])
}

fn dodge(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if s1[0] == 1.0 {
        s1[0]
    } else {
        *min(&1.0, &((s2[0]) / (1.0 - s1[0])))
    };
    let g = if s1[1] == 1.0 {
        s1[1]
    } else {
        *min(&1.0, &((s2[1]) / (1.0 - s1[1])))
    };
    let b = if s1[2] == 1.0 {
        s1[2]
    } else {
        *min(&1.0, &((s2[2]) / (1.0 - s1[2])))
    };

    Rgba::with_all(r, g, b, s1[3])
}

fn burn(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if s1[0] == 1.0 {
        s1[0]
    } else {
        *max(&0.0, &(1.0 - ((1.0 - s2[0])) / s1[0]))
    };
    let g = if s1[1] == 1.0 {
        s1[1]
    } else {
        *max(&0.0, &(1.0 - ((1.0 - s2[1])) / s1[1]))
    };
    let b = if s1[2] == 1.0 {
        s1[2]
    } else {
        *max(&0.0, &(1.0 - ((1.0 - s2[2])) / s1[2]))
    };

    Rgba::with_all(r, g, b, s1[3])
}

fn linear_dodge(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = *min(&(s1[0] + s2[0]), &1.0);
    let g = *min(&(s1[1] + s2[1]), &1.0);
    let b = *min(&(s1[2] + s2[2]), &1.0);

    Rgba::with_all(r, g, b, s1[3])
}

fn linear_burn(s1: &Rgba, s2: &Rgba) -> Rgba {
    let r = if (s1[0] + s2[0]) < 1.0 {
        0.0
    } else {
        (s1[0] + s2[0] - 1.0)
    };
    let g = if (s1[1] + s2[1]) < 1.0 {
        0.0
    } else {
        (s1[1] + s2[1] - 1.0)
    };
    let b = if (s1[2] + s2[2]) < 1.0 {
        0.0
    } else {
        (s1[2] + s2[2] - 1.0)
    };

    Rgba::with_all(r, g, b, s1[3])
}
