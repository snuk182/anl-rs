use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::noise_gen::*;
use super::random_gen::*;

#[derive(Clone, Debug, PartialEq)]
pub enum BasisType {
    Value,
    Gradient,
    Gradval,
    Simplex,
    White,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterpType {
    None,
    Linear,
    Cubic,
    Quintic,
}

pub struct ImplicitBasisFunction {
    base: ImplicitModuleBase,
    scale: [f64; 4],
    offset: [f64; 4],
    interp: InterpFunc,
    f2d: NoiseFunc2,
    f3d: NoiseFunc3,
    f4d: NoiseFunc4,
    f6d: NoiseFunc6,
    seed: u32,
    rotmatrix: [[f64; 3]; 3],
    cos2d: f64,
    sin2d: f64,
}

impl Default for ImplicitBasisFunction {
    fn default() -> Self {
        ImplicitBasisFunction {
            base: Default::default(),
            scale: [0.0; 4],
            offset: [0.0; 4],
            interp: unsafe { ::std::mem::uninitialized() },
            f2d: unsafe { ::std::mem::uninitialized() },
            f3d: unsafe { ::std::mem::uninitialized() },
            f4d: unsafe { ::std::mem::uninitialized() },
            f6d: unsafe { ::std::mem::uninitialized() },
            seed: 0,
            rotmatrix: [[0.0; 3]; 3],
            cos2d: 0.0,
            sin2d: 0.0,
        }
    }
}

impl ImplicitBasisFunction {
    pub fn new() -> ImplicitBasisFunction {
        ImplicitBasisFunction::with_types(BasisType::Gradient, InterpType::Quintic)
    }

    pub fn with_types(btype: BasisType, itype: InterpType) -> ImplicitBasisFunction {
        let mut f: ImplicitBasisFunction = Default::default();

        f.set_type(btype);
        f.set_interp(itype);
        f.set_seed(1000);

        f
    }

    pub fn set_type(&mut self, t: BasisType) {
        match t {
            BasisType::Value => {
                self.f2d = value_noise_2d;
                self.f3d = value_noise_3d;
                self.f4d = value_noise_4d;
                self.f6d = value_noise_6d;
            }
            BasisType::Gradient => {
                self.f2d = gradient_noise_2d;
                self.f3d = gradient_noise_3d;
                self.f4d = gradient_noise_4d;
                self.f6d = gradient_noise_6d;
            }
            BasisType::Gradval => {
                self.f2d = gradval_noise_2d;
                self.f3d = gradval_noise_3d;
                self.f4d = gradval_noise_4d;
                self.f6d = gradval_noise_6d;
            }
            BasisType::White => {
                self.f2d = white_noise_2d;
                self.f3d = white_noise_3d;
                self.f4d = white_noise_4d;
                self.f6d = white_noise_6d;
            }
            BasisType::Simplex => {
                self.f2d = simplex_noise_2d;
                self.f3d = simplex_noise_3d;
                self.f4d = simplex_noise_4d;
                self.f6d = simplex_noise_6d;
            }
        }
        self.set_magic_numbers(t)
    }

    pub fn set_interp(&mut self, interp: InterpType) {
        match interp {
            InterpType::None => self.interp = no_interp,
            InterpType::Linear => self.interp = linear_interp,
            InterpType::Cubic => self.interp = hermite_interp,
            InterpType::Quintic => self.interp = quintic_interp,
        }
    }

    pub fn set_rotation_angle(&mut self, x: f64, y: f64, z: f64, angle: f64) {
        self.rotmatrix[0][0] = 1.0 + (1.0 - angle.cos()) * (x * x - 1.0);
        self.rotmatrix[1][0] = -z * angle.sin() + (1.0 - angle.cos()) * x * y;
        self.rotmatrix[2][0] = y * angle.sin() + (1.0 - angle.cos()) * x * z;

        self.rotmatrix[0][1] = z * angle.sin() + (1.0 - angle.cos()) * x * y;
        self.rotmatrix[1][1] = 1.0 + (1.0 - angle.cos()) * (y * y - 1.0);
        self.rotmatrix[2][1] = -x * angle.sin() + (1.0 - angle.cos()) * y * z;

        self.rotmatrix[0][2] = -y * angle.sin() + (1.0 - angle.cos()) * x * z;
        self.rotmatrix[1][2] = x * angle.sin() + (1.0 - angle.cos()) * y * z;
        self.rotmatrix[2][2] = 1.0 + (1.0 - angle.cos()) * (z * z - 1.0);
    }

    pub fn set_magic_numbers(&mut self, btype: BasisType) {
        // This function is a damned hack.
        // The underlying noise functions don't return values in the range [-1,1] cleanly, and the ranges vary depending
        // on basis type and dimensionality. There's probably a better way to correct the ranges, but for now I'm just
        // setting he magic numbers self.scale and self.offset manually to empirically determined magic numbers.
        match btype {
            BasisType::Value => {
                self.scale[0] = 1.0;
                self.offset[0] = 0.0;
                self.scale[1] = 1.0;
                self.offset[1] = 0.0;
                self.scale[2] = 1.0;
                self.offset[2] = 0.0;
                self.scale[3] = 1.0;
                self.offset[3] = 0.0;
            }
            BasisType::Gradient => {
                self.scale[0] = 1.86848;
                self.offset[0] = -0.000118;
                self.scale[1] = 1.85148;
                self.offset[1] = -0.008272;
                self.scale[2] = 1.64127;
                self.offset[2] = -0.01527;
                self.scale[3] = 1.92517;
                self.offset[3] = 0.03393;
            }
            BasisType::Gradval => {
                self.scale[0] = 0.6769;
                self.offset[0] = -0.00151;
                self.scale[1] = 0.6957;
                self.offset[1] = -0.133;
                self.scale[2] = 0.74622;
                self.offset[2] = 0.01916;
                self.scale[3] = 0.7961;
                self.offset[3] = -0.0352;
            }
            BasisType::White => {
                self.scale[0] = 1.0;
                self.offset[0] = 0.0;
                self.scale[1] = 1.0;
                self.offset[1] = 0.0;
                self.scale[2] = 1.0;
                self.offset[2] = 0.0;
                self.scale[3] = 1.0;
                self.offset[3] = 0.0;
            }
            BasisType::Simplex => {
                self.scale[0] = 1.0;
                self.offset[0] = 0.0;
                self.scale[1] = 1.0;
                self.offset[1] = 0.0;
                self.scale[2] = 1.0;
                self.offset[2] = 0.0;
                self.scale[3] = 1.0;
                self.offset[3] = 0.0;
            }
        }
    }
}

impl ImplicitModule for ImplicitBasisFunction {
    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
        let mut lcg = LCG::new();
        lcg.set_seed(seed);

        let ax = get_01(&mut lcg);
        let ay = get_01(&mut lcg);
        let az = get_01(&mut lcg);
        let length = (ax * ax + ay * ay + az * az).sqrt();
        // let (ax, ay, az) = (length, length, length);
        self.set_rotation_angle(length,
                                length,
                                length,
                                get_01(&mut lcg) * ::std::f64::consts::PI * 2.0);
        let angle = get_01(&mut lcg) * ::std::f64::consts::PI * 2.0;
        self.cos2d = angle.cos();
        self.sin2d = angle.sin();
    }

    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let nx = x * self.cos2d - y * self.sin2d;
        let ny = y * self.cos2d + x * self.sin2d;
        (self.f2d)(nx, ny, self.seed, self.interp)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        (self.f3d)(nx, ny, nz, self.seed, self.interp)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        (self.f4d)(nx, ny, nz, w, self.seed, self.interp)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        (self.f6d)(nx, ny, nz, w, u, v, self.seed, self.interp)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }

    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
