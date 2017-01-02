use super::{ImplicitModule, RgbaModule, Rgba};

use std::f64::consts::PI;

const PI2: f64 = PI * 2.0;

pub enum MappingMode {
    SeamlessNone,
    SeamlessX,
    SeamlessY,
    SeamlessZ,
    SeamlessXY,
    SeamlessXZ,
    SeamlessYZ,
    SeamlessXYZ,
}

pub struct MappingRanges {
    mapx0: f64,
    mapy0: f64,
    mapz0: f64,
    mapx1: f64,
    mapy1: f64,
    mapz1: f64,
    loopx0: f64,
    loopy0: f64,
    loopz0: f64,
    loopx1: f64,
    loopy1: f64,
    loopz1: f64,
}

impl MappingRanges {
    pub fn new() -> MappingRanges {
        MappingRanges {
            mapx0: -1.0,
            mapy0: -1.0,
            mapz0: -1.0,
            loopx0: -1.0,
            loopy0: -1.0,
            loopz0: -1.0,
            mapx1: 1.0,
            mapy1: 1.0,
            mapz1: 1.0,
            loopx1: 1.0,
            loopy1: 1.0,
            loopz1: 1.0,
        }
    }
}

pub fn map_2d<Matrix: AsMut<[Row]>, Row: AsMut<[f64]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut ImplicitModule, ranges: &mut MappingRanges, z: f64) {
    let (w, h) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 { (w, h) } else { (0, 0) }
        } else {
            (0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            let mut p = x as f64 / w as f64;
            let mut q = y as f64 / h as f64;
            let r;
            let nx;
            let ny;
            let nz;
            let nw;
            let nu;
            let nv;
            let val;
            let dx;
            let dy;
            let dz;

            match seamlessmode {
                MappingMode::SeamlessNone => {
                    nx = ranges.mapx0 + p * (ranges.mapx1 - ranges.mapx0);
                    ny = ranges.mapy0 + q * (ranges.mapy1 - ranges.mapy0);
                    nz = z;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessX => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    nw = z;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessY => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nw = z;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessZ => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.mapy0 + p * dy;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    nz = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nw = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessXY => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nu = z;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessXZ => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapx1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    nw = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nu = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessYZ => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nw = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nu = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessXYZ => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nu = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nv = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, nv);
                }
            }
            a.as_mut()[x].as_mut()[y] = val;
        }
    }
}

pub fn map_2d_no_z<Matrix: AsMut<[Row]>, Row: AsMut<[f64]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut ImplicitModule, ranges: &mut MappingRanges) {
    let (w, h) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 { (w, h) } else { (0, 0) }
        } else {
            (0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            let mut p = x as f64 / w as f64;
            let mut q = y as f64 / h as f64;
            let nx;
            let ny;
            let nz;
            let nw;
            let val;
            let dx;
            let dy;

            match seamlessmode {
                MappingMode::SeamlessNone => {
                    nx = ranges.mapx0 + p * (ranges.mapx1 - ranges.mapx0);
                    ny = ranges.mapy0 + q * (ranges.mapy1 - ranges.mapy0);
                    val = m.get_2d(nx, ny);
                }
                MappingMode::SeamlessX => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessY => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessXY => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                _ => {
                    val = 0.0;
                }
            }
            a.as_mut()[x].as_mut()[y] = val;
        }
    }
}

pub fn map_3d<Matrix: AsMut<[Row]>, Row: AsMut<[Col]>, Col: AsMut<[f64]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut ImplicitModule, ranges: &mut MappingRanges) {
    let (w, h, d) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 {
                let d = a.as_mut()[0].as_mut()[0].as_mut().len();
                if d > 0 { (w, h, d) } else { (0, 0, 0) }
            } else {
                (0, 0, 0)
            }
        } else {
            (0, 0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            for z in 0..d {
                let mut p = x as f64 / w as f64;
                let mut q = y as f64 / h as f64;
                let mut r = z as f64 / d as f64;
                let nx;
                let ny;
                let nz;
                let nw;
                let nu;
                let nv;
                let val;
                let dx;
                let dy;
                let dz;

                match seamlessmode {
                    MappingMode::SeamlessNone => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.mapy0 + q * dy;
                        nz = ranges.mapz0 + r * dz;
                        val = m.get_3d(nx, ny, nz);
                    }
                    MappingMode::SeamlessX => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.mapy0 + q * dy;
                        nw = ranges.mapz0 + r * dz;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessY => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nw = ranges.mapz0 + r * dz;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessZ => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.mapy0 + q * dy;
                        nz = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nw = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessXY => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nu = ranges.mapz0 + r * dz;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessXZ => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.mapy0 + q * dy;
                        nw = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nu = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessYZ => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nw = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nu = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessXYZ => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nu = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nv = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, nv);
                    }
                }
                a.as_mut()[x].as_mut()[y].as_mut()[z] = val;
            }
        }
    }
}



pub fn map_rgba_2d<Matrix: AsMut<[Row]>, Row: AsMut<[Rgba]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut RgbaModule, ranges: &mut MappingRanges, z: f64) {
    let (w, h) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 { (w, h) } else { (0, 0) }
        } else {
            (0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            let mut p = x as f64 / w as f64;
            let mut q = y as f64 / h as f64;
            let r;
            let nx;
            let ny;
            let nz;
            let nw;
            let nu;
            let nv;
            let val;
            let dx;
            let dy;
            let dz;

            match seamlessmode {
                MappingMode::SeamlessNone => {
                    nx = ranges.mapx0 + p * (ranges.mapx1 - ranges.mapx0);
                    ny = ranges.mapy0 + q * (ranges.mapy1 - ranges.mapy0);
                    nz = z;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessX => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    nw = z;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessY => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nw = z;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessZ => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.mapy0 + p * dy;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    nz = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nw = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                MappingMode::SeamlessXY => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nu = z;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessXZ => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapx1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    nw = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nu = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessYZ => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nw = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nu = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                }
                MappingMode::SeamlessXYZ => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    dz = ranges.loopz1 - ranges.loopz0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    r = (z - ranges.mapz0) / (ranges.mapz1 - ranges.mapz0);
                    let zval = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    nu = ranges.loopz0 + (zval * PI2).cos() * dz / PI2;
                    nv = ranges.loopz0 + (zval * PI2).sin() * dz / PI2;
                    val = m.get_6d(nx, ny, nz, nw, nu, nv);
                }
            }
            a.as_mut()[x].as_mut()[y] = val;
        }
    }
}

pub fn map_rgba_2d_no_z<Matrix: AsMut<[Row]>, Row: AsMut<[Rgba]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut RgbaModule, ranges: &mut MappingRanges) {
    let (w, h) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 { (w, h) } else { (0, 0) }
        } else {
            (0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            let mut p = x as f64 / w as f64;
            let mut q = y as f64 / h as f64;
            let nx;
            let ny;
            let nz;
            let nw;
            let val;
            let dx;
            let dy;

            match seamlessmode {
                MappingMode::SeamlessNone => {
                    nx = ranges.mapx0 + p * (ranges.mapx1 - ranges.mapx0);
                    ny = ranges.mapy0 + q * (ranges.mapy1 - ranges.mapy0);
                    val = m.get_2d(nx, ny);
                }
                MappingMode::SeamlessX => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.mapy1 - ranges.mapy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.mapy0 + q * dy;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessY => {
                    dx = ranges.mapx1 - ranges.mapx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.mapx0 + p * dx;
                    ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    val = m.get_3d(nx, ny, nz);
                }
                MappingMode::SeamlessXY => {
                    dx = ranges.loopx1 - ranges.loopx0;
                    dy = ranges.loopy1 - ranges.loopy0;
                    p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                    q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                    nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                    ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                    nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                    nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                    val = m.get_4d(nx, ny, nz, nw);
                }
                _ => {
                    val = Rgba::with_value(0.0);
                }
            }
            a.as_mut()[x].as_mut()[y] = val;
        }
    }
}

pub fn map_rgba_3d<Matrix: AsMut<[Row]>, Row: AsMut<[Col]>, Col: AsMut<[Rgba]>>(seamlessmode: MappingMode, mut a: Matrix, m: &mut RgbaModule, ranges: &mut MappingRanges) {
    let (w, h, d) = {
        let w = a.as_mut().len();
        if w > 0 {
            let h = a.as_mut()[0].as_mut().len();
            if h > 0 {
                let d = a.as_mut()[0].as_mut()[0].as_mut().len();
                if d > 0 { (w, h, d) } else { (0, 0, 0) }
            } else {
                (0, 0, 0)
            }
        } else {
            (0, 0, 0)
        }
    };

    for x in 0..w {
        for y in 0..h {
            for z in 0..d {
                let mut p = x as f64 / w as f64;
                let mut q = y as f64 / h as f64;
                let mut r = z as f64 / d as f64;
                let nx;
                let ny;
                let nz;
                let nw;
                let nu;
                let nv;
                let val;
                let dx;
                let dy;
                let dz;

                match seamlessmode {
                    MappingMode::SeamlessNone => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.mapy0 + q * dy;
                        nz = ranges.mapz0 + r * dz;
                        val = m.get_3d(nx, ny, nz);
                    }
                    MappingMode::SeamlessX => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.mapy0 + q * dy;
                        nw = ranges.mapz0 + r * dz;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessY => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nw = ranges.mapz0 + r * dz;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessZ => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.mapy0 + q * dy;
                        nz = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nw = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_4d(nx, ny, nz, nw);
                    }
                    MappingMode::SeamlessXY => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.mapz1 - ranges.mapz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nu = ranges.mapz0 + r * dz;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessXZ => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.mapy1 - ranges.mapy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.mapy0 + q * dy;
                        nw = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nu = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessYZ => {
                        dx = ranges.mapx1 - ranges.mapx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.mapx0 + p * dx;
                        ny = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nz = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nw = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nu = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, 0.0);
                    }
                    MappingMode::SeamlessXYZ => {
                        dx = ranges.loopx1 - ranges.loopx0;
                        dy = ranges.loopy1 - ranges.loopy0;
                        dz = ranges.loopz1 - ranges.loopz0;
                        p = p * (ranges.mapx1 - ranges.mapx0) / (ranges.loopx1 - ranges.loopx0);
                        q = q * (ranges.mapy1 - ranges.mapy0) / (ranges.loopy1 - ranges.loopy0);
                        r = r * (ranges.mapz1 - ranges.mapz0) / (ranges.loopz1 - ranges.loopz0);
                        nx = ranges.loopx0 + (p * PI2).cos() * dx / PI2;
                        ny = ranges.loopx0 + (p * PI2).sin() * dx / PI2;
                        nz = ranges.loopy0 + (q * PI2).cos() * dy / PI2;
                        nw = ranges.loopy0 + (q * PI2).sin() * dy / PI2;
                        nu = ranges.loopz0 + (r * PI2).cos() * dz / PI2;
                        nv = ranges.loopz0 + (r * PI2).sin() * dz / PI2;
                        val = m.get_6d(nx, ny, nz, nw, nu, nv);
                    }
                }
                a.as_mut()[x].as_mut()[y].as_mut()[z] = val;
            }
        }
    }
}
