use super::implicit_base::ImplicitModuleBase;
use super::{ImplicitModule, ScalarParameter};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ImplicitTriangle {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    period: ScalarParameter,
    offset: ScalarParameter,
}

impl ImplicitTriangle {
    pub fn with_period_offset(period: f64, offset: f64) -> ImplicitTriangle {
        ImplicitTriangle {
            base: Default::default(),
            source: ScalarParameter::Value(0.0),
            period: ScalarParameter::Value(period),
            offset: ScalarParameter::Value(offset),
        }
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }

    pub fn set_period_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.period = ScalarParameter::Source(m);
    }
    pub fn set_period_value(&mut self, v: f64) {
        self.period = ScalarParameter::Value(v);
    }

    pub fn set_offset_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.offset = ScalarParameter::Source(m);
    }
    pub fn set_offset_value(&mut self, v: f64) {
        self.offset = ScalarParameter::Value(v);
    }
}

impl ImplicitModule for ImplicitTriangle {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let val = self.source.get_2d(x, y);
        let period = self.period.get_2d(x, y);
        let offset = self.offset.get_2d(x, y);

        if offset >= 1.0 {
            sawtooth(val, period)
        } else if offset <= 0.0 {
            1.0 - sawtooth(val, period)
        } else {
            let s1 = if (offset - sawtooth(val, period)) >= 0.0 {
                1.0
            } else {
                0.0
            };
            let s2 = if ((1.0 - offset) - (sawtooth(-val, period))) >= 0.0 {
                1.0
            } else {
                0.0
            };
            sawtooth(val, period) * s1 / offset + sawtooth(-val, period) * s2 / (1.0 - offset)
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let val = self.source.get_3d(x, y, z);
        let period = self.period.get_3d(x, y, z);
        let offset = self.offset.get_3d(x, y, z);

        if offset >= 1.0 {
            sawtooth(val, period)
        } else if offset <= 0.0 {
            1.0 - sawtooth(val, period)
        } else {
            let s1 = if (offset - sawtooth(val, period)) >= 0.0 {
                1.0
            } else {
                0.0
            };
            let s2 = if ((1.0 - offset) - (sawtooth(-val, period))) >= 0.0 {
                1.0
            } else {
                0.0
            };
            sawtooth(val, period) * s1 / offset + sawtooth(-val, period) * s2 / (1.0 - offset)
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let val = self.source.get_4d(x, y, z, w);
        let period = self.period.get_4d(x, y, z, w);
        let offset = self.offset.get_4d(x, y, z, w);

        if offset >= 1.0 {
            sawtooth(val, period)
        } else if offset <= 0.0 {
            1.0 - sawtooth(val, period)
        } else {
            let s1 = if (offset - sawtooth(val, period)) >= 0.0 {
                1.0
            } else {
                0.0
            };
            let s2 = if ((1.0 - offset) - (sawtooth(-val, period))) >= 0.0 {
                1.0
            } else {
                0.0
            };
            sawtooth(val, period) * s1 / offset + sawtooth(-val, period) * s2 / (1.0 - offset)
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let val = self.source.get_6d(x, y, z, w, u, v);
        let period = self.period.get_6d(x, y, z, w, u, v);
        let offset = self.offset.get_6d(x, y, z, w, u, v);

        if offset >= 1.0 {
            sawtooth(val, period)
        } else if offset <= 0.0 {
            1.0 - sawtooth(val, period)
        } else {
            let s1 = if (offset - sawtooth(val, period)) >= 0.0 {
                1.0
            } else {
                0.0
            };
            let s2 = if ((1.0 - offset) - (sawtooth(-val, period))) >= 0.0 {
                1.0
            } else {
                0.0
            };
            sawtooth(val, period) * s1 / offset + sawtooth(-val, period) * s2 / (1.0 - offset)
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}

#[inline(always)]
fn sawtooth(x: f64, p: f64) -> f64 {
    (2.0 * (x / p - (0.5 + x / p).floor())) * 0.5 + 0.5
}
