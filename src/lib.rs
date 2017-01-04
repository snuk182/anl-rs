/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).
///
/// The basic idea of ANL is that functions are modular and may be connected together in chains to build more complex functions. All functions provide an interface allowing the function to be queried for a value. The interface provides methods for generating values in 2, 3, 4 or 6 dimensions. (The reason for providing such high orders of function is explained in the section on seamless noise.) Some modules are only generators, with no modifiable inputs or parameters, and are only connectable to other modules by being used as sources. Others are modifiers that somehow modify the output of another function; for these, other functions can be specified for the inputs.
///
/// The functions in ANL are organized into 2 main categories: Implicit functions and RGBA functions. Implicit functions are the noise functions that generate double-precision floating point output values. RGBA functions operate in RGBA space, and the values they output are 4-component single-precision floating point vectors representing RGBA colors. There are adapter functions that can take a number of Implicit functions and compose them into an RGBA function. As well, there is an adapter that can decompose an RGBA function, extracting a specified channel to be used as an Implicit source.

#[macro_use]
extern crate lazy_static;

pub mod utility;
pub mod curve;
pub mod hsv;
pub mod mapping;
pub mod vector_types;

pub mod noise_gen;
pub mod cellular_gen;
pub mod random_gen;

mod implicit_base;
mod rgba_module_base;

pub use implicit_base::{ImplicitModule, ScalarParameter};
pub use rgba_module_base::*;

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
mod implicit_sphere;
mod implicit_tiers;
mod implicit_triangle;
mod implicit_bright_contrast;

mod rgba_blend;
mod rgba_blend_ops;
mod rgba_color_ops;
mod rgba_compose_channels;
mod rgba_constant;
mod rgba_curve;
mod rgba_hsv_to_rgba;
mod rgba_implicit_greyscale;
mod rgba_normalize;
mod rgba_rgba_to_hsv;
mod rgba_rotate_color;
mod rgba_select;

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
pub use implicit_sphere::*;
pub use implicit_tiers::*;
pub use implicit_triangle::*;
pub use implicit_bright_contrast::*;

pub use rgba_blend::*;
pub use rgba_blend_ops::*;
pub use rgba_color_ops::*;
pub use rgba_compose_channels::*;
pub use rgba_constant::*;
pub use rgba_curve::*;
pub use rgba_hsv_to_rgba::*;
pub use rgba_implicit_greyscale::*;
pub use rgba_normalize::*;
pub use rgba_rgba_to_hsv::*;
pub use rgba_rotate_color::*;
pub use rgba_select::*;