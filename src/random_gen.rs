extern crate time;

use std::sync::RwLock;

pub trait PRNG {
	fn get(&mut self) -> u32;
	fn set_seed(&mut self, seed: u32);
}

pub fn set_seed_time(prng: &mut PRNG) {
	prng.set_seed((time::precise_time_ns() as f64 / 1000.0) as u32);
}

pub fn get_target(prng: &mut PRNG, t: u32) -> u32 {
	let v = get_01(prng);
	(v * t as f64) as u32
}

pub fn get_range (prng: &mut PRNG, low: u32, high: u32) -> u32 {
	let (low, high) = if high < low {
		(high, low)
	} else {
		(low, high)
	};
	let rg = ((high - low) + 1) as f64;
	let val = low as f64 + get_01(prng) * rg;
	val as u32
}

pub fn get_01(prng: &mut PRNG) -> f64 {
	prng.get() as f64 / 4294967295f64
}

pub
struct LCG {
	m_state: u32,
}

impl LCG {
	pub fn new() -> LCG {
		let mut lcg = LCG {
			m_state: 0,
		};
		
		lcg.set_seed(10000);
		lcg
	}
}

impl PRNG for LCG {
	fn get(&mut self) -> u32 {
		self.m_state = 69069*self.m_state + 362437;
		self.m_state
	}
	fn set_seed(&mut self, seed: u32) {
		self.m_state = seed;
	}
}

// Setup a static, global LCG for seeding other generators.
lazy_static! {
	static ref _LCG: RwLock<LCG> = RwLock::new(LCG::new());
}

// The following generators are based on generators created by George Marsaglia
// They use the static lcg created above for seeding, to initialize various
// state and tables. Seeding them is a bit more involved than an LCG.
pub
struct Xorshift {
	m_x: u32,
	m_y: u32, 
	m_z: u32, 
	m_w: u32, 
	m_v: u32,
}

impl Xorshift {
	pub fn new() -> Xorshift {
		let mut x = Xorshift {
			m_x: 0,
			m_y: 0, 
			m_z: 0, 
			m_w: 0, 
			m_v: 0,
		};
		
		x.set_seed(10000);
		x
	}
}

impl PRNG for Xorshift {
	fn get(&mut self) -> u32 {
		let t = self.m_x ^ (self.m_x >> 7);
		self.m_x = self.m_y;
		self.m_y = self.m_z;
		self.m_z = self.m_w;
		self.m_w = self.m_v;
		self.m_v = (self.m_v ^ (self.m_v << 6)) ^ (t ^ (t << 13));
		(self.m_y + self.m_y + 1) * self.m_v
	}
	fn set_seed(&mut self, seed: u32) {
		let mut lcg = _LCG.write().unwrap();
		lcg.set_seed(seed);
		self.m_x = lcg.get();
		self.m_y = lcg.get();
		self.m_z = lcg.get();
		self.m_w = lcg.get();
		self.m_v = lcg.get();
	}
}

pub
struct MWC256 {
	m_q: [u32; 256],
	c:   u32,
}

lazy_static! {
	static ref _MWC256: RwLock<usize> = RwLock::new(255);
}

impl MWC256 {
	pub fn new() -> MWC256 {
		let mut m = MWC256 {
			m_q: [0; 256],
			c: 0,
		};
		m.set_seed(10000);
		m
	}
}

impl PRNG for MWC256 {
	fn get(&mut self) -> u32 {
	    let t: u64;
	    let a = 809430660u64;
	    
	    let mut i = _MWC256.write().unwrap();
	    
        t = a * self.m_q[*i] as u64 + self.c as u64; 
        *i += 1;
        self.c = (t >> 32) as u32;
        self.m_q[*i] = t as u32;
        self.m_q[*i]
	}
	fn set_seed(&mut self, seed: u32) {
		let mut lcg = _LCG.write().unwrap();
		lcg.set_seed(seed);
		for i in 0..256 {
			self.m_q[i] = lcg.get();
		}
		self.c = get_target(&mut *lcg, 809430660);
	}
}

pub
struct CMWC4096 {
	m_q: [u32; 4096],
	c:   u32,
}

lazy_static! {
	static ref _CMWC4096: RwLock<usize> = RwLock::new(2095);
}

impl CMWC4096 {
	pub fn new() -> CMWC4096 {
		let mut m = CMWC4096 {
			m_q: [0; 4096],
			c: 0,
		};
		m.set_seed(10000);
		m
	}
}

impl PRNG for CMWC4096 {
	fn get(&mut self) -> u32 {
		let mut t: u64;
		let a = 18782u64;
		let b = 4294967295u64;
		
        let r = b-1;
        
        let mut i = _CMWC4096.write().unwrap();

        *i = (*i+1) & 4095;
        t = a * self.m_q[*i] as u64 + self.c as u64;
        self.c = (t >> 32) as u32; 
        t = (t & b) + self.c as u64;
        if t > r { 
        	self.c += 1; 
        	t = t - b;
        }
        self.m_q[*i] = (r - t) as u32;
        self.m_q[*i]
	}
	
	fn set_seed(&mut self, seed: u32) {
		let mut lcg = _LCG.write().unwrap();
		lcg.set_seed(seed);
		for i in 0..4096 {
			self.m_q[i] = lcg.get()
		}
	
		self.c = get_target(&mut *lcg, 18781)
	}
}

pub
struct KISS {
	z: u32,
	w: u32, 
	jsr: u32, 
	jcong: u32,
}

impl KISS {
	pub fn new() -> KISS {
		let mut k = KISS {
			z: 0,
			w: 0, 
			jsr: 0, 
			jcong: 0,
		};
		
		k.set_seed(10000);
		k
	}
}

impl PRNG for KISS {
	fn get(&mut self) -> u32 {
		self.z = 36969*(self.z&65535) + (self.z >> 16);
		self.w = 18000*(self.w&65535) + (self.w >> 16);
		let mwc = (self.z << 16) + self.w;
	
		self.jcong = 69069*self.jcong + 1234567;
	
		self.jsr ^= self.jsr << 17;
		self.jsr ^= self.jsr >> 13;
		self.jsr ^= self.jsr << 5;
	
		(mwc ^ self.jcong) + self.jsr		
	}
	fn set_seed(&mut self, seed: u32) {
		let mut lcg = _LCG.write().unwrap();
		lcg.set_seed(seed);
		self.z = lcg.get();
		self.w = lcg.get();
		self.jsr = lcg.get();
		self.jcong = lcg.get();
	}
}
