/// The documentation is taken from original C++ library by Joshua Tippetts ( http://accidentalnoise.sourceforge.net/docs.html ). 

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
	
	/// This example is kind of a neat one (if a trifle useless). 
	/// The first thing the example does is to create a random number generator(RNG). ANL provides a number of different RNGs encapsulated in classes, 
	/// and based on variants of RNGs devised by George Marsaglia. It seeds the generator using the system time. After that, 7 noise fractal modules are created. 
	/// Many applications won't require nearly so many, and the fewer you can use the better your performance will be. After the fractals are generated, 
	/// they are seeded from the RNG then 7 more modules of type AutoCorrect are created.

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
	
	/// AutoCorrect modules attempt to "tame" the output of a function and remap it to a different range. Some functions (mult-fractals especially) 
	/// are difficult to correct into an optimal range, and so AutoCorrect provides a way of doing it manually. When a function is set as the source to AutoCorrect, 
	/// the module will iterate some number of times, and generate that many samples from random locations in the input source. It will determine the max and min values 
	/// in the sample set, and from those will determine a scale/translate pair that is used to correct the output of the function into a desired range. 
	/// The system isn't perfect, as the random sampling might still miss the absolute max and min values of the function, but it works sufficiently well. 
	/// There is a bit of overhead in creating the function, during the calculate() phase when a source is initially set.
	/// 
	/// The AutoCorrect modules are set to specific ranges. The first three correct their inputs to the range (0,1). These will be used later to specify the Red, 
	/// Green and Blue channels of an RGBA color. The next AutoCorrect remaps its input to the range (0,360). This is used to specify a randomized rotation angle, 
	/// in degrees. The final three remap their inputs to the range (-1,1). These will be used to define a 3D rotation axis in RGBA color space.
	/// 
	/// Once the AutoCorrect modules are constructed, their sources are set from the fractals created earlier. Now, when the get() methods of the AutoCorrect modules 
	/// are called, they will first call their respective source module for a value, then apply their scale/translate pairs to "fix" the value before returning it.
	
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
	
	/// The next function created is an RGBA function, CompositeChannels. This function accepts up to 4 inputs from Implicit modules, one each for the channels 
	/// of Red, Green, Blue and Alpha. (Or Hue, Saturation, Value and Alpha, if the function is operating in HSV space, settable via the mode parameter in the constructor.) 
	/// This function represents how input parameters to functions work. If a function takes an Implicit input, the input may be over-ridden to a constant double-
	/// precision value, or it may be set to the output of another Implicit function. 
	///
	/// In this case, the output of the function would have been a constant value (magenta). Any combination of constants and functional inputs may be used. In our case, 
	/// we are using functions for the Red, Green and Blue channels, and a constant value of 1.0 for Alpha. 
	
	let compose1 = Rc::new(RefCell::new(anl::RgbaComposeChannels::with_mode(anl::CompositeChannelsMode::Rgb)));
	compose1.borrow_mut().set_red_or_hue_module(ac1.clone());
	compose1.borrow_mut().set_green_or_saturation_module(ac2.clone());
	compose1.borrow_mut().set_blue_or_value_module(ac3.clone());
	compose1.borrow_mut().set_alpha_value(1.0);
	
	/// snuk182 - ATTENTION. The RotateColor here does not work as intended and is not used in this example. Should be a bug in a Rust version.
	///
	/// Now, next we create an RGBA module of type RotateColor. This function takes an RGBA input, and 4 Implicit inputs. 
	/// The Implicit inputs represent the (ax,ay,az,angle) components of an axis/angle rotation. The quantity being rotated is the RGBA color, in colorspace. 
	/// Each color channel is first re-mapped to (-1,1), then a matrix is contructed from the axis/angle rotation. The components are transformed by this matrix, 
	/// then re-normalized to (0,1) and output. For the inputs, we set our ComposeChannels function as the RGBA input, and our remaining 4 fractal modules 
	/// for the (ax,ay,az,angle) inputs. Again, as before, it is just as easy to use double-precision constants for these inputs, and it is also as easy to specify 
	/// an RGBA constant for the RGBA input, rather than the compose function. 
	
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
    
    /// To recap, we compose an RGBA from 3 channels of fractal noise, then rotate the color around an axis represented by 3 more channels of fractal noise, 
    /// rotating by an angle specified by a final channel of fractal noise. The results are pretty, but also pretty useless. However, it does demonstrate the basic idea 
    /// of the library.
    
    println!("\nPlease visit the 'target' folder for the results");
}
