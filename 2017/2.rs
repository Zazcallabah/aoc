
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_chk(){
		let m = Sheet::new("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8");
		assert_eq!(18,m.chk());
	}
}

fn main() {
	let s = Sheet::new(&std::fs::read_to_string("2017/2.txt").unwrap());
	println!("part 1: {}",s.chk());
	println!("part 2: {}",s.div());
}

struct Sheet {
	lines : Vec<Row>,
}

struct Row {
	cells : Vec<i32>,
}

impl Row {
	fn new(data:&str) -> Row {
		Row{ cells: data.split('\t')
			.map(|s| s.parse().unwrap() )
			.collect::<Vec<i32>>()
		}
	}
	fn div(&self) -> i32 {
		for a in 0..self.cells.len()-1 {
			for b in a..self.cells.len() {
				let x = self.cells[a];
				let y = self.cells[b];

				if x > y && x % y == 0 {
					return x/y
				}
				else if y > x && y % x == 0 {
					return y/x
				}
			}
		}
		panic!("no div found");
	}
	fn chk(&self) -> i32 {
		let mut min = std::i32::MAX;
		let mut max = std::i32::MIN;

		for &c in &self.cells {
			if c < min {
				min = c;
			}
			if c > max {
				max = c;
			}
		}
		max-min
	}
}

impl Sheet {
	fn new(data:&str)->Sheet{
		let lines = data.lines().map(|l| Row::new(l) ).collect::<Vec<Row>>();
		Sheet{lines}
	}
	fn div(&self) -> i32 {
		let mut sum = 0;
		for l in &self.lines {
			sum += l.div();
		}
		sum
	}


	fn chk(&self) -> i32 {
		let mut sum = 0;
		for l in &self.lines {
			sum += l.chk();
		}
		sum
	}
}