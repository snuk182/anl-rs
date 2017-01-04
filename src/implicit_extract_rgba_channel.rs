/// The documentation is taken from original [C++ library by Joshua Tippetts](http://accidentalnoise.sourceforge.net/docs.html).

use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;
use super::rgba_module_base::{RgbaParameter, RgbaModule};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ExtractChannel {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
}

/// ExtractRGBAChannel accepts an ['RgbaParameter'](enum.RgbaParameter.html) and extracts one of the channels. The accepted values for channel are listed in the [`ExtractChannel`](enum.ExtractChannel.html).
pub struct ImplicitExtractRgbaChannel {
    base: ImplicitModuleBase,
    channel: ExtractChannel,
    source: RgbaParameter,
}

impl ImplicitExtractRgbaChannel {
    pub fn with_channel(channel: ExtractChannel) -> ImplicitExtractRgbaChannel {
        ImplicitExtractRgbaChannel {
            base: Default::default(),
            channel: channel,
            source: RgbaParameter::with_grey(0.0),
        }
    }

    pub fn set_source_constant(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.source = RgbaParameter::with_color(r, g, b, a);
    }

    pub fn set_source_module(&mut self, m: Rc<RefCell<RgbaModule>>) {
        self.source = RgbaParameter::with_module(m);
    }
}

impl ImplicitModule for ImplicitExtractRgbaChannel {
    fn get_2d(&mut self, x: f64, y: f64) -> f64 {
        self.source.get_2d(x, y)[self.channel as usize] as f64
    }
    fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.source.get_3d(x, y, z)[self.channel as usize] as f64
    }
    fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        self.source.get_4d(x, y, z, w)[self.channel as usize] as f64
    }
    fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
        self.source.get_6d(x, y, z, w, u, v)[self.channel as usize] as f64
    }

    fn spacing(&self) -> f64 {
        self.base.spacing
    }
    fn set_deriv_spacing(&mut self, s: f64) {
        self.base.spacing = s;
    }
}
