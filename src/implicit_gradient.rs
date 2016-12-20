use super::implicit_base::ImplicitModuleBase;
use super::ImplicitModule;

pub struct ImplicitGradient {
	base: ImplicitModuleBase,
	gx1: f64, gy1: f64, gz1: f64, gw1: f64, gu1: f64, gv1: f64,
	gx2: f64, gy2: f64, gz2: f64, gw2: f64, gu2: f64, gv2: f64,
	x: f64, y: f64, z: f64, w: f64, u: f64, v: f64,
	vlen: f64
}

impl Default for ImplicitGradient {
	fn default() -> Self {
		ImplicitGradient {
			gx1: 0.0, gy1: 0.0, gz1: 0.0, gw1: 0.0, gu1: 0.0, gv1: 0.0,
			gx2: 0.0, gy2: 0.0, gz2: 0.0, gw2: 0.0, gu2: 0.0, gv2: 0.0,
			x: 0.0, y: 0.0, z: 0.0, w: 0.0, u: 0.0, v: 0.0,
			vlen: 0.0,
			base: Default::default(),
		}
	}
}

impl ImplicitGradient {
	pub fn new() -> ImplicitGradient {
		let mut g: ImplicitGradient = Default::default();
		g.set_gradient(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		g
	}
	
	pub fn set_gradient(&mut self, x1: f64, x2: f64, y1: f64, y2: f64, z1: f64, z2: f64, w1: f64, w2: f64, u1: f64, u2: f64, v1: f64, v2: f64) {
		self.gx1 = x1;
		self.gx2 = x2;
		self.gy1 = y1;
		self.gy2 = y2;
		self.gz1 = z1;
		self.gz2 = z2;
		self.gw1 = w1;
		self.gw2 = w2;
		self.gu1 = u1;
		self.gu2 = u2;
		self.gv1 = v1;
		self.gv2 = v2;
	
		self.x = x2 - x1;
		self.y = y2 - y1;
		self.z = z2 - z1;
		self.w = w2 - w1;
		self.u = u2 - u1;
		self.v = v2 - v1;
	
		self.vlen = self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w + self.u*self.u + self.v*self.v;
	}
}

impl ImplicitModule for ImplicitGradient {
	fn set_seed(&mut self, _: u32) {}

	fn get_2d(&mut self, x: f64, y: f64) -> f64 {
		// Subtract from (1) and take dotprod
		let dx = x - self.gx1;
		let dy = y - self.gy1;
		let mut dp = dx*self.x + dy*self.y;
		dp /= self.vlen;
		//dp=clamp(dp/self.vlen,0.0,1.0);
		//return lerp(dp,1.0,-1.0);
		dp
	}
	fn get_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
		let dx = x - self.gx1;
		let dy = y - self.gy1;
		let dz = z - self.gz1;
		let mut dp = dx*self.x + dy*self.y + dz*self.z;
		dp /= self.vlen;
		//dp=clamp(dp/self.vlen,0.0,1.0);
		//return lerp(dp,1.0,-1.0);
		dp
	}
	fn get_4d(&mut self, x: f64, y: f64, z: f64, w: f64) -> f64 {
		let dx = x - self.gx1;
		let dy = y - self.gy1;
		let dz = z - self.gz1;
		let dw = w - self.gw1;
		let mut dp = dx*self.x + dy*self.y + dz*self.z + dw*self.w;
		dp /= self.vlen;
		//dp=clamp(dp/self.vlen,0.0,1.0);
		//return lerp(dp,1.0,-1.0);
		dp
	}
	fn get_6d(&mut self, x: f64, y: f64, z: f64, w: f64, u: f64, v: f64) -> f64 {
		let dx = x - self.gx1;
		let dy = y - self.gy1;
		let dz = z - self.gz1;
		let dw = w - self.gw1;
		let du = u - self.gu1;
		let dv = v - self.gv1;
		let mut dp = dx*self.x + dy*self.y + dz*self.z + dw*self.w + du*self.u + dv*self.v;
		dp /= self.vlen;
		//dp=clamp(dp/self.vlen,0.0,1.0);
		//return lerp(clamp(dp,0.0,1.0),1.0,-1.0);
		dp
	}
	
	fn spacing(&self) -> f64 {
		self.base.spacing
	}
}
