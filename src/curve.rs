use std::ops::{Add, Mul, Sub};

struct ControlPoint<T>
    where T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Mul<f64, Output = T>
{
    t: f64,
    value: T,
}

pub struct Curve<T>
    where T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Mul<f64, Output = T>
{
    points: Vec<ControlPoint<T>>,
}

impl<T> Curve<T>
    where T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Mul<f64, Output = T>
{
    pub fn new() -> Curve<T> {
        Curve { points: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Curve<T> {
        Curve { points: Vec::with_capacity(capacity) }
    }

    pub fn push_point(&mut self, t: f64, point: T) {
        let i = self.find_control_point(t);
        self.points.insert(i,
                           ControlPoint {
                               t: t,
                               value: point,
                           });
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }

    pub fn no_interp(&self, t: f64) -> T {
        if self.points.len() < 2 {
            return Default::default();
        }

        if t <= self.points[0].t {
            return self.points[0].value;
        }
        if t >= self.points[self.points.len() - 1].t {
            return self.points[self.points.len() - 1].value;
        }

        let iter = self.find_control_point(t);
        if iter == self.points.len() - 1 {
            // Something weird happened
            return Default::default();
        }

        let prev_iter = iter - 1;
        if prev_iter == self.points.len() - 1 {
            // Again, something weird happened
            return Default::default();
        }
        self.points[prev_iter].value
    }

    pub fn linear_interp(&self, t: f64) -> T {
        if self.points.len() < 2 {
            return Default::default();
        }
        if t <= self.points[0].t {
            return self.points[0].value;
        }
        if t >= self.points[self.points.len() - 1].t {
            return self.points[self.points.len() - 1].value;
        }

        let iter = self.find_control_point(t);
        if iter == self.points.len() - 1 {
            // Something weird happened
            return Default::default();
        }

        let prev_iter = iter - 1;
        if prev_iter == self.points.len() - 1 {
            // Again, something weird happened
            return Default::default();
        }

        let t0 = self.points[prev_iter].t;
        let t1 = self.points[iter].t;
        let interp = (t - t0) / (t1 - t0);

        let v0 = self.points[prev_iter].value;
        let v1 = self.points[iter].value;
        v0 + (v1 - v0) * interp
    }

    pub fn cubic_interp(&self, t: f64) -> T {
        if self.points.len() < 2 {
            return Default::default();
        }
        if t <= self.points[0].t {
            return self.points[0].value;
        }
        if t >= self.points[self.points.len() - 1].t {
            return self.points[self.points.len() - 1].value;
        }

        let iter = self.find_control_point(t);
        if iter == self.points.len() - 1 {
            // Something weird happened
            return Default::default();
        }

        let prev_iter = iter - 1;
        if prev_iter == self.points.len() - 1 {
            // Again, something weird happened
            return Default::default();
        }

        let t0 = self.points[prev_iter].t;
        let t1 = self.points[iter].t;
        let mut interp = (t - t0) / (t1 - t0);
        interp = interp * interp * (3.0 - 2.0 * interp);

        let v0 = self.points[prev_iter].value;
        let v1 = self.points[iter].value;
        v0 + (v1 - v0) * interp
    }

    pub fn quintic_interp(&self, t: f64) -> T {
        if self.points.len() < 2 {
            return Default::default();
        }
        if t <= self.points[0].t {
            return self.points[0].value;
        }
        if t >= self.points[self.points.len() - 1].t {
            return self.points[self.points.len() - 1].value;
        }

        let iter = self.find_control_point(t);
        if iter == self.points.len() - 1 {
            // Something weird happened
            return Default::default();
        }

        let prev_iter = iter - 1;
        if prev_iter == self.points.len() - 1 {
            // Again, something weird happened
            return Default::default();
        }

        let t0 = self.points[prev_iter].t;
        let t1 = self.points[iter].t;
        let mut interp = (t - t0) / (t1 - t0);
        interp = interp * interp * interp * (interp * (interp * 6.0 - 15.0) + 10.0);

        let v0 = self.points[prev_iter].value;
        let v1 = self.points[iter].value;
        v0 + (v1 - v0) * interp
    }

    fn find_control_point(&self, t: f64) -> usize {
        // Find the first control point where p.t > t
        for i in 0..self.points.len() {
            if t <= self.points[i].t {
                return i;
            }
        }

        0
    }
}
