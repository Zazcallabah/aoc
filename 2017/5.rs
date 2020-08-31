fn count( mut prg: Vec<i32> ) -> u32 {
	let mut px = 0isize;
	let mut counter = 0;

	while px >= 0 && px < prg.len() as isize {
		let jmp = prg[px as usize];
		prg[px as usize] += 1;
		counter += 1;
		px += jmp as isize;
	}
	return counter;
}
fn count2( mut prg: Vec<i32> ) -> u32 {
	let mut px = 0isize;
	let mut counter = 0;

	while px >= 0 && px < prg.len() as isize {
		let jmp = prg[px as usize];
		if jmp >= 3 {
			prg[px as usize] -= 1;
		}
		else {
			prg[px as usize] += 1;
		}
		counter += 1;
		px += jmp as isize;
	}
	return counter;
}

fn main() {
	let data = std::fs::read_to_string("./2017/5.txt").unwrap();

	let prg : Vec<i32> = data.lines().map(|l| l.parse::<i32>().unwrap()).collect();

	println!("part 1: {}",count(prg));

	let prg2 : Vec<i32> = data.lines().map(|l| l.parse::<i32>().unwrap()).collect();
	println!("part 2: {}",count2(prg2));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(5,count(vec![0,3,0,1,-3]));
	}
	#[test]
	fn test_part2() {
		assert_eq!(10,count2(vec![0,3,0,1,-3]));
	}
}
