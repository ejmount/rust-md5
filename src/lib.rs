//use std;
use std::slice::*;

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

const BLOCK_SIZE : usize = 64;


#[derive(Copy)]
pub struct Md5 {
		a0 : u32, 
		b0 : u32, 
		c0 : u32, 
		d0 : u32,
		end_of_buffer : usize,
		length_so_far : usize,
		buffer : [u8; BLOCK_SIZE],
}

struct StateVector {
		a : u32,
	    b : u32,
	    c : u32,
	    d : u32,
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
			buffer: [0; BLOCK_SIZE],
			end_of_buffer: 0,
			length_so_far: 0,
		}
	}

	pub fn update(&mut self, bytes : &[u8]) -> &mut Md5 {

		let remaining_space = BLOCK_SIZE - self.end_of_buffer;
		let bytes_length = bytes.len();

		if remaining_space > bytes_length {			
			let ptr = &mut self.buffer[BLOCK_SIZE-remaining_space.. BLOCK_SIZE-remaining_space+bytes.len()];
			ptr.clone_from_slice(&bytes[0..bytes_length]);
			self.length_so_far += bytes_length;
			self.end_of_buffer += bytes_length;
		}
		else{
			{
				let ptr = &mut self.buffer[BLOCK_SIZE - remaining_space .. BLOCK_SIZE];
				ptr.clone_from_slice(&bytes[0..remaining_space]);
			}
			self.end_of_buffer = 0;
			self.length_so_far += remaining_space;
			self.compute_block(None);

			let total_blocks = (bytes_length - remaining_space) / 64; 

			for i in 0..total_blocks {
				let start_index = i*BLOCK_SIZE + remaining_space;
				self.compute_block(Some(&bytes[start_index..start_index+BLOCK_SIZE]));
				self.length_so_far += BLOCK_SIZE;
			}

			let leftover = (bytes_length - remaining_space) % 64;
			let buffer_ptr = &mut self.buffer[0..leftover];
			buffer_ptr.clone_from_slice(&bytes[bytes_length-leftover..bytes_length]);
			self.end_of_buffer += leftover;
			self.length_so_far += leftover;
		}
		return self
	}

	fn compute_block(&mut self, input : Option<&[u8]>) {

		let data : &[u8] = match input {
			Some(a) => a,
			None => &self.buffer
		};

		let mut m : [u32; 16] = [0;16];

		for i in 0..16 {
			m[i] = from_le_bytes(data[4*i+0], data[4*i+1], data[4*i+2], data[4*i+3]);
		}

		let m = m;

		let mut f : u32;
		let mut g : u32;

		let mut sv = StateVector{
				a:self.a0,
				b:self.b0,
				c:self.c0,
				d:self.d0,
		};

		for i in 0..16 {
			f = (sv.b & sv.c) | (!sv.b & sv.d); 
		    g = i;
    		Md5::update_state(&mut sv, &m, i, f, g);
		}

		for j in 0..16 {
			let i = j+16;
			f = (sv.d & sv.b) | (!sv.d & sv.c);
           	g = (5*i + 1) % 16;
    		Md5::update_state(&mut sv, &m, i, f, g);
		}

		for j in 0..16 {
			let i = j+32;
			f = sv.b ^ sv.c ^ sv.d;
            g = (3*i + 5) % 16;
    		Md5::update_state(&mut sv, &m, i, f, g);
		}

		for j in 0..16 {
			let i = j+48;
			f = sv.c ^ (sv.b | !sv.d);
            g = (7*i) % 16;
         	Md5::update_state(&mut sv, &m, i, f, g);   
		}
	    self.a0 = self.a0.wrapping_add(sv.a);
	    self.b0 = self.b0.wrapping_add(sv.b);
	    self.c0 = self.c0.wrapping_add(sv.c);
	    self.d0 = self.d0.wrapping_add(sv.d);
	}

	#[inline(always)]
	fn update_state(sv : &mut StateVector, m : &[u32; 16], i : u32, f : u32, g : u32) {
		let d_temp = sv.d;
        sv.d = sv.c;
        sv.c = sv.b;
        sv.b = sv.b.wrapping_add(sv.a.wrapping_add(f).wrapping_add(K[i as usize]).wrapping_add(m[g as usize]).rotate_left(S[i as usize]));
        sv.a = d_temp;
	}

	fn complete_data(&self) -> Md5 {

		let mut temp = self.clone();

		temp.update(&[0b10000000u8]);

		let mod512 = temp.end_of_buffer;	
		let padding_length = 
			if mod512 > 56 
				{ 64-mod512+56 } 
			else 	
				{ 56 - mod512 };

		for _ in 0..padding_length {
			temp.update(&[0]);
		}

		let size  = 8*self.length_so_far as u64;
		let mut size_bytes : [u8; 8] = [0; 8];
		for i in 0..8 {
			size_bytes[i] = (size.wrapping_shr(8*(i as u32)) % 256) as u8;
		}
		temp.update(&size_bytes);

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

#[inline(always)]
fn to_le_bytes(i: &u32) -> (u8, u8, u8, u8) {
	let q1 = (i >> 24u32) % 256;
	let q2 = (i >> 16u32) % 256;
	let q3 = (i >> 8u32)  % 256;
	let q4 = (i >> 0u32)  % 256;
	return (q4 as u8, q3 as u8, q2 as u8, q1 as u8)
}

#[inline(always)]
fn from_le_bytes(a: u8, b:u8, c:u8, d:u8) -> u32 {
	return (a as u32) + ((b as u32) << 8) + ((c as u32) << 16) + ((d as u32) << 24);
}

mod tests {

	use super::*;

	#[test]
	fn empty_object_test() {

		let mut hash = Md5::new();
		let vals = vec![];
		hash.update(&vals);
		let trueans = "D41D8CD98F00B204E9800998ECF8427E";
		let ans = hash.hexdigest();

		assert!(to_hex_string(&ans) == trueans);
	}

	#[test]
	fn seven_hundred_as() {
		let mut hash = Md5::new();
		let a = 'A' as u8;
		let vals = vec![a; 700];
		hash.update(&vals);
		let trueans = "C04C6D6896853D32B720D69A6027E6BE";
		let ans = hash.hexdigest();

		assert!(to_hex_string(&ans) == trueans);	
	}

	#[test]
	fn associativity_test() {

		let mut hash = Md5::new();
		let vals = vec![b'B'; 32];
    	let mut vals2 = Vec::new();
	    vals2.resize(64, b'A');

	    hash.update(&vals);
	    hash.update(&vals2);
	    
	    let mut vals3 = vec![b'A'; 32+64];
	    for i in 0..32 {
	        vals3[i]=b'B';
	    }
	    
	    let mut secondhash = Md5::new();
	    secondhash.update(&vals3);

		let ans = hash.hexdigest();
		let ans2 = secondhash.hexdigest();
		assert!(to_hex_string(&ans) == to_hex_string(&ans2));
	}

}
