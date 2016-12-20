use super::implicit_base::{ImplicitModuleBase, MAX_SOURCES};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub enum CombinerType {
    Add,
    Mul,
    Max,
    Min,
    Avg,
}

pub struct ImplicitCombiner {
    base: ImplicitModuleBase,
    sources: [Option<Rc<RefCell<ImplicitModule>>>; MAX_SOURCES],
    ctype: CombinerType,
}

impl ImplicitCombiner {
    pub fn with_type(ctype: CombinerType) -> ImplicitCombiner {
        let mut c = ImplicitCombiner {
            base: Default::default(),
            sources: unsafe { ::std::mem::uninitialized() },
            ctype: ctype,
        };

        for s in c.sources.iter_mut() {
            unsafe {
                ::std::ptr::write(s, None);
            }
        }

        c
    }

    pub fn set_type(&mut self, ctype: CombinerType) {
        self.ctype = ctype;
    }

    pub fn clear_all_sources(&mut self) {
        for s in self.sources.iter_mut() {
            *s = None;
        }
    }

    pub fn set_source(&mut self, which: usize, source: Option<Rc<RefCell<ImplicitModule>>>) {
        if which < MAX_SOURCES {
            self.sources[which] = source;
        }
    }

    fn add_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += s.borrow_mut().get_2d(x, y);
            }
        }
        value
    }

    fn add_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += s.borrow_mut().get_3d(x, y, z);
            }
        }
        value
    }

    fn add_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += s.borrow_mut().get_4d(x, y, z, w);
            }
        }
        value
    }

    fn add_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += s.borrow_mut().get_6d(x, y, z, w, u, v);
            }
        }
        value
    }

    fn mul_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut value = 1.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value *= s.borrow_mut().get_2d(x, y);
            }
        }
        value
    }

    fn mul_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 1.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value *= s.borrow_mut().get_3d(x, y, z);
            }
        }
        value
    }

    fn mul_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut value = 1.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value *= s.borrow_mut().get_4d(x, y, z, w);
            }
        }
        value
    }

    fn mul_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut value = 1.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value *= s.borrow_mut().get_6d(x, y, z, w, u, v);
            }
        }
        value
    }

    fn min_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let ref s = self.sources[c].as_ref().unwrap();
            let mut b = s.borrow_mut();
            b.get_2d(x, y)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_2d(x, y)
                };
                if v < mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn min_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_3d(x, y, z)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_3d(x, y, z)
                };
                if v < mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn min_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_4d(x, y, z, w)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_4d(x, y, z, w)
                };
                if v < mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn min_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_6d(x, y, z, w, u, v)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_6d(x, y, z, w, u, v)
                };
                if v < mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn max_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_2d(x, y)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_2d(x, y)
                };
                if v > mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn max_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_3d(x, y, z)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_3d(x, y, z)
                };
                if v > mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn max_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_4d(x, y, z, w)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_4d(x, y, z, w)
                };
                if v > mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn max_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut c = 0;

        for s in self.sources.iter() {
            if s.is_none() {
                c += 1;
            } else {
                break;
            }
        }

        if c == MAX_SOURCES {
            return 0.0;
        }

        let mut mn = {
            let mut b = self.sources[c].as_ref().unwrap().borrow_mut();
            b.get_6d(x, y, z, w, u, v)
        };

        for d in c..MAX_SOURCES {
            if let Some(ref s) = self.sources[d] {
                let v = {
                    let mut b = s.borrow_mut();
                    b.get_6d(x, y, z, w, u, v)
                };
                if v > mn {
                    mn = v
                }
            }
        }

        return mn;
    }

    fn avg_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut count = 0.0;
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += {
                    let mut b = s.borrow_mut();
                    b.get_2d(x, y)
                };
                count += 1.0
            }
        }
        if count == 0.0 { 0.0 } else { value / count }
    }

    fn avg_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut count = 0.0;
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += {
                    let mut b = s.borrow_mut();
                    b.get_3d(x, y, z)
                };
                count += 1.0
            }
        }
        if count == 0.0 { 0.0 } else { value / count }
    }

    fn avg_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut count = 0.0;
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += {
                    let mut b = s.borrow_mut();
                    b.get_4d(x, y, z, w)
                };
                count += 1.0
            }
        }
        if count == 0.0 { 0.0 } else { value / count }
    }

    fn avg_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut count = 0.0;
        let mut value = 0.0;
        for s in self.sources.iter() {
            if let &Some(ref s) = s {
                value += {
                    let mut b = s.borrow_mut();
                    b.get_6d(x, y, z, w, u, v)
                };
                count += 1.0
            }
        }
        if count == 0.0 { 0.0 } else { value / count }
    }
}

impl ImplicitModule for ImplicitCombiner {
    fn set_seed(&mut self, _: u32) {}

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        match self.ctype {
            CombinerType::Add => self.add_get_2d(x, y),
            CombinerType::Mul => self.mul_get_2d(x, y),
            CombinerType::Max => self.max_get_2d(x, y),
            CombinerType::Min => self.min_get_2d(x, y),
            CombinerType::Avg => self.avg_get_2d(x, y),
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        match self.ctype {
            CombinerType::Add => self.add_get_3d(x, y, z),
            CombinerType::Mul => self.mul_get_3d(x, y, z),
            CombinerType::Max => self.max_get_3d(x, y, z),
            CombinerType::Min => self.min_get_3d(x, y, z),
            CombinerType::Avg => self.avg_get_3d(x, y, z),
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        match self.ctype {
            CombinerType::Add => self.add_get_4d(x, y, z, w),
            CombinerType::Mul => self.mul_get_4d(x, y, z, w),
            CombinerType::Max => self.max_get_4d(x, y, z, w),
            CombinerType::Min => self.min_get_4d(x, y, z, w),
            CombinerType::Avg => self.avg_get_4d(x, y, z, w),
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        match self.ctype {
            CombinerType::Add => self.add_get_6d(x, y, z, w, u, v),
            CombinerType::Mul => self.mul_get_6d(x, y, z, w, u, v),
            CombinerType::Max => self.max_get_6d(x, y, z, w, u, v),
            CombinerType::Min => self.min_get_6d(x, y, z, w, u, v),
            CombinerType::Avg => self.avg_get_6d(x, y, z, w, u, v),
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
}
