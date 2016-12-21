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
