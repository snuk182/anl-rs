extern crate num;

use std::ops::{Add, Mul, Sub, Div, Index, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use self::num::Float;

macro_rules! vec_type {
	($name:ident, $size:expr, [$( $id:ident : $index:expr, )+]) => {
		
	}
}

#[derive(Clone, Copy, PartialEq)]
struct Vec2d<T>([T; 2]) where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign;
						 
impl <T> Default for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn default() -> Self {
		Vec2d([Default::default(); 2])
	}
} 						 
						 
impl <T> Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	pub fn new() -> Vec2d<T> {
		Default::default()
	}
	
	pub fn with_value(a: T) -> Vec2d<T> {
		Vec2d([a; 2])
	} 
	
	pub fn with_x_y(x: T, y: T) -> Vec2d<T> {
		Vec2d([x, y])
	}
	
	#[inline(always)]
	pub fn x(&self) -> T {
		self.0[0]
	}
	
	#[inline(always)]
    pub fn y(&self) -> T {
		self.0[1]
	}
    
    pub fn dotprod(&self, tvec: &Vec2d<T>) -> T {
        self.0[0] * tvec.0[0] + self.0[1] * tvec.0[1]
    }
    
    pub fn len(&self) -> T {
        self.dotprod(self).sqrt()
    }

    pub fn normalize(&mut self) {
        let lng = self.len();
        *self /= lng;
    }

    #[inline(always)]
	pub fn set_x(&mut self, x: T) {
		self.0[0] = x;
	}
	
	#[inline(always)]
    pub fn set_y(&mut self, y: T) {
		self.0[1] = y;
	}
    
    #[inline(always)]
    pub fn set_x_y(&mut self, x: T, y: T) {
    	self.0[0] = x;
		self.0[1] = y;
	}
}	
						 
impl <T> Index<usize> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = T;
	
	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}							 
						 
impl <T> Add<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn add(self, rhs: Vec2d<T>) -> Self::Output {
		Vec2d ([self.0[0]+rhs.0[0], self.0[1]+rhs.0[1]])
	}						 	
}	
						 
impl <T> AddAssign<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn add_assign(&mut self, rhs: Vec2d<T>) {
		self.0[0]+=rhs.0[0];
		self.0[1]+=rhs.0[1];
	}						 	
}							 
						 
impl <T> Mul<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn mul(self, rhs: Vec2d<T>) -> Self::Output {
		Vec2d ([self.0[0]*rhs.0[0], self.0[1]*rhs.0[1]])
	}						 	
}
						 
impl <T> MulAssign<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn mul_assign(&mut self, rhs: Vec2d<T>) {
		self.0[0]*=rhs.0[0];
		self.0[1]*=rhs.0[1];
	}						 	
}							 
						 
impl <T> Sub<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn sub(self, rhs: Vec2d<T>) -> Self::Output {
		Vec2d ([self.0[0]-rhs.0[0], self.0[1]-rhs.0[1]])
	}						 	
}	
						 
impl <T> SubAssign<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn sub_assign(&mut self, rhs: Vec2d<T>) {
		self.0[0]-=rhs.0[0];
		self.0[1]-=rhs.0[1];
	}						 	
}						 
						 
impl <T> Div<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn div(self, rhs: Vec2d<T>) -> Self::Output {
		Vec2d ([self.0[0]/rhs.0[0], self.0[1]/rhs.0[1]])
	}						 	
}	
						 
impl <T> DivAssign<Vec2d<T>> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn div_assign(&mut self, rhs: Vec2d<T>) {
		self.0[0]/=rhs.0[0];
		self.0[1]/=rhs.0[1];
	}						 	
}						 
						 
impl <T> Neg for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn neg(self) -> Self::Output {
		Vec2d ([-self.0[0], -self.0[1]])
	}						 	
}	
						 
impl <T> Add<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn add(self, rhs: T) -> Self::Output {
		Vec2d ([self.0[0]+rhs, self.0[1]+rhs])
	}						 	
}
						 
impl <T> AddAssign<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn add_assign(&mut self, rhs: T) {
		self.0[0]+=rhs;
		self.0[1]+=rhs;
	}						 	
}							 
						 
impl <T> Mul<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn mul(self, rhs: T) -> Self::Output {
		Vec2d ([self.0[0]*rhs, self.0[1]*rhs])
	}						 	
}
						 
impl <T> MulAssign<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn mul_assign(&mut self, rhs: T) {
		self.0[0]*=rhs;
		self.0[1]*=rhs;
	}						 	
}							 
						 
impl <T> Sub<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn sub(self, rhs: T) -> Self::Output {
		Vec2d ([self.0[0]-rhs, self.0[1]-rhs])
	}						 	
}	
						 
impl <T> SubAssign<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn sub_assign(&mut self, rhs: T) {
		self.0[0]-=rhs;
		self.0[1]-=rhs;
	}						 	
}						 
						 
impl <T> Div<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	type Output = Vec2d<T>;
	
	fn div(self, rhs: T) -> Self::Output {
		Vec2d ([self.0[0]/rhs, self.0[1]/rhs])
	}						 	
}	
						 
impl <T> DivAssign<T> for Vec2d<T> where T: Float + Copy + Default + PartialEq + Neg<Output = T> 
						 + Add<f64, Output = T> + Mul<f64, Output = T> + Sub<f64, Output = T> + Div<f64, Output = T>
						 + AddAssign + MulAssign + SubAssign + DivAssign {
	fn div_assign(&mut self, rhs: T) {
		self.0[0]/=rhs;
		self.0[1]/=rhs;
	}						 	
}		