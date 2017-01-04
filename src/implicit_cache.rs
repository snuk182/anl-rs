/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::{ScalarParameter, ImplicitModule};

struct Cache {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    u: f64,
    v: f64,
    val: f64,
    valid: bool,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            u: 0.0,
            v: 0.0,
            val: 0.0,
            valid: false,
        }
    }
}

// 
pub struct ImplicitCache {
    base: ImplicitModuleBase,
    source: ScalarParameter,
    c2: Cache,
    c3: Cache,
    c4: Cache,
    c6: Cache,
}

impl ImplicitCache {
    pub fn new(source: ScalarParameter) -> ImplicitCache {
        ImplicitCache {
            base: Default::default(),
            source: source,
            c2: Default::default(),
            c3: Default::default(),
            c4: Default::default(),
            c6: Default::default(),
        }
    }
}

impl ImplicitModule for ImplicitCache {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        if !self.c2.valid || self.c2.x != x || self.c2.y != y {
            self.c2.x = x;
            self.c2.y = y;
            self.c2.valid = true;
            self.c2.val = self.source.get_2d(x, y);
        }
        self.c2.val
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        if !self.c3.valid || self.c3.x != x || self.c3.y != y || self.c3.z != z {
            self.c3.x = x;
            self.c3.y = y;
            self.c3.z = z;
            self.c3.valid = true;
            self.c3.val = self.source.get_3d(x, y, z);
        }
        self.c3.val
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        if !self.c4.valid || self.c4.x != x || self.c4.y != y || self.c4.z != z || self.c4.w != w {
            self.c4.x = x;
            self.c4.y = y;
            self.c4.z = z;
            self.c4.w = w;
            self.c4.valid = true;
            self.c4.val = self.source.get_4d(x, y, z, w);
        }
        self.c4.val
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        if !self.c6.valid || self.c6.x != x || self.c6.y != y || self.c6.z != z || self.c6.w != w || self.c6.u != u || self.c6.v != v {
            self.c6.x = x;
            self.c6.y = y;
            self.c6.z = z;
            self.c6.w = w;
            self.c6.u = u;
            self.c6.v = v;
            self.c6.valid = true;
            self.c6.val = self.source.get_6d(x, y, z, w, u, v);
        }
        self.c6.val
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
