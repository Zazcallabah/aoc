fn main() {
	let result = md5("zpqevtbw",64,false);
	println!("64th key : {:?}",result[63]);
	let result = md5("zpqevtbw",64,true);
	println!("\n64th key stretched : {:?}",result[63]);
}

use std::io::Write;

#[derive(Debug,Copy,Clone)]
struct Md5Data {
	index: u32,
	md5: u128,
	bitfield3: u16,
	bitfield5: u16,
}

impl Md5Data {
	fn make(md5:u128,index:u32) -> Md5Data {
		let mut bitfield = (0u16,0u16);
		for pos in (0..30).rev() {
			let a = (md5 >> (pos*4)) & 0xf;
			let b = (md5 >> ((pos+1)*4)) & 0xf;
			let c = (md5 >> ((pos+2)*4)) & 0xf;

			if a == b && b == c {
				if bitfield.1 == 0 {
					bitfield.1 |= 1u16 << a;
				}
				if pos <= 27 {
					let d = (md5 >> ((pos+3)*4)) & 0xf;
					let e = (md5 >> ((pos+4)*4)) & 0xf;
					if a == d && d == e {
						bitfield.0 |= 1u16 << a;
					}
				}
			}
		}

		let (bitfield5,bitfield3) = bitfield;

		Md5Data{index,md5,bitfield3,bitfield5}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_md5data(){
		let t = 0x34e092_3cc38887_a57bd7b1_d4f953dfu128;
		let r = Md5Data::make(t,0);

		assert_eq!(0,r.index);
		assert_eq!(t,r.md5);
		assert_eq!(0,r.bitfield5);
		assert_eq!(0b0000_0001_0000_0000,r.bitfield3);
	}

	#[test]
	fn test_bitfield_edge(){

		let t = 0xddd37f73_6db183b6_b4c186b8_7dd6236cu128;
		let r = Md5Data::make(t,0);
		assert_eq!(0b0010_0000_0000_0000,r.bitfield3);

		let t = 0xaaa092_3cc38a87_a57bd7b1_d4aab000u128;
		let r = Md5Data::make(t,0);
		assert_eq!(0b0000_0100_0000_0000,r.bitfield3);

		let t = 0x88aba092_3cc38a87_a57bd7b1_d4aab000u128;
		let r = Md5Data::make(t,0);
		assert_eq!(0b0000_0000_0000_0001,r.bitfield3);

		let t =  0xaaaaaf73_6db183b6_b487dd62_a3600000u128;
		let r = Md5Data::make(t,0);
		assert_eq!(0b0000_0100_0000_0001,r.bitfield5);
	}

	#[test]
	fn test_shiftlogic(){
		let t =  0x12345678_aaaaaaaa_aaaaaaaa_aaaaaaaau128;
		let a = (t >> (29*4)) & 0xf;
		assert_eq!(3,a);
		let a = (t >> ((29+1)*4)) & 0xf;
		assert_eq!(2,a);
		let a = (t >> ((29+2)*4)) & 0xf;
		assert_eq!(1,a);

		let a = (t >> (28*4)) & 0xf;
		assert_eq!(4,a);
		let a = (t >> ((28+1)*4)) & 0xf;
		assert_eq!(3,a);
		let a = (t >> ((28+2)*4)) & 0xf;
		assert_eq!(2,a);

		let a = (t >> (27*4)) & 0xf;
		assert_eq!(5,a);
		let a = (t >> ((27+1)*4)) & 0xf;
		assert_eq!(4,a);
		let a = (t >> ((27+2)*4)) & 0xf;
		assert_eq!(3,a);
		let a = (t >> ((27+3)*4)) & 0xf;
		assert_eq!(2,a);
		let a = (t >> ((27+4)*4)) & 0xf;
		assert_eq!(1,a);
	}

	#[test]
	fn test_md5(){
		let r = md5("abc",64, false);
		assert_eq!(64,r.len());
		assert_eq!(39,r[0].index);
		assert_eq!(0b01000000_00000000,r[0].bitfield3);
		assert_eq!(92,r[1].index);
		assert_eq!(0b00000010_00000000,r[1].bitfield3);
		assert_eq!(22728,r[63].index);
	}

	#[test]
	fn test_stretch(){
		let first = stretch("abc0",1);
		assert_eq!("577571be4de9dcce85a041ba0410f29f",first);
		assert_eq!("eec80a0c92dc8a0777c619d9bb51e910",stretch(&first,1));
		assert_eq!("16062ce768787384c81fe17a7a60c7e3",stretch(&first,2));
		assert_eq!("a107ff634856bb300138cac6568c0f24",stretch(&first,2016));
	}

	#[test]
	fn test_part2() {
		let r = md5("abc", 1, true );
		assert_eq!(10,r[0].index);
	}
}

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn stretch(data:&str, depth:usize)->String{

	let mut digest = Md5::new();
	let mut output = data.to_owned();
	for _ in 0..depth {
		digest.reset();
		digest.input_str(&output);
		output = digest.result_str();
	}
	output
}


// mod the function from day 5, originally from https://rosettacode.org/wiki/MD5/Implementation#Rust
fn md5(msg: &str, fetchcount : usize, should_stretch: bool ) -> Vec<Md5Data> {
	let mut bytes: Vec<u8> = vec![0; 64];
	for (i, b) in msg.bytes().enumerate() {
		bytes[i] = b;
	}
	let startix = msg.len();
	let mut output: Vec<Md5Data> = Vec::with_capacity(64);

	let mut cache : Vec<Md5Data> = Vec::with_capacity(1000);

	for counter in 0.. {
		if should_stretch && counter % 100 == 0 {
			print!(".");
			std::io::stdout().flush();
		}
		let counterstr = counter.to_string();
		let msglength = counterstr.len() + startix;
		for (i, b) in counterstr.bytes().enumerate() {
			bytes[startix + i] = b
		}
		bytes[msglength] = 0b1000_0000;

		let bitcount = msglength * 8;

		bytes[56] = bitcount as u8;
		bytes[57] = (bitcount >> 8) as u8;

		let mut A = 0x6745_2301u32;
		let mut B = 0xefcd_ab89u32;
		let mut C = 0x98ba_dcfeu32;
		let mut D = 0x1032_5476u32;

		let F = |X: u32, Y: u32, Z: u32| -> u32 { X & Y | !X & Z };
		let G = |X: u32, Y: u32, Z: u32| -> u32 { X & Z | Y & !Z };
		let H = |X: u32, Y: u32, Z: u32| -> u32 { X ^ Y ^ Z };
		let I = |X: u32, Y: u32, Z: u32| -> u32 { Y ^ (X | !Z) };

		/* This step uses a 64-element table T[1 ... 64] constructed from the sine function.  */
		let T = [
			0x00000000, // enable use as a 1-indexed table
			0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
			0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
			0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
			0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
			0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
			0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
			0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
			0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
			0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
			0xeb86d391,
		];

		/* Copy block into X. */
		let mut X = unsafe { std::mem::transmute::<&mut [u8], &mut [u32]>(&mut bytes) };
		#[cfg(target_endian = "big")]
		for j in 0..16 {
			X[j] = X[j].swap_bytes();
		}

		/* Save Registers A,B,C,D */
		let aa = A;
		let bb = B;
		let cc = C;
		let dd = D;

		/* Round 1.  Let [abcd k s i] denote the operation
		a = b + ((a + F(b,c,d) + X[k] + T[i]) <<< s). */
		macro_rules! op1 {
			($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
				$a = $b.wrapping_add(
					($a.wrapping_add(F($b, $c, $d))
						.wrapping_add(X[$k])
						.wrapping_add(T[$i]))
					.rotate_left($s),
					)
			};
		}

		/* Do the following 16 operations. */
		op1!(A, B, C, D, 0, 7, 1);
		op1!(D, A, B, C, 1, 12, 2);
		op1!(C, D, A, B, 2, 17, 3);
		op1!(B, C, D, A, 3, 22, 4);

		op1!(A, B, C, D, 4, 7, 5);
		op1!(D, A, B, C, 5, 12, 6);
		op1!(C, D, A, B, 6, 17, 7);
		op1!(B, C, D, A, 7, 22, 8);

		op1!(A, B, C, D, 8, 7, 9);
		op1!(D, A, B, C, 9, 12, 10);
		op1!(C, D, A, B, 10, 17, 11);
		op1!(B, C, D, A, 11, 22, 12);

		op1!(A, B, C, D, 12, 7, 13);
		op1!(D, A, B, C, 13, 12, 14);
		op1!(C, D, A, B, 14, 17, 15);
		op1!(B, C, D, A, 15, 22, 16);

		/* Round 2. Let [abcd k s i] denote the operation
		a = b + ((a + G(b,c,d) + X[k] + T[i]) <<< s). */
		macro_rules! op2 {
			($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
				$a = $b.wrapping_add(
					($a.wrapping_add(G($b, $c, $d))
						.wrapping_add(X[$k])
						.wrapping_add(T[$i]))
					.rotate_left($s),
					)
			};
		}

		/* Do the following 16 operations. */
		op2!(A, B, C, D, 1, 5, 17);
		op2!(D, A, B, C, 6, 9, 18);
		op2!(C, D, A, B, 11, 14, 19);
		op2!(B, C, D, A, 0, 20, 20);

		op2!(A, B, C, D, 5, 5, 21);
		op2!(D, A, B, C, 10, 9, 22);
		op2!(C, D, A, B, 15, 14, 23);
		op2!(B, C, D, A, 4, 20, 24);

		op2!(A, B, C, D, 9, 5, 25);
		op2!(D, A, B, C, 14, 9, 26);
		op2!(C, D, A, B, 3, 14, 27);
		op2!(B, C, D, A, 8, 20, 28);

		op2!(A, B, C, D, 13, 5, 29);
		op2!(D, A, B, C, 2, 9, 30);
		op2!(C, D, A, B, 7, 14, 31);
		op2!(B, C, D, A, 12, 20, 32);

		/* Round 3. Let [abcd k s t] denote the operation
		a = b + ((a + H(b,c,d) + X[k] + T[i]) <<< s). */
		macro_rules! op3 {
			($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
				$a = $b.wrapping_add(
					($a.wrapping_add(H($b, $c, $d))
						.wrapping_add(X[$k])
						.wrapping_add(T[$i]))
					.rotate_left($s),
					)
			};
		}

		/* Do the following 16 operations. */
		op3!(A, B, C, D, 5, 4, 33);
		op3!(D, A, B, C, 8, 11, 34);
		op3!(C, D, A, B, 11, 16, 35);
		op3!(B, C, D, A, 14, 23, 36);

		op3!(A, B, C, D, 1, 4, 37);
		op3!(D, A, B, C, 4, 11, 38);
		op3!(C, D, A, B, 7, 16, 39);
		op3!(B, C, D, A, 10, 23, 40);

		op3!(A, B, C, D, 13, 4, 41);
		op3!(D, A, B, C, 0, 11, 42);
		op3!(C, D, A, B, 3, 16, 43);
		op3!(B, C, D, A, 6, 23, 44);

		op3!(A, B, C, D, 9, 4, 45);
		op3!(D, A, B, C, 12, 11, 46);
		op3!(C, D, A, B, 15, 16, 47);
		op3!(B, C, D, A, 2, 23, 48);

		/* Round 4. Let [abcd k s t] denote the operation
		a = b + ((a + I(b,c,d) + X[k] + T[i]) <<< s). */
		macro_rules! op4 {
			($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
				$a = $b.wrapping_add(
					($a.wrapping_add(I($b, $c, $d))
						.wrapping_add(X[$k])
						.wrapping_add(T[$i]))
					.rotate_left($s),
					)
			};
		}

		/* Do the following 16 operations. */
		op4!(A, B, C, D, 0, 6, 49);
		op4!(D, A, B, C, 7, 10, 50);
		op4!(C, D, A, B, 14, 15, 51);
		op4!(B, C, D, A, 5, 21, 52);

		op4!(A, B, C, D, 12, 6, 53);
		op4!(D, A, B, C, 3, 10, 54);
		op4!(C, D, A, B, 10, 15, 55);
		op4!(B, C, D, A, 1, 21, 56);

		op4!(A, B, C, D, 8, 6, 57);
		op4!(D, A, B, C, 15, 10, 58);
		op4!(C, D, A, B, 6, 15, 59);
		op4!(B, C, D, A, 13, 21, 60);

		op4!(A, B, C, D, 4, 6, 61);
		op4!(D, A, B, C, 11, 10, 62);
		op4!(C, D, A, B, 2, 15, 63);
		op4!(B, C, D, A, 9, 21, 64);

		A = A.wrapping_add(aa);
		B = B.wrapping_add(bb);
		C = C.wrapping_add(cc);
		D = D.wrapping_add(dd);

		let md5val = (A.swap_bytes() as u128) << 96 | (B.swap_bytes() as u128) << 64 | (C.swap_bytes() as u128) << 32 | D.swap_bytes() as u128;

		let info = if should_stretch {
			let md5str = format!("{:032x}",md5val);
			let md5stretched = stretch( &md5str, 2016 );
			let md5stretchedint = u128::from_str_radix(&md5stretched,16).unwrap();
			Md5Data::make(md5stretchedint,counter)
		}
		else {
			Md5Data::make(md5val,counter)
		};

		if cache.len() < 1000 {
			cache.push(info);
		}
		else {
			let cacheix = (counter as usize) % cache.len();
			let potential = cache[cacheix];
			cache[cacheix] = info;

			if potential.bitfield3 != 0 && cache.iter().any(|m| m.bitfield5 & potential.bitfield3 > 0) {
				output.push(potential);
				if output.len() == fetchcount {
					return output
				}
			}
		}
	}
	panic!("found end of time");
}
