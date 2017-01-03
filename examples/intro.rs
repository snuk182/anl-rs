extern crate anl;
extern crate image;

use anl::*;
use anl::random_gen::PRNG;
use anl::RgbaModule;

use std::fs::*;
use std::rc::Rc;
use std::cell::RefCell;

const SIZE: u32 = 800;
const NAME: &'static str = "intro";

fn main() {
	println!("Initializing '{}'", NAME);

	let mut rnd = anl::random_gen::CMWC4096::new();
	anl::random_gen::set_seed_time(&mut rnd);
	
	let frac1 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac2 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac3 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac4 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::RidgedMulti, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac5 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac6 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	let frac7 = Rc::new(RefCell::new(anl::ImplicitFractal::new(anl::FractalType::FBM, anl::BasisType::Gradient, anl::InterpType::Quintic)));
	
	frac1.borrow_mut().set_seed(rnd.get());
	frac2.borrow_mut().set_seed(rnd.get());
	frac3.borrow_mut().set_seed(rnd.get());
	frac4.borrow_mut().set_seed(rnd.get());
	frac5.borrow_mut().set_seed(rnd.get());
	frac6.borrow_mut().set_seed(rnd.get());
	frac7.borrow_mut().set_seed(rnd.get());
	
	let ac1 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(0.0, 1.0)));
	let ac2 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(0.0, 1.0)));
	let ac3 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(0.0, 1.0)));
	let ac4 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(0.0, 360.0)));
	let ac5 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(-1.0, 1.0)));
	let ac6 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(-1.0, 1.0)));
	let ac7 = Rc::new(RefCell::new(anl::ImplicitAutoCorrect::with_range(-1.0, 1.0)));
	
	ac1.borrow_mut().set_source(Some(frac1.clone()));
	ac2.borrow_mut().set_source(Some(frac2.clone()));
	ac3.borrow_mut().set_source(Some(frac3.clone()));
	ac4.borrow_mut().set_source(Some(frac4.clone()));
	ac5.borrow_mut().set_source(Some(frac5.clone()));
	ac6.borrow_mut().set_source(Some(frac6.clone()));
	ac7.borrow_mut().set_source(Some(frac7.clone()));
	
	let compose1 = Rc::new(RefCell::new(anl::RgbaComposeChannels::with_mode(anl::CompositeChannelsMode::Rgb)));
	compose1.borrow_mut().set_red_or_hue_module(ac1.clone());
	compose1.borrow_mut().set_green_or_saturation_module(ac2.clone());
	compose1.borrow_mut().set_blue_or_value_module(ac3.clone());
	compose1.borrow_mut().set_alpha_value(1.0);
	
	let rot = Rc::new(RefCell::new(anl::RgbaRotateColor::with_axises_angles(0.0, 0.0, 0.0, 0.0)));
	rot.borrow_mut().set_degree_module(ac4.clone());
	rot.borrow_mut().set_ax_module(ac5.clone());
	rot.borrow_mut().set_ay_module(ac6.clone());
	rot.borrow_mut().set_az_module(ac7.clone());
	rot.borrow_mut().set_normalize_axis(true);
	rot.borrow_mut().set_source_module(compose1.clone());
	
	println!("Writing '{}'", NAME);

    let mut imgbuf = image::ImageBuffer::new(SIZE, SIZE);

    for x in 0..SIZE {
        for y in 0..SIZE {
        	let rgba = compose1.borrow_mut().get_2d(x as f64 / SIZE as f64, y as f64 / SIZE as f64);
            imgbuf.put_pixel(x, y, image::Rgba([
            		(rgba.r() * ::std::u8::MAX as f32) as u8, 
	            	(rgba.g() * ::std::u8::MAX as f32) as u8, 
		            (rgba.b() * ::std::u8::MAX as f32) as u8, 
		        	(rgba.a() * ::std::u8::MAX as f32) as u8]));
        }
    }

    let ref mut fout = File::create(format!("./target/{}.png", NAME)).unwrap();

    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
    
    println!("\nPlease visit the 'target' folder for the results");
}
