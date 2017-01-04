/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::*;
use super::ImplicitModule;
use super::implicit_basis_function::*;
use super::utility::{min, max};

use std::rc::Rc;
use std::cell::RefCell;

pub enum FractalType {
    FBM,
    RidgedMulti,
    Billow,
    Multi,
    HybridMulti,
    DecarpentierSwiss,
}

/// Fractals are a special type of combiner that combine up to 20 noise sources using fractal methods pioneered by Perlin, Musgrave, and friends. They come in various types (specifiable through [`set_type()`](struct.ImplicitFractal.html#set_type) or in the constructor). Each fractal has up to 20 built-in [`BasisFunction`](struct.ImplicitBasisFunction.html)s whose basistype and interptype can be set via the provided methods. Additionally, you can obtain a pointer to any source of the fractal via [`get_basis()`](struct.ImplicitFractal.html#get_basis). Any source module in the fractal may also be overridden by an external noise function via overrideSource(). The complexity of this system serves a purpose: "generic" fractals of a given type, with basis functions of all the same type, can easily be instance just by construction, yet more complex behavior can be produced by overriding layers with external sources, to build up very complex fractals, if so desired.
/// Fractals are highly parameterized. The two most useful parameters are `numoctaves` which determines how many layers contribute to the fractal, and `frequency` which specifies the density of the function. Frequency mimics combining a [`ScaleDomain`](struct.ImplicitScaleDomain.html) function to the source, multiplying the input coordinates by frequency before calculating the function. Other parameters that control the fractal are `offset`, `lacunarity`, `gain` and `H`. These parameters can have subtle, drastic, or no effect on the fractal, depending on the type, and they are typically best left alone. 
#[allow(non_snake_case)]
pub struct ImplicitFractal {
    base: ImplicitModuleBase,
    basis: [Rc<RefCell<ImplicitBasisFunction>>; MAX_SOURCES],
    source: [Option<Rc<RefCell<ImplicitModule>>>; MAX_SOURCES],
    exparray: [f64; MAX_SOURCES],
    correct: [[f64; 2]; MAX_SOURCES],
    offset: f64,
    gain: f64,
    H: f64,
    frequency: f64,
    lacunarity: f64,
    numoctaves: u32,
    ftype: FractalType,
}

impl Default for ImplicitFractal {
    fn default() -> Self {
        let mut f = ImplicitFractal {
            base: Default::default(),
            basis: unsafe { ::std::mem::uninitialized() },
            source: unsafe { ::std::mem::uninitialized() },
            exparray: [0.0; MAX_SOURCES],
            correct: [[0.0; 2]; MAX_SOURCES],
            offset: 0.0,
            gain: 0.0,
            H: 0.0,
            frequency: 0.0,
            lacunarity: 0.0,
            numoctaves: 0,
            ftype: FractalType::FBM,
        };

        let imp = Rc::new(RefCell::new(ImplicitBasisFunction::default()));

        for s in f.basis.iter_mut() {
            unsafe { ::std::ptr::write(s, imp.clone()) }
        }

        for s in f.source.iter_mut() {
            unsafe {
                ::std::ptr::write(s, None);
            }
        }

        f
    }
}

#[allow(unused_assignments, non_snake_case)]
impl ImplicitFractal {
    pub fn set_num_octaves(&mut self, n: u32) {
        self.numoctaves = if n >= MAX_SOURCES as u32 {
            (MAX_SOURCES - 1) as u32
        } else {
            n
        }
    }

    pub fn set_frequency(&mut self, f: f64) {
        self.frequency = f;
    }

    pub fn set_lacunarity(&mut self, l: f64) {
        self.lacunarity = l
    }

    pub fn set_gain(&mut self, g: f64) {
        self.gain = g;
    }

    pub fn set_offset(&mut self, o: f64) {
        self.offset = o;
    }

    pub fn set_h(&mut self, h: f64) {
        self.H = h
    }

    pub fn new(ftype: FractalType, basistype: BasisType, interptype: InterpType) -> ImplicitFractal {
        let mut v = ImplicitFractal {
            numoctaves: 8,
            frequency: 1.0,
            lacunarity: 2.0,
            ..Default::default()
        };

        v.set_all_source_types(basistype, interptype);
        v.reset_all_sources();
        v.set_type(ftype);

        return v;
    }

    pub fn set_type(&mut self, t: FractalType) {
        self.ftype = t;
        match self.ftype {
            FractalType::FBM => {
                self.H = 1.0;
                self.gain = 0.5;
                self.offset = 0.0;
                self.fbm_calc_weights();
            }
            FractalType::RidgedMulti => {
                self.H = 0.9;
                self.gain = 0.5;
                self.offset = 1.0;
                self.ridged_multi_calc_weights();
            }
            FractalType::Billow => {
                self.H = 1.0;
                self.gain = 0.5;
                self.offset = 0.0;
                self.billow_calc_weights();
            }
            FractalType::Multi => {
                self.H = 1.0;
                self.offset = 0.0;
                self.gain = 0.0;
                self.multi_calc_weights();
            }
            FractalType::HybridMulti => {
                self.H = 0.25;
                self.gain = 1.0;
                self.offset = 0.7;
                self.hybrid_multi_calc_weights();
            }
            FractalType::DecarpentierSwiss => {
                self.H = 0.9;
                self.gain = 0.6;
                self.offset = 0.15;
                self.decarpentier_swiss_calc_weights();
            }
        }
    }

    pub fn set_all_source_types(&mut self, basis_type: BasisType, interp: InterpType) {
        for i in self.basis.iter_mut() {
            let mut bi = i.borrow_mut();
            bi.set_type(basis_type.clone());
            bi.set_interp(interp.clone());
        }
    }

    pub fn set_source_type(&mut self, which: usize, basis_type: BasisType, interp: InterpType) {
        if which < MAX_SOURCES {
            let mut bw = self.basis[which].borrow_mut();
            bw.set_type(basis_type);
            bw.set_interp(interp);
        }
    }

    pub fn override_source(&mut self, which: usize, b: Option<Rc<RefCell<ImplicitModule>>>) {
        if which < MAX_SOURCES {
            self.source[which] = b;
        }
    }

    pub fn reset_source(&mut self, which: usize) {
        if which < MAX_SOURCES {
            self.source[which] = Some(self.basis[which].clone())
        }
    }

    pub fn reset_all_sources(&mut self) {
        for c in 0..MAX_SOURCES {
            self.source[c] = Some(self.basis[c].clone());
        }
    }

    pub fn get_basis(&mut self, which: usize) -> Option<Rc<RefCell<ImplicitBasisFunction>>> {
        if which < MAX_SOURCES {
            Some(self.basis[which].clone())
        } else {
            None
        }
    }

    fn fbm_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let mut minvalue = 0.0;
        let mut maxvalue = 0.0;
        for i in 0..MAX_SOURCES {
            minvalue += -1.0 * self.exparray[i];
            maxvalue += 1.0 * self.exparray[i];

            let A = -1.0;
            let B = 1.0;
            let scale = (B - A) / (maxvalue - minvalue);
            let bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }
    }

    fn ridged_multi_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let mut minvalue = 0.0;
        let mut maxvalue = 0.0;
        for i in 0..MAX_SOURCES {
            minvalue += (self.offset - 1.0) * (self.offset - 1.0) * self.exparray[i];
            maxvalue += (self.offset) * (self.offset) * self.exparray[i];

            let A = -1.0;
            let B = 1.0;
            let scale = (B - A) / (maxvalue - minvalue);
            let bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }
    }

    pub fn decarpentier_swiss_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let mut minvalue = 0.0;
        let mut maxvalue = 0.0;
        for i in 0..MAX_SOURCES {
            minvalue += (self.offset - 1.0) * (self.offset - 1.0) * self.exparray[i];
            maxvalue += (self.offset) * (self.offset) * self.exparray[i];

            let A = -1.0;
            let B = 1.0;
            let scale = (B - A) / (maxvalue - minvalue);
            let bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }
    }

    pub fn billow_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let mut minvalue = 0.0;
        let mut maxvalue = 0.0;
        for i in 0..MAX_SOURCES {
            minvalue += -1.0 * self.exparray[i];
            maxvalue += 1.0 * self.exparray[i];

            let A = -1.0;
            let B = 1.0;
            let scale = (B - A) / (maxvalue - minvalue);
            let bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }
    }

    pub fn multi_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let mut minvalue = 1.0;
        let mut maxvalue = 1.0;
        for i in 0..MAX_SOURCES {
            minvalue *= -1.0 * self.exparray[i] + 1.0;
            maxvalue *= 1.0 * self.exparray[i] + 1.0;

            let A = -1.0;
            let B = 1.0;
            let scale = (B - A) / (maxvalue - minvalue);
            let bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }

    }

    pub fn hybrid_multi_calc_weights(&mut self) {
        for i in 0..MAX_SOURCES {
            self.exparray[i] = self.lacunarity.powf(-(i as f64) * self.H);
        }

        // Calculate scale/bias pairs by guessing at minimum and maximum values and remapping to [-1,1]
        let A = -1.0;
        let B = 1.0;

        let mut minvalue = self.offset - 1.0;
        let mut maxvalue = self.offset + 1.0;
        let mut weightmin = self.gain * minvalue;
        let mut weightmax = self.gain * maxvalue;

        let mut scale = (B - A) / (maxvalue - minvalue);
        let mut bias = A - minvalue * scale;
        self.correct[0][0] = scale;
        self.correct[0][1] = bias;

        for i in 1..MAX_SOURCES {
            if weightmin > 1.0 {
                weightmin = 1.0
            }
            if weightmax > 1.0 {
                weightmax = 1.0
            }

            let mut signal = (self.offset - 1.0) * self.exparray[i];
            minvalue += signal * weightmin;
            weightmin *= self.gain * signal;

            signal = (self.offset + 1.0) * self.exparray[i];
            maxvalue += signal * weightmax;
            weightmax *= self.gain * signal;

            scale = (B - A) / (maxvalue - minvalue);
            bias = A - minvalue * scale;
            self.correct[i][0] = scale;
            self.correct[i][1] = bias;
        }
    }

    pub fn fbm_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;;
        y *= self.frequency;;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_2d(x, y);
                sum += n * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
            }
        }
        sum
    }

    pub fn fbm_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;;
        y *= self.frequency;;
        z *= self.frequency;;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_3d(x, y, z);
                sum += n * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
            }
        }
        sum
    }

    pub fn fbm_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;;
        y *= self.frequency;;
        z *= self.frequency;;
        w *= self.frequency;;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_4d(x, y, z, w);
                sum += n * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
            }
        }
        sum
    }

    pub fn fbm_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;;
        y *= self.frequency;;
        z *= self.frequency;;
        w *= self.frequency;;
        u *= self.frequency;;
        v *= self.frequency;;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                // let n = self.source[i].get_4d(x,y,z,w);
                let n = ii.borrow_mut().get_6d(x, y, z, w, u, v);
                sum += n * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
                u *= self.lacunarity;
                v *= self.lacunarity;
            }
        }
        sum
    }

    pub fn multi_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut value = 1.0;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;
        y *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                value *= ii.borrow_mut().get_2d(x, y) * self.exparray[i] + 1.0;
                x *= self.lacunarity;
                y *= self.lacunarity;
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn multi_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut value = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                value *= ii.borrow_mut().get_4d(x, y, z, w) * self.exparray[i] + 1.0;
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn multi_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                value *= ii.borrow_mut().get_3d(x, y, z) * self.exparray[i] + 1.0;
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
            }
        }

        return value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1];
    }

    pub fn multi_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut value = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;
        u *= self.frequency;
        v *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                value *= ii.borrow_mut().get_6d(x, y, z, w, u, v) * self.exparray[i] + 1.0;
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
                u *= self.lacunarity;
                v *= self.lacunarity;
            }
        }

        return value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1];
    }

    pub fn billow_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;
        y *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_2d(x, y);
                sum += (2.0 * n.abs() - 1.0) * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
            }
        }
        sum
    }

    pub fn billow_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_4d(x, y, z, w);
                sum += (2.0 * n.abs() - 1.0) * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
            }
        }
        sum
    }

    pub fn billow_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_3d(x, y, z);
                sum += (2.0 * n.abs() - 1.0) * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
            }
        }
        sum
    }

    pub fn billow_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;
        u *= self.frequency;
        v *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let n = ii.borrow_mut().get_6d(x, y, z, w, u, v);
                sum += (2.0 * n.abs() - 1.0) * amp;
                amp *= self.gain;

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
                u *= self.lacunarity;
                v *= self.lacunarity;
            }
        }
        sum
    }

    pub fn ridged_multi_get_2d(&mut self, x: f64, y: f64) -> f64 {
        // let mut sum = 0.0;
        // let mut amp = 1.0;
        //
        // x *= self.frequency;
        // y *= self.frequency;
        //
        // for i in 0..self.numoctaves as usize {
        // if let Some(ref ii) = self.source[i] {
        // let n = ii.borrow_mut().get_2d(x, y);
        // n = 1.0 - n.abs();
        // sum += amp * n;
        // amp *= self.gain;
        //
        // x *= self.lacunarity;
        // y *= self.lacunarity;
        // }
        // }
        // sum
        let mut result = 0.0;
        let mut signal = 0.0;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;
        y *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                signal = ii.borrow_mut().get_2d(x, y);
                signal = self.offset - signal.abs();
                signal *= signal;
                result += signal * self.exparray[i];

                x *= self.lacunarity;
                y *= self.lacunarity;
            }
        }

        result * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn ridged_multi_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut result = 0.0;
        let mut signal = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                signal = ii.borrow_mut().get_4d(x, y, z, w);
                signal = self.offset - signal.abs();
                signal *= signal;
                result += signal * self.exparray[i];

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
            }
        }

        result * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn ridged_multi_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        // let mut sum = 0.0;
        // let mut amp = 1.0;
        //
        // x *= self.frequency;
        // y *= self.frequency;
        // z *= self.frequency;
        //
        // for i in 0..self.numoctaves as usize {
        // if let Some(ref ii) = self.source[i] {
        // let n = ii.borrow_mut().get_3d(x, y, z);
        // n = 1.0 - n.abs();
        // sum += amp * n;
        // amp *= self.gain;
        //
        // x *= self.lacunarity;
        // y *= self.lacunarity;
        // z *= self.lacunarity;
        // }
        // }
        // sum

        let mut result = 0.0;
        let mut signal = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                signal = ii.borrow_mut().get_3d(x, y, z);
                signal = self.offset - signal.abs();
                signal *= signal;
                result += signal * self.exparray[i];

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
            }
        }

        result * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn ridged_multi_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut result = 0.0;
        let mut signal = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;
        u *= self.frequency;
        v *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                signal = ii.borrow_mut().get_6d(x, y, z, w, u, v);
                signal = self.offset - signal.abs();
                signal *= signal;
                result += signal * self.exparray[i];

                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
                u *= self.lacunarity;
                v *= self.lacunarity;
            }
        }

        return result * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1];
    }

    pub fn hybrid_multi_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut value: f64 = 1.0;
        let mut signal: f64;
        let mut weight: f64;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;
        y *= self.frequency;

        if let Some(ref oo) = self.source[0] {
            value = oo.borrow_mut().get_2d(x, y) + self.offset;
            weight = self.gain * value;
            x *= self.lacunarity;
            y *= self.lacunarity;

            for i in 1..self.numoctaves as usize {
                if weight > 1.0 {
                    weight = 1.0
                }
                if let Some(ref ii) = self.source[i] {
                    signal = (ii.borrow_mut().get_2d(x, y) + self.offset) * self.exparray[i];
                    value += weight * signal;
                    weight *= self.gain * signal;
                    x *= self.lacunarity;
                    y *= self.lacunarity;
                }
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn hybrid_multi_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut value: f64 = 1.0;
        let mut signal: f64;
        let mut weight: f64;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        if let Some(ref oo) = self.source[0] {
            value = oo.borrow_mut().get_2d(x, y) + self.offset;
            weight = self.gain * value;
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;

            for i in 1..self.numoctaves as usize {
                if weight > 1.0 {
                    weight = 1.0
                }
                if let Some(ref ii) = self.source[i] {
                    signal = (ii.borrow_mut().get_3d(x, y, z) + self.offset) * self.exparray[i];
                    value += weight * signal;
                    weight *= self.gain * signal;
                    x *= self.lacunarity;
                    y *= self.lacunarity;
                    z *= self.lacunarity;
                }
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn hybrid_multi_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut value: f64 = 1.0;
        let mut signal: f64;
        let mut weight: f64;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;

        if let Some(ref oo) = self.source[0] {
            value = oo.borrow_mut().get_2d(x, y) + self.offset;
            weight = self.gain * value;
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
            w *= self.lacunarity;

            for i in 1..self.numoctaves as usize {
                if weight > 1.0 {
                    weight = 1.0
                }
                if let Some(ref ii) = self.source[i] {
                    signal = (ii.borrow_mut().get_4d(x, y, z, w) + self.offset) * self.exparray[i];
                    value += weight * signal;
                    weight *= self.gain * signal;
                    x *= self.lacunarity;
                    y *= self.lacunarity;
                    z *= self.lacunarity;
                    w *= self.lacunarity;
                }
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn hybrid_multi_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut value: f64 = 1.0;
        let mut signal: f64;
        let mut weight: f64;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;
        u *= self.frequency;
        v *= self.frequency;

        if let Some(ref oo) = self.source[0] {
            value = oo.borrow_mut().get_2d(x, y) + self.offset;
            weight = self.gain * value;
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
            w *= self.lacunarity;
            u *= self.lacunarity;
            v *= self.lacunarity;

            for i in 1..self.numoctaves as usize {
                if weight > 1.0 {
                    weight = 1.0
                }
                if let Some(ref ii) = self.source[i] {
                    signal = (ii.borrow_mut().get_6d(x, y, z, w, u, v) + self.offset) * self.exparray[i];
                    value += weight * signal;
                    weight *= self.gain * signal;
                    x *= self.lacunarity;
                    y *= self.lacunarity;
                    z *= self.lacunarity;
                    w *= self.lacunarity;
                    u *= self.lacunarity;
                    v *= self.lacunarity;
                }
            }
        }

        value * self.correct[self.numoctaves as usize - 1][0] + self.correct[self.numoctaves as usize - 1][1]
    }

    pub fn decarpentier_swiss_get_2d(&mut self, x: f64, y: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut dx_sum = 0.0;
        let mut dy_sum = 0.0;

        let mut x = x;
        let mut y = y;

        x *= self.frequency;
        y *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let mut bii = ii.borrow_mut();
                let n = bii.get_2d(x + self.offset * dx_sum, y + self.offset * dy_sum);
                let dx = bii.get_dx_2(x + self.offset * dx_sum, y + self.offset * dy_sum);
                let dy = bii.get_dy_2(x + self.offset * dx_sum, y + self.offset * dy_sum);
                sum += amp * (1.0 - n.abs());
                dx_sum += amp * dx * -n;
                dy_sum += amp * dy * -n;
                amp *= self.gain * *(max(&0.0, min(&sum, &1.0)));
                x *= self.lacunarity;
                y *= self.lacunarity;
            }
        }
        sum
    }

    pub fn decarpentier_swiss_get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut dx_sum = 0.0;
        let mut dy_sum = 0.0;
        let mut dz_sum = 0.0;
        let mut dw_sum = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let mut bii = ii.borrow_mut();
                let n = bii.get_4d(x + self.offset * dx_sum,
                                   y + self.offset * dy_sum,
                                   z + self.offset * dz_sum,
                                   w + self.offset * dw_sum);
                let dx = bii.get_dx_4(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum);
                let dy = bii.get_dy_4(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum);
                let dz = bii.get_dz_4(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum);
                let dw = bii.get_dw_4(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum);
                sum += amp * (1.0 - n.abs());
                dx_sum += amp * dx * -n;
                dy_sum += amp * dy * -n;
                dz_sum += amp * dz * -n;
                dw_sum += amp * dw * -n;
                amp *= self.gain * *(max(&0.0, min(&sum, &1.0)));
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
            }
        }
        sum
    }

    pub fn decarpentier_swiss_get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut dx_sum = 0.0;
        let mut dy_sum = 0.0;
        let mut dz_sum = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let mut bii = ii.borrow_mut();
                let n = bii.get_3d(x + self.offset * dx_sum,
                                   y + self.offset * dy_sum,
                                   z + self.offset * dz_sum);
                let dx = bii.get_dx_3(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum);
                let dy = bii.get_dy_3(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum);
                let dz = bii.get_dz_3(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum);
                sum += amp * (1.0 - n.abs());
                dx_sum += amp * dx * -n;
                dy_sum += amp * dy * -n;
                dz_sum += amp * dz * -n;
                amp *= self.gain * *(max(&0.0, min(&sum, &1.0)));
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
            }
        }
        return sum;
    }

    pub fn decarpentier_swiss_get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut dx_sum = 0.0;
        let mut dy_sum = 0.0;
        let mut dz_sum = 0.0;
        let mut dw_sum = 0.0;
        let mut du_sum = 0.0;
        let mut dv_sum = 0.0;

        let mut x = x;
        let mut y = y;
        let mut z = z;
        let mut w = w;
        let mut u = u;
        let mut v = v;

        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;
        w *= self.frequency;
        u *= self.frequency;
        v *= self.frequency;

        for i in 0..self.numoctaves as usize {
            if let Some(ref ii) = self.source[i] {
                let mut bii = ii.borrow_mut();
                let n = bii.get_6d(x + self.offset * dx_sum,
                                   y + self.offset * dy_sum,
                                   z + self.offset * dz_sum,
                                   w + self.offset * dw_sum,
                                   u + self.offset * du_sum,
                                   v + self.offset * dv_sum);
                let dx = bii.get_dx_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dx_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                let dy = bii.get_dy_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                let dz = bii.get_dz_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                let dw = bii.get_dw_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                let du = bii.get_du_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                let dv = bii.get_dv_6(x + self.offset * dx_sum,
                                      y + self.offset * dy_sum,
                                      z + self.offset * dz_sum,
                                      w + self.offset * dw_sum,
                                      u + self.offset * du_sum,
                                      v + self.offset * dv_sum);
                sum += amp * (1.0 - n.abs());
                dx_sum += amp * dx * -n;
                dy_sum += amp * dy * -n;
                dz_sum += amp * dz * -n;
                dw_sum += amp * dw * -n;
                du_sum += amp * du * -n;
                dv_sum += amp * dv * -n;
                amp *= self.gain * *(max(&0.0, min(&sum, &1.0)));
                x *= self.lacunarity;
                y *= self.lacunarity;
                z *= self.lacunarity;
                w *= self.lacunarity;
                u *= self.lacunarity;
                v *= self.lacunarity;
            }
        }
        sum
    }
}

impl ImplicitModule for ImplicitFractal {
    fn set_seed(&mut self, seed: u32) {
        for c in 0..MAX_SOURCES as u32 {
            if let Some(ref cc) = self.source[c as usize] {
                cc.borrow_mut().set_seed(seed + c * 300)
            }
        }
    }

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        match self.ftype {
            FractalType::FBM => self.fbm_get_2d(x, y),
            FractalType::RidgedMulti => self.ridged_multi_get_2d(x, y),
            FractalType::Billow => self.billow_get_2d(x, y),
            FractalType::Multi => self.multi_get_2d(x, y),
            FractalType::HybridMulti => self.hybrid_multi_get_2d(x, y),
            FractalType::DecarpentierSwiss => self.decarpentier_swiss_get_2d(x, y),
        }
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        match self.ftype {
            FractalType::FBM => self.fbm_get_3d(x, y, z),
            FractalType::RidgedMulti => self.ridged_multi_get_3d(x, y, z),
            FractalType::Billow => self.billow_get_3d(x, y, z),
            FractalType::Multi => self.multi_get_3d(x, y, z),
            FractalType::HybridMulti => self.hybrid_multi_get_3d(x, y, z),
            FractalType::DecarpentierSwiss => self.decarpentier_swiss_get_3d(x, y, z),
        }
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        match self.ftype {
            FractalType::FBM => self.fbm_get_4d(x, y, z, w),
            FractalType::RidgedMulti => self.ridged_multi_get_4d(x, y, z, w),
            FractalType::Billow => self.billow_get_4d(x, y, z, w),
            FractalType::Multi => self.multi_get_4d(x, y, z, w),
            FractalType::HybridMulti => self.hybrid_multi_get_4d(x, y, z, w),
            FractalType::DecarpentierSwiss => self.decarpentier_swiss_get_4d(x, y, z, w),
        }
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        match self.ftype {
            FractalType::FBM => self.fbm_get_6d(x, y, z, w, u, v),
            FractalType::RidgedMulti => self.ridged_multi_get_6d(x, y, z, w, u, v),
            FractalType::Billow => self.billow_get_6d(x, y, z, w, u, v),
            FractalType::Multi => self.multi_get_6d(x, y, z, w, u, v),
            FractalType::HybridMulti => self.hybrid_multi_get_6d(x, y, z, w, u, v),
            FractalType::DecarpentierSwiss => self.decarpentier_swiss_get_6d(x, y, z, w, u, v),
        }
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
