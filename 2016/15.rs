
struct Disc {
	index: u32,
	start: u32,
	size: u32,
}

impl Disc {
	fn new(index:u32,size:u32,start:u32) -> Disc {
		Disc{index,size,start}
	}

	fn init() -> Vec<Disc> {
		vec![
			Disc::new(1,13,1),
			Disc::new(2,19,10),
			Disc::new(3,3,2),
			Disc::new(4,7,1),
			Disc::new(5,5,3),
			Disc::new(6,17,5),
		]
	}

	fn test() -> Vec<Disc> {
		vec![
			Disc::new(1,5,4),
			Disc::new(2,2,1),
		]
	}

	fn get_position(&self, time: u32)->u32 {
		(self.start + self.index + time) % self.size
	}
}

fn get_first(discs:&[Disc]) -> u32 {
	for time in 0.. {
		if discs.iter().all(|d| d.get_position(time) == 0 ) {
			return time
		}
	}
	panic!("found end of ints");
}

fn main() {
	let mut discs = Disc::init();

	println!("first {}",get_first(&discs));

	discs.push(Disc{index:7,size:11,start:0});
	println!("second {}",get_first(&discs));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getpos(){
		let t = Disc::test();
		assert_eq!(0,t[0].get_position(0));
		assert_eq!(1,t[1].get_position(0));
		assert_eq!(0,t[0].get_position(5));
		assert_eq!(0,t[1].get_position(5));
	}
}
