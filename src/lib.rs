#[macro_use]
extern crate lazy_static;

mod utility;
mod curve;
mod vector_types;

mod noise_gen;
mod cellular_gen;
mod random_gen;

mod implicit_base;
mod rgba_module_base;

pub use implicit_base::{ImplicitModule, ScalarParameter};

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
mod implicit_clamp;
mod implicit_constant;
mod implicit_cos;
mod implicit_sin;
mod implicit_curve;
mod implicit_extract_rgba_channel;
mod implicit_floor;
mod implicit_function_gradient;
mod implicit_gain;
mod implicit_magnitude;
mod implicit_modifier;
mod implicit_normalize_coords;
mod implicit_pow;
mod implicit_rgba_dot_product;
mod implicit_rotate_domain;
mod implicit_saw_tooth;

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
pub use implicit_clamp::*;
pub use implicit_constant::*;
pub use implicit_cos::*;
pub use implicit_sin::*;
pub use implicit_curve::*;
pub use implicit_extract_rgba_channel::*;
pub use implicit_floor::*;
pub use implicit_function_gradient::*;
pub use implicit_gain::*;
pub use implicit_magnitude::*;
pub use implicit_modifier::*;
pub use implicit_normalize_coords::*;
pub use implicit_rgba_dot_product::*;
pub use implicit_pow::*;
pub use implicit_rotate_domain::*;
pub use implicit_saw_tooth::*;