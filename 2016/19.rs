// 1
// 1x 1
// 123 1x3 xx3
// 1234 1x3x 1xxx
// 12345 1x3x5 xx3xx
// 123456 1x3x5x xxxx5x
// 1234567 1x3x5x7 xx3x5x7 xxxxxx7
// 12345678 1x3x5x7x 1xxxxxxx
// 123456789 1x3x5x7x9 xx3x5x7x9 xx3xxxxxx

// hypothesis, grouped by uneven numbered, with the series length doubling each time

// 1
// 1 3
// 1 3 5 7
// 1 3 5 7 9 11 13 15

// so if we get the power of two preceding our total number via pow(2,floor(log(n,2))) [there is definitely an off-by-one in there somewhere]
// our winning elf is 2(n - p)+1

struct Verificator {
	v : Vec<Option<u8>>
}

impl std::fmt::Display for Verificator {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let row = 	self.v.iter()
			.map(|b| if let Some(c) = b { 48u8+c } else { 46 } )
			.map(|b| if b > 57 { b+7 } else { b })
			.collect::<Vec<u8>>();

		writeln!(f, "{}", String::from_utf8(row).unwrap() )
	}
}

impl Verificator {
	fn new(size:u32)->Verificator{
		let sizeu : u8 = size as u8;
		Verificator{ v: (1..=sizeu).into_iter().map(|i| Some(i) ).collect::<Vec<Option<u8>>>() }
	}

	fn calculate(&mut self) -> u8 {
		let mut count = self.v.len();
		let mut kill_next = false;
		let mut last = 0u8;

		for ix in 0.. {
			let current = ix % self.v.len();
			if kill_next {
				if self.v[current].is_some() {
					self.v[current] = None;
					kill_next = false;
					count -= 1;
				}
			}
			else {
				if self.v[current].is_some() {
					kill_next = true;
					last = self.v[current].unwrap();
				}

			}
			if count <= 1 {
				break
			}
		}
		last
	}
}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: u32) -> u32 {
    assert!(x > 0);
    num_bits::<u32>() as u32 - x.leading_zeros() - 1
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_verificator() {
		assert_eq!(1, Verificator::new(2).calculate());
		assert_eq!(3, Verificator::new(3).calculate());
		assert_eq!(1, Verificator::new(4).calculate());
		assert_eq!(3, Verificator::new(5).calculate());
		assert_eq!(5, Verificator::new(6).calculate());
		assert_eq!(7, Verificator::new(7).calculate());
		assert_eq!(1, Verificator::new(8).calculate());
		assert_eq!(3, Verificator::new(9).calculate());
	}

	#[test]
	fn test_log2() {
		assert_eq!(1, log_2(2));
		assert_eq!(10, log_2(1024));
		assert_eq!(5, log_2(32));
		assert_eq!(4, log_2(31));
	}

	#[test]
	fn test_getseries() {
		assert_eq!(0, get_series(1));
		assert_eq!(1, get_series(2));
		assert_eq!(1, get_series(3));
		assert_eq!(2, get_series(7));
		assert_eq!(3, get_series(8));
	}

	#[test]
	fn test_part1() {
		assert_eq!(1842613,get_winner(3018458));
	}

	#[test]
	fn testverify(){
		for i in 1..100 {
			verify(i);
		}
	}

	fn verify(n:u32) {
		let mut v = Verificator::new(n);
		assert_eq!(v.calculate() as u32,get_winner(n));
	}
}

fn get_series(n:u32)->u32{
	log_2(n)
}

fn get_winner(n:u32)->u32{
	let pow = 2u32.pow( get_series(n) );
	2 * (n - pow) + 1
}
