use std::mem;

fn main() {
	let now = std::time::SystemTime::now();
	let r = md5("ugkcyxxp", false);
	println!("1 ({}s) {}", now.elapsed().unwrap().as_secs(), r);

	let now = std::time::SystemTime::now();
	let r = md5("ugkcyxxp", true);
	println!("2 ({}s) {}", now.elapsed().unwrap().as_secs(), r);
}

// implementation originally from https://rosettacode.org/wiki/MD5/Implementation#Rust
fn md5(msg: &str, complex: bool) -> String {
	let mut bytes: Vec<u8> = vec![0; 64];
	for (i, b) in msg.bytes().enumerate() {
		bytes[i] = b;
	}
	let startix = msg.len();
	let mut output: Vec<Option<u8>> = vec![None; 8];

	let mut counter: u32 = 0;
	let mut hits = 0usize;

	while hits < 8 {
		counter += 1;
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
		let mut X = unsafe { mem::transmute::<&mut [u8], &mut [u32]>(&mut bytes) };
		#[cfg(target_endian = "big")]
		for j in 0..16 {
			X[j] = X[j].swap_bytes();
		}

		/* Save Registers A,B,C,D */
		let aa = A;

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

		if complex {
			if let Some((i, c)) = extract_complex(A.swap_bytes()) {
				if output[i].is_none() {
					output[i] = Some(c);
					println!(".");
					hits += 1;
				}
			}
		} else if let Some(c) = extract_simple(A.swap_bytes()) {
			output[hits] = Some(c);
			println!(".");
			hits += 1;
		}
	}

	output.iter().map(|c| u8_to_char(c.unwrap())).collect()
}

const LOOKUP: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

fn u8_to_char(data: u8) -> char {
	LOOKUP[(data & 0xf) as usize]
}

fn extract_simple(data: u32) -> Option<u8> {
	if data & 0xffff_f000 == 0 {
		Some(((data & 0xf00) >> 8) as u8)
	} else {
		None
	}
}

fn extract_complex(data: u32) -> Option<(usize, u8)> {
	if data & 0xffff_f000 == 0 {
		let ix: u8 = ((data & 0xf00) >> 8) as u8;
		if ix <= 7 {
			return Some((ix as usize, ((data & 0xf0) >> 4) as u8));
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extractcomplex() {
		assert_eq!(None,extract_complex(0x00001234));
		assert_eq!(Some((2,3)),extract_complex(0x00000234));
	}

	#[test]
	fn test_extractsimple() {
		assert_eq!(None,extract_simple(0x00001234));
		assert_eq!(Some(2),extract_simple(0x00000234));
	}

	#[test]
	fn test_u8charlookup() {
		assert_eq!('0', u8_to_char(0));
		assert_eq!('f', u8_to_char(15));
	}
}