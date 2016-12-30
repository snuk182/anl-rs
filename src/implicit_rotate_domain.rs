use super::implicit_base::{ImplicitModuleBase, ScalarParameter};
use super::ImplicitModule;

use std::rc::Rc;
use std::cell::RefCell;

// Given angle r in radians and unit vector u = ai + bj + ck or [a,b,c]', define:
//
// q0 = cos(r/2),  q1 = sin(r/2) a,  q2 = sin(r/2) b,  q3 = sin(r/2) c
//
// and construct from these values the rotation matrix:
//
// (q0² + q1² - q2² - q3²)      2(q1q2 - q0q3)          2(q1q3 + q0q2)
//
// Q  =      2(q2q1 + q0q3)     (q0² - q1² + q2² - q3²)      2(q2q3 - q0q1)
//
// 2(q3q1 - q0q2)          2(q3q2 + q0q1)     (q0² - q1² - q2² + q3²)
//
// Multiplication by Q then effects the desired rotation, and in particular:
//
// Q u = u
//

pub struct ImplicitRotateDomain {
    base: ImplicitModuleBase,
    rotmatrix: [[f64; 3]; 3],
    ax: ScalarParameter,
    ay: ScalarParameter,
    az: ScalarParameter,
    angledeg: ScalarParameter,
    source: ScalarParameter,
}

impl ImplicitRotateDomain {
    pub fn with_axises_angles(ax: f64, ay: f64, az: f64, deg: f64) -> ImplicitRotateDomain {
        ImplicitRotateDomain {
            base: Default::default(),
            rotmatrix: [[0.0; 3]; 3],
            ax: ScalarParameter::Value(ax),
            ay: ScalarParameter::Value(ay),
            az: ScalarParameter::Value(az),
            angledeg: ScalarParameter::Value(deg),
            source: ScalarParameter::Value(0.0),
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

    pub fn set_source_module(&mut self, m: Rc<RefCell<ImplicitModule>>) {
        self.source = ScalarParameter::Source(m);
    }
    pub fn set_source_value(&mut self, v: f64) {
        self.source = ScalarParameter::Value(v);
    }

    #[allow(dead_code)]
    fn calculate_rot_matrix_2d(&mut self, x: f64, y: f64) {
        let angle = self.angledeg.get_2d(x, y) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_2d(x, y);
        let ay = self.ay.get_2d(x, y);
        let az = self.az.get_2d(x, y);

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

    fn calculate_rot_matrix_3d(&mut self, x: f64, y: f64, z: f64) {
        let angle = self.angledeg.get_3d(x, y, z) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_3d(x, y, z);
        let ay = self.ay.get_3d(x, y, z);
        let az = self.az.get_3d(x, y, z);

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
    fn calculate_rot_matrix_4d(&mut self, x: f64, y: f64, z: f64, w: f64) {
        let angle = self.angledeg.get_4d(x, y, z, w) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_4d(x, y, z, w);
        let ay = self.ay.get_4d(x, y, z, w);
        let az = self.az.get_4d(x, y, z, w);

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
    fn calculate_rot_matrix_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) {
        let angle = self.angledeg.get_6d(x, y, z, w, u, v) * 360.0 * ::std::f64::consts::PI / 180.0;
        let ax = self.ax.get_6d(x, y, z, w, u, v);
        let ay = self.ay.get_6d(x, y, z, w, u, v);
        let az = self.az.get_6d(x, y, z, w, u, v);

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
}

impl ImplicitModule for ImplicitRotateDomain {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        let angle = self.angledeg.get_2d(x, y) * 360.0 * ::std::f64::consts::PI / 180.0;
        let cos2d = angle.cos();
        let sin2d = angle.sin();
        // self.calculate_rot_matrix_2d(x,y);
        let nx = x * cos2d - y * sin2d;
        let ny = y * cos2d + x * sin2d;
        self.source.get_2d(nx, ny)
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.calculate_rot_matrix_3d(x, y, z);
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        self.source.get_3d(nx, ny, nz)
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.calculate_rot_matrix_4d(x, y, z, w);
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        self.source.get_4d(nx, ny, nz, w)
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.calculate_rot_matrix_6d(x, y, z, w, u, v);
        let nx = (self.rotmatrix[0][0] * x) + (self.rotmatrix[1][0] * y) + (self.rotmatrix[2][0] * z);
        let ny = (self.rotmatrix[0][1] * x) + (self.rotmatrix[1][1] * y) + (self.rotmatrix[2][1] * z);
        let nz = (self.rotmatrix[0][2] * x) + (self.rotmatrix[1][2] * y) + (self.rotmatrix[2][2] * z);
        self.source.get_6d(nx, ny, nz, w, u, v)
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
