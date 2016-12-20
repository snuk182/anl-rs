//#![feature(conservative_impl_trait)]
#[macro_use]
extern crate lazy_static;

mod noise_gen;
pub use noise_gen::{value_noise_2d, value_noise_3d, value_noise_4d, value_noise_6d, gradient_noise_2d, gradient_noise_3d, gradient_noise_4d, gradient_noise_6d, gradval_noise_2d, gradval_noise_3d, gradval_noise_4d, gradval_noise_6d, white_noise_2d, white_noise_3d, white_noise_4d, white_noise_6d,
                    cellular_function_2d, cellular_function_3d, cellular_function_4d, cellular_function_6d, simplex_noise_2d, simplex_noise_3d, simplex_noise_4d, simplex_noise_6d, new_simplex_noise_4d, no_interp, linear_interp, hermite_interp, quintic_interp};

mod implicit_base;
pub use implicit_base::{ScalarParameter, ImplicitModule, MAX_SOURCES};

pub mod implicit_cache;
pub mod implicit_auto_correct;
pub mod implicit_bias;
pub mod implicit_blend;
pub mod implicit_cellular;
pub mod implicit_combiner;
pub mod implicit_fractal;
pub mod implicit_gradient;
pub mod implicit_scale_domain;
pub mod implicit_scale_offset;
pub mod implicit_select;
pub mod implicit_translate_domain;

pub mod utility;

pub mod cellular_gen;
pub mod random_gen;
pub mod implicit_basis_function;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
