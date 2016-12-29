extern crate anl;
extern crate image;

use anl::*;

use std::fs::*;
use std::rc::Rc;
use std::cell::RefCell;

const SIZE: u32 = 800;

macro_rules! case {
   ($name:ident, $start:expr, $conf:expr) => {
	   {
		   fn $name() {
			   let mut $name = $start;
			   $conf
			   write_image(stringify!($name), &mut $name);
		   }
		   
		   $name();
	   }
   }
}

fn main() {
    case!(gradient, anl::ImplicitGradient::new(), {});
    case!(select, anl::ImplicitSelect::new(), 
	    {
		    select.set_low_source_value(0.0);
		    select.set_high_source_value(1.0);
		    select.set_control_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
		    select.set_threshold_source_value(0.5);
	    }
    );
    case!(fractal, 
	    anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Cubic),
	    {
		    fractal.set_num_octaves(2);
		    fractal.set_seed(1234);
		    fractal.set_frequency(4.0);
		    fractal.set_lacunarity(0.3);
		    fractal.set_gain(1.0);
	    }
    );
    case!(cellular, anl::ImplicitCellular::with_coefficients(0.0, 1.0, -1.0, 0.0), 
	    {
		    cellular.set_cellular_source(Some(Rc::new(RefCell::new(anl::cellular_gen::CellularGenerator::new()))));
		    cellular.set_seed(1234);
	    }
    );
    case!(normalize_coords, anl::ImplicitNormalizeCoords::new(),
	    {
		    normalize_coords.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    case!(rgba_dot_product, anl::ImplicitRgbaDotProduct::new(),
	    {
		    rgba_dot_product.set_source_1_constant(0.0, 50.0, 150.0, 255.0);
		    rgba_dot_product.set_source_2_constant(0.0, 0.25, 0.75, 1.0);
	    }
    );
    case!(saw_tooth, anl::ImplicitSawTooth::with_period(0.4),
	    {
		    saw_tooth.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    case!(sin, anl::ImplicitSin::new(),
	    {
		    sin.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    case!(cos, anl::ImplicitCos::new(),
	    {
		    cos.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    
    case!(sphere, anl::ImplicitSphere::new_2d(0.3, 0.3),
	    {
		    sphere.set_radius_value(0.3);
	    }
    );
    
    case!(tiers, anl::ImplicitTiers::with_num_tiers(3, false),
	    {
		    tiers.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    
    case!(triangle, anl::ImplicitTriangle::with_period_offset(0.2, 0.1),
	    {
		    triangle.set_source_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
	    }
    );
    
    case!(rgba_blend, anl::ImplicitExtractRgbaChannel::with_channel(anl::ExtractChannel::Red),
	    {
	    	let mut m = anl::RgbaBlend::new();
	    	m.set_control_module(Rc::new(RefCell::new(anl::ImplicitGradient::new())));
		    rgba_blend.set_source_module(Rc::new(RefCell::new(m)));
	    }
    );
    case!(rgba_blend_ops, anl::ImplicitExtractRgbaChannel::with_channel(anl::ExtractChannel::Green),
	    {
	    	let m = anl::RgbaBlendOps::with_modes(BlendOps::OneMinusSrc1Alpha, BlendOps::Src1Alpha);
		    rgba_blend_ops.set_source_module(Rc::new(RefCell::new(m)));
	    }
    );    

    println!("\nPlease visit the 'target' folder for the results");
}

fn write_image(name: &str, module: &mut ImplicitModule) {
    println!("Writing '{}'", name);

    let mut imgbuf = image::ImageBuffer::new(SIZE, SIZE);

    for x in 0..SIZE {
        for y in 0..SIZE {
            let bright = ((module.get_2d(x as f64 / SIZE as f64, y as f64 / SIZE as f64) / 2.0 + 0.5) * ::std::u8::MAX as f64) as u8;
            imgbuf.put_pixel(x, y, image::Rgba([bright, bright, bright, 0xff]));
        }
    }

    let ref mut fout = File::create(format!("./target/{}.png", name)).unwrap();

    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
}
