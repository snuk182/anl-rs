//#![feature(conservative_impl_trait)]
#[macro_use]
extern crate lazy_static;

mod noise_gen;
pub use noise_gen::{value_noise_2d, value_noise_3d, value_noise_4d, value_noise_6d, gradient_noise_2d, gradient_noise_3d, gradient_noise_4d, gradient_noise_6d, gradval_noise_2d, gradval_noise_3d, gradval_noise_4d, gradval_noise_6d, white_noise_2d, white_noise_3d, white_noise_4d, white_noise_6d,
                    cellular_function_2d, cellular_function_3d, cellular_function_4d, cellular_function_6d, simplex_noise_2d, simplex_noise_3d, simplex_noise_4d, simplex_noise_6d, new_simplex_noise_4d, no_interp, linear_interp, hermite_interp, quintic_interp};

mod implicit_base;
pub use implicit_base::{ScalarParameter, ImplicitModule, MAX_SOURCES};

mod implicit_cache;
mod implicit_auto_correct;
mod implicit_bias;
mod implicit_blend;
mod implicit_cellular;
mod implicit_combiner;
mod implicit_fractal;
mod implicit_gradient;
mod implicit_scale_domain;
mod implicit_scale_offset;
mod implicit_select;
mod implicit_translate_domain;
mod implicit_basis_function;

pub use implicit_cache::*;
pub use implicit_auto_correct::*;
pub use implicit_bias::*;
pub use implicit_blend::*;
pub use implicit_cellular::*;
pub use implicit_combiner::*;
pub use implicit_fractal::*;
pub use implicit_gradient::*;
pub use implicit_scale_domain::*;
pub use implicit_scale_offset::*;
pub use implicit_select::*;
pub use implicit_translate_domain::*;
pub use implicit_basis_function::*;

pub mod utility;

pub mod cellular_gen;
pub mod random_gen;

