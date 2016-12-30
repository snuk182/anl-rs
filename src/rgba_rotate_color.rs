use super::rgba_module_base::{RgbaModule, RgbaParameter, Rgba};
use super::implicit_base::ScalarParameter;
use super::utility::clamp;
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

pub struct RgbaRotateColor {
    ax: ScalarParameter,
    ay: ScalarParameter,
    az: ScalarParameter,
    angledeg: ScalarParameter,
    source: RgbaParameter,
    normalize_axis: bool,
    rotmatrix: [[f64; 3]; 3],
}

impl RgbaRotateColor {
    pub fn with_axises_angles(ax: f64, ay: f64, az: f64, deg: f64) -> RgbaRotateColor {
        RgbaRotateColor {
            ax: ScalarParameter::Value(ax),
            ay: ScalarParameter::Value(ay),
            az: ScalarParameter::Value(az),
            angledeg: ScalarParameter::Value(deg),
            source: RgbaParameter::Constant(Rgba::with_value(0.0)),
            normalize_axis: false,
            rotmatrix: [[0.0; 3]; 3],
        }
    }

    pub fn set_ax_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.ax = ScalarParameter::Source(m);
    }
    pub fn set_ax_value(&mut self, v: f64) {
        self.ax = ScalarParameter::Value(v);
    }

    pub fn set_ay_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.ay = ScalarParameter::Source(m);
    }
    pub fn set_ay_value(&mut self, v: f64) {
        self.ay = ScalarParameter::Value(v);
    }

    pub fn set_az_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.az = ScalarParameter::Source(m);
    }
    pub fn set_az_value(&mut self, v: f64) {
        self.az = ScalarParameter::Value(v);
    }

    pub fn set_degree_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.angledeg = ScalarParameter::Source(m);
    }
    pub fn set_degree_value(&mut self, v: f64) {
        self.angledeg = ScalarParameter::Value(v);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source = RgbaParameter::Module(m);
    }
    pub fn set_source_value(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_normalize_axis(&mut self, normalize: bool) {
        self.normalize_axis = normalize;
    }

    fn calculate_rot_matrix(&mut self, angle: f64, ax: f64, ay: f64, az: f64) {
        let cosangle = angle.cos();
        let sinangle = angle.sin();

        self.rotmatrix[0][0] = 1.0 + (1.0 - cosangle) * (ax * ax - 1.0);
        self.rotmatrix[1][0] = -az * sinangle + (1.0 - cosangle) * ax * ay;
        self.rotmatrix[2][0] = ay * sinangle + (1.0 - cosangle) * ax * az;

        self.rotmatrix[0][1] = az * sinangle + (1.0 - cosangle) * ax * ay;
        self.rotmatrix[1][1] = 1.0 + (1.0 - cosangle) * (ay * ay - 1.0);
        self.rotmatrix[2][1] = -ax * sinangle + (1.0 - cosangle) * ay * az;

        self.rotmatrix[0][2] = -ay * sinangle + (1.0 - cosangle) * ax * az;
        self.rotmatrix[1][2] = ax * sinangle + (1.0 - cosangle) * ay * az;
        self.rotmatrix[2][2] = 1.0 + (1.0 - cosangle) * (az * az - 1.0);
    }

    fn calculate_rot_matrix_2d(&mut self, x: f64, y: f64) {
        let angle = self.angledeg.get_2d(x, y) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_2d(x, y);
        let ay = self.ay.get_2d(x, y);
        let az = self.az.get_2d(x, y);

        self.calculate_rot_matrix(angle, ax, ay, az);
    }

    fn calculate_rot_matrix_3d(&mut self, x: f64, y: f64, z: f64) {
        let angle = self.angledeg.get_3d(x, y, z) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_3d(x, y, z);
        let ay = self.ay.get_3d(x, y, z);
        let az = self.az.get_3d(x, y, z);

        self.calculate_rot_matrix(angle, ax, ay, az);
    }
    fn calculate_rot_matrix_4d(&mut self, x: f64, y: f64, z: f64, w: f64) {
        let angle = self.angledeg.get_4d(x, y, z, w) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_4d(x, y, z, w);
        let ay = self.ay.get_4d(x, y, z, w);
        let az = self.az.get_4d(x, y, z, w);

        self.calculate_rot_matrix(angle, ax, ay, az);
    }
    fn calculate_rot_matrix_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) {
        let angle = self.angledeg.get_6d(x, y, z, w, u, v) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_6d(x, y, z, w, u, v);
        let ay = self.ay.get_6d(x, y, z, w, u, v);
        let az = self.az.get_6d(x, y, z, w, u, v);

        self.calculate_rot_matrix(angle, ax, ay, az);
    }

    fn get(&mut self, s: &mut Rgba) {
        let mut r = s[0] * 2.0 - 1.0;
        let mut g = s[1] * 2.0 - 1.0;
        let mut b = s[2] * 2.0 - 1.0;

        s.set(0, r);
        s.set(1, g);
        s.set(2, b);

        *s = Rgba::with_all(((self.rotmatrix[0][0] * s[0] as f64) + (self.rotmatrix[1][0] * s[1] as f64) + (self.rotmatrix[2][0] * s[2] as f64)) as f32,
                            ((self.rotmatrix[0][1] * s[0] as f64) + (self.rotmatrix[1][1] * s[1] as f64) + (self.rotmatrix[2][1] * s[2] as f64)) as f32,
                            ((self.rotmatrix[0][2] * s[0] as f64) + (self.rotmatrix[1][2] * s[1] as f64) + (self.rotmatrix[2][2] * s[2] as f64)) as f32,
                            s[3]);

        r = clamp((s[0] * 0.5 + 0.5) as f64, 0.0, 1.0) as f32;
        g = clamp((s[1] * 0.5 + 0.5) as f64, 0.0, 1.0) as f32;
        b = clamp((s[2] * 0.5 + 0.5) as f64, 0.0, 1.0) as f32;

        s.set(0, r);
        s.set(1, g);
        s.set(2, b);
    }
}

impl RgbaModule for RgbaRotateColor {
    fn get_2d(&mut self, x: f64, y: f64) -> Rgba {
        let mut s = self.source.get_2d(x, y);
        self.calculate_rot_matrix_2d(x, y);
        self.get(&mut s);
        s
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> Rgba {
        let mut s = self.source.get_3d(x, y, z);
        self.calculate_rot_matrix_3d(x, y, z);
        self.get(&mut s);
        s
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> Rgba {
        let mut s = self.source.get_4d(x, y, z, w);
        self.calculate_rot_matrix_4d(x, y, z, w);
        self.get(&mut s);
        s
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> Rgba {
        let mut s = self.source.get_6d(x, y, z, w, u, v);
        self.calculate_rot_matrix_6d(x, y, z, w, u, v);
        self.get(&mut s);
        s
    }
}
