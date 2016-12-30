#![allow(dead_code)]
#![allow(unused_imports)]

extern crate num;

use std::ops::{Add, Mul, Sub, Div, Index, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use self::num::{Num, NumCast, ToPrimitive};

macro_rules! vec_type {
	($name:ident, $size:expr, [ $( $id:ident = $index:expr ),+ ]) => {
		#[derive(Clone, Copy, PartialEq)]
		pub struct $name<T>([T; $size]) where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign;
								 
		impl <T> Default for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn default() -> Self {
				$name([Default::default(); $size])
			}
		} 						 
								 
		impl <T> $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			pub fn new() -> $name<T> {
				Default::default()
			}
			
			pub fn with_value(a: T) -> $name<T> {
				$name([a; $size])
			} 
			
			pub fn with_all($($id: T),+) -> $name<T> {
				$name([$($id),+])
			}
			
			$(
			#[inline(always)]
			pub fn $id(&self) -> T {
				self.0[$index]
			}
			)+
			
			pub fn dotprod(&self, tvec: &$name<T>) -> T {
				let mut sum = T::zero();
		        
		        for i in 0..$size {
		        	sum += self.0[i] * tvec.0[i]
		        }
		        
		        sum
		    }
		    
		    pub fn len(&self) -> T {
		    	use self::num::NumCast;
		        T::from(self.dotprod(self).to_f64().unwrap().sqrt()).unwrap()
		    }
		
		    pub fn normalize(&mut self) {
		        let lng = self.len();
		        *self /= lng;
		    }
			
			/*$(
		    #[inline(always)]
			pub fn set_$id(&mut self, $id: T) {
				self.0[$index] = $id;
			}
			)+ */
			
			#[inline(always)]
			pub fn set(&mut self, index: usize, val: T) {
				self.0[index] = val;
			}
			
			
			#[inline(always)]
		    pub fn set_all(&mut self, $($id: T),+) {
		    	$(self.0[$index] = $id);+				
			}
		}	
								 
		impl <T> Index<usize> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = T;
			
			fn index(&self, index: usize) -> &Self::Output {
				&self.0[index]
			}
		}							 
								 
		impl <T> Add<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn add(self, rhs: $name<T>) -> Self::Output {
				$name ([ $(self.0[$index] + rhs.0[$index]),+ ])
			}						 	
		}	
								 
		impl <T> AddAssign<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn add_assign(&mut self, rhs: $name<T>) {
				$(self.0[$index]+=rhs.0[$index]);+
			}						 	
		}							 
								 
		impl <T> Mul<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn mul(self, rhs: $name<T>) -> Self::Output {
				$name ([$(self.0[$index] * rhs.0[$index]),+])
			}						 	
		}
								 
		impl <T> MulAssign<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn mul_assign(&mut self, rhs: $name<T>) {
				$(self.0[$index]*=rhs.0[$index]);+
			}						 	
		}							 
								 
		impl <T> Sub<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn sub(self, rhs: $name<T>) -> Self::Output {
				$name ([ $(self.0[$index] - rhs.0[$index]),+ ])
			}						 	
		}	
								 
		impl <T> SubAssign<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn sub_assign(&mut self, rhs: $name<T>) {
				$(self.0[$index]-=rhs.0[$index]);+
			}						 	
		}						 
								 
		impl <T> Div<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn div(self, rhs: $name<T>) -> Self::Output {
				$name ([ $(self.0[$index] / rhs.0[$index]),+ ])
			}						 	
		}	
								 
		impl <T> DivAssign<$name<T>> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn div_assign(&mut self, rhs: $name<T>) {
				$(self.0[$index] /= rhs.0[$index]);+
			}						 	
		}						 
								 
		impl <T> Neg for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn neg(self) -> Self::Output {
				$name ([ $(-self.0[$index]),+ ])
			}						 	
		}	
								 
		impl <T> Add<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn add(self, rhs: T) -> Self::Output {
				$name ([ $(self.0[$index] + rhs),+ ])
			}						 	
		}
								 
		impl <T> AddAssign<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn add_assign(&mut self, rhs: T) {
				$(self.0[$index] += rhs);+
			}						 	
		}							 
								 
		impl <T> Mul<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn mul(self, rhs: T) -> Self::Output {
				$name ([ $(self.0[$index] * rhs),+ ])
			}						 	
		}
								 
		impl <T> MulAssign<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn mul_assign(&mut self, rhs: T) {
				$(self.0[$index] *= rhs);+
			}						 	
		}							 
								 
		impl <T> Sub<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn sub(self, rhs: T) -> Self::Output {
				$name ([ $(self.0[$index] - rhs),+ ])
			}						 	
		}	
								 
		impl <T> SubAssign<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn sub_assign(&mut self, rhs: T) {
				$(self.0[$index] -= rhs);+
			}						 	
		}						 
								 
		impl <T> Div<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			type Output = $name<T>;
			
			fn div(self, rhs: T) -> Self::Output {
				$name ([ $(self.0[$index] / rhs),+ ])
			}						 	
		}	
								 
		impl <T> DivAssign<T> for $name<T> where T: Num + NumCast + ToPrimitive + Copy + Default + PartialEq + Neg<Output = T> 						 
								 + AddAssign + MulAssign + SubAssign + DivAssign {
			fn div_assign(&mut self, rhs: T) {
				$(self.0[$index] /= rhs);+
			}						 	
		}									 	
	}
}

vec_type!(Vec2, 2, [x = 0, y = 1]);
vec_type!(Vec3, 3, [x = 0, y = 1, z = 2]);
vec_type!(Vec4, 4, [x = 0, y = 1, z = 2, w = 3]);