extern crate core;
use std::slice::Iter;

const K : [u32; 64] = [
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
	0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
	0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
	0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
	0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
	0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
	0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
	0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 
];

const S : [u32; 64] = 
[
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 
	17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 
	9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 
	11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 
	4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 
	6, 10, 15, 21, 6, 10, 15, 21 
];



pub struct Md5 {
		a0 : u32, 
		b0 : u32, 
		c0 : u32, 
		d0 : u32,
		end_of_buffer : usize,
		length_so_far : u64,
		buffer : [u8; 64],
}

impl Clone for Md5 {
    fn clone(&self) -> Md5 {
    		return Md5 {
				a0: self.a0,
				b0: self.b0, 
				c0: self.c0,
				d0: self.d0,
				buffer: self.buffer,
				end_of_buffer: self.end_of_buffer,
				length_so_far: self.length_so_far,
			}
    }
}


impl Md5 {
	pub fn new() -> Md5 {
		return Md5 {
			a0: 0x67452301, 
			b0: 0xefcdab89,
			c0: 0x98badcfe,
			d0: 0x10325476,
			buffer: [0; 64],
			end_of_buffer: 0,
			length_so_far: 0,
		}
	}

	pub fn update(&mut self, bytes : Iter<u8>) -> &Md5 {
		for v in bytes {
			self.buffer[self.end_of_buffer] = *v;
			self.length_so_far += 1;
			self.end_of_buffer += 1;
			if self.end_of_buffer >= self.buffer.len() {
				self.compute_block();
				self.end_of_buffer = 0;
			}
		}
		return self;
	}

	fn compute_block(&mut self) {
		use core::ops::Not;		

		let mut m : [u32; 16] = [0;16];

		for (i, bits) in self.buffer.chunks(4).enumerate() {
			m[i] = from_le_bytes(bits[0], bits[1], bits[2], bits[3])
		}

		let m = m;

	    let mut a = self.a0;
	    let mut b = self.b0;
	    let mut c = self.c0;
	    let mut d = self.d0;
		let mut f : u32;
		let mut g : u32;
		//main loop:
	    for i in 0..64 {
		    match i {
		    	0 ... 15 => { 
		    		f = (b & c) | (b.not() & d); 
		    		g = i;
		    	}
		        16...31 => {
		            f = (d & b) | (d.not() & c);
		            g = (5*i + 1) % 16;
		        }
		        32...47 => {
		            f = b ^ c ^ d;
		            g = (3*i + 5) % 16;
		        }
		        48...63 => {
		            f = c ^ (b | d.not());
		            g = (7*i) % 16;
		        }
		        _ => panic!("Out of bounds")
		    }

	        let d_temp = d;
	        d = c;
	        c = b;
	        b = b.wrapping_add(leftrotate(a.wrapping_add(f).wrapping_add(K[i as usize]).wrapping_add(m[g as usize]), S[i as usize]));
	        a = d_temp;
	    }
	    self.a0 = self.a0.wrapping_add(a);
	    self.b0 = self.b0.wrapping_add(b);
	    self.c0 = self.c0.wrapping_add(c);
	    self.d0 = self.d0.wrapping_add(d);
	}


	fn complete_data(&self) -> Md5 {

		let mut temp = self.clone();

		temp.update([0b10000000 as u8].iter());

		let mod512 = temp.end_of_buffer;	
		let padding_length = 
			if mod512 > 56 
				{ 64-mod512+56 } 
			else 	
				{ 56 - mod512 };

		for _ in 0..padding_length {
			temp.update([0 as u8].iter());
		}

		let size : u64 = 8*self.length_so_far as u64;
		let mut size_bytes : [u8; 8] = [0; 8];
		for i in 0..8 {
			size_bytes[i as usize] = (size.wrapping_shr(8*i) % 256) as u8;
		}
		temp.update(size_bytes.iter());

		return temp;
	}


	pub fn hexdigest(&self) -> [u8;16] {

		let completedhash = self.complete_data();

		let mut result = [0u8; 16];
		for (index, v) in [completedhash.a0, completedhash.b0, completedhash.c0, completedhash.d0].iter().enumerate() {
			let bytes = to_le_bytes(v);
			result[4*index] = bytes.0;
			result[4*index+1] = bytes.1;
			result[4*index+2] = bytes.2;
			result[4*index+3] = bytes.3;
		}

		return result;
	}
}


pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
  strs.join("")
}

fn to_le_bytes(i: &u32) -> (u8, u8, u8, u8) {
	let q1 = (i >> 24u32) % 256;
	let q2 = (i >> 16u32) % 256;
	let q3 = (i >> 8u32)  % 256;
	let q4 = (i >> 0u32)  % 256;
	return (q4 as u8, q3 as u8, q2 as u8, q1 as u8)
}

fn from_le_bytes(a: u8, b:u8, c:u8, d:u8) -> u32 {
	return (a as u32) + ((b as u32) << 8) + ((c as u32) << 16) + ((d as u32) << 24);
}

fn leftrotate (x : u32, c: u32) -> u32 {
	    return x.rotate_left(c);
}

mod tests {

	#[test]
	fn empty_object_test() {

		let mut hash = Md5::new();
		let vals = vec![];
		hash.update(vals.iter()	);
		let trueans = "D41D8CD98F00B204E9800998ECF8427E";
		let ans = hash.hexdigest();

		assert!(to_hex_string(&ans) == trueans);
	}

	#[test]
	fn seven_hundred_as() {
		let mut hash = Md5::new();
		let a = 'A' as u8;
		let vals = vec![a; 700];
		hash.update(vals.iter()	);
		let trueans = "C04C6D6896853D32B720D69A6027E6BE";
		let ans = hash.hexdigest();

		assert!(to_hex_string(&ans) == trueans);	
	}
}