use super::noise_gen::{cellular_function_2d, cellular_function_3d, cellular_function_4d, cellular_function_6d};

pub struct CellularCache {
    pub f: [f64; 4],
    pub d: [f64; 4],
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    pub u: f64,
    pub v: f64,
    pub valid: bool,
}

impl CellularCache {
    fn new() -> CellularCache {
        CellularCache {
            f: [0.0; 4],
            d: [0.0; 4],
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            u: 0.0,
            v: 0.0,
            valid: false,
        }
    }
}

pub struct CellularGenerator {
    seed: u32,
    cache2: CellularCache,
    cache3: CellularCache,
    cache4: CellularCache,
    cache6: CellularCache,
}

impl CellularGenerator {
    pub fn new() -> CellularGenerator {
        CellularGenerator {
            seed: 0,
            cache2: CellularCache::new(),
            cache3: CellularCache::new(),
            cache4: CellularCache::new(),
            cache6: CellularCache::new(),
        }
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
        self.cache2.valid = false;
        self.cache3.valid = false;
        self.cache4.valid = false;
        self.cache6.valid = false;
    }

    pub fn get_2d<'a>(&'a mut self, x: f64, y: f64) -> &'a CellularCache {
        if !self.cache2.valid || x != self.cache2.x || y != self.cache2.y {
            cellular_function_2d(x, y, self.seed, &mut self.cache2.f, &mut self.cache2.d);
            self.cache2.x = x;
            self.cache2.y = y;
            self.cache2.valid = true;
        }
        &self.cache2
    }

    pub fn get_3d<'a>(&'a mut self, x: f64, y: f64, z: f64) -> &'a CellularCache {
        if !self.cache3.valid || x != self.cache3.x || y != self.cache3.y || z != self.cache3.z {
            cellular_function_3d(x, y, z, self.seed, &mut self.cache3.f, &mut self.cache3.d);
            self.cache3.x = x;
            self.cache3.y = y;
            self.cache3.z = z;
            self.cache3.valid = true;
        }
        &self.cache3
    }

    pub fn get_4d<'a>(&'a mut self, x: f64, y: f64, z: f64, w: f64) -> &'a CellularCache {
        if !self.cache4.valid || x != self.cache4.x || y != self.cache4.y || z != self.cache4.z || w != self.cache4.w {
            cellular_function_4d(x,
                                 y,
                                 z,
                                 w,
                                 self.seed,
                                 &mut self.cache4.f,
                                 &mut self.cache4.d);
            self.cache4.x = x;
            self.cache4.y = y;
            self.cache4.z = z;
            self.cache4.w = w;
            self.cache4.valid = true;
        }
        &self.cache4
    }

    pub fn get_6d<'a>(&'a mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> &'a CellularCache {
        if self.cache6.valid || x != self.cache6.x || y != self.cache6.y || z != self.cache6.z || w != self.cache6.w || u != self.cache6.u || v != self.cache6.v {
            cellular_function_6d(x,
                                 y,
                                 z,
                                 w,
                                 u,
                                 v,
                                 self.seed,
                                 &mut self.cache6.f,
                                 &mut self.cache6.d);
            self.cache6.x = x;
            self.cache6.y = y;
            self.cache6.z = z;
            self.cache6.w = w;
            self.cache6.u = u;
            self.cache6.v = v;
            self.cache6.valid = true;
        }

        &self.cache6
    }
}
