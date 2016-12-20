#[inline(always)]
pub fn clamp(v: f64, l: f64, h: f64) -> f64 {
	if v < l 		{ l }
	else if v > h 	{ h }
	else			{ v }
}

#[inline(always)]
pub fn lerp(t: f64, a: f64, b: f64) -> f64 {
	a + t*(b-a)
}

#[inline(always)]
pub fn is_power_of_2(n: f64) -> bool {
	let nn = n as i64;
    ((nn-1) & nn) != 0
}

#[inline(always)]
pub fn hermite_blend(t: f64) -> f64 {
	(t * t * (3.0 - 2.0 * t))
}

#[inline(always)]
pub fn quintic_blend(t: f64) -> f64 {
	t * t * t * (t*(t * 6.0-15.0) + 10.0)
}

#[inline(always)]
pub fn array_dot(arr: &[f64], a: f64, b: f64) -> f64 {
	a * arr[0] + b * arr[1] 
}

#[inline(always)]
pub fn array_dot_3(arr: &[f64], a: f64, b: f64, c: f64) -> f64 {
	a * arr[0] + b * arr[1] + c * arr[2]
}

#[inline(always)]
pub fn array_dot_4(arr: &[f64], a: f64, b: f64, c: f64, d: f64) -> f64 {
	a * arr[0] + b * arr[1] + c * arr[2] + d * arr[3]
}

#[inline(always)]
pub fn array_dot_6(arr: &[f64], a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> f64 {
	a * arr[0] + b * arr[1] + c * arr[2] + d * arr[3] + e * arr[4] + f * arr[5]
}

#[inline(always)]
pub fn fast_floor(t: f64) -> i32 {
	return if t > 0.0 { t as i32 } else { (t-1.0) as i32 }
}

#[inline(always)]
pub fn bias(b: f64, t: f64) -> f64 {
	return t.powf(b.ln()/0.5f64.ln())
}

#[inline(always)]
pub fn gain(g: f64, t: f64) -> f64 {
	if t < 0.5 {
		return bias(1.0-g, 2.0*t) / 2.0
	} else {
		return 1.0 - bias(1.0-g, 2.0-2.0*t)/2.0
	}
}