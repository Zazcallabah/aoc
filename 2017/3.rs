fn path_length(ix:usize) -> usize {
	if ix <= 1 {
		return 0
	}


	//   x . .
	// - - - .
	// - - - .
	// - - - .

	// find side length of inner square
	let candidate = (ix as f64).sqrt().floor() as usize;
	let side = if candidate % 2 == 0 { candidate - 1 } else { candidate };

	//   x . .
	// - - - .
	// - - - .
	// - - i .
	let firstindex = side * side;

	if ix == firstindex {
		return side - 1;
	}

	let pathlength = ix - firstindex;
	//  8 7 6 5 4
	//  9 - - - 3
	// 10 - - - 2
	// 11 - - - 1
	// 12 3 4 5 16


	// symmetrical in along two axes, rotate to simplify
	let rotated = pathlength % (side+1);
	// 0 3 2 1 0
	// 1 - - - 3
	// 2 - - - 2
	// 3 - - - 1
	// 0 1 2 3 0

	if rotated == 0 {
		return side + 1;
	}

	if rotated <= (side / 2)+1 {
		return side + 1 - rotated;
	}

	return rotated;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(438,path_length(265149));
	}

	#[test]
	fn test_large(){
		assert_eq!(5,path_length(26));
		assert_eq!(4,path_length(27));
		assert_eq!(3,path_length(28));
		assert_eq!(4,path_length(29));
		assert_eq!(5,path_length(30));
		assert_eq!(6,path_length(31));
		assert_eq!(5,path_length(32));
		assert_eq!(4,path_length(33));
		assert_eq!(3,path_length(34));
		assert_eq!(4,path_length(35));
		assert_eq!(5,path_length(36));
		assert_eq!(6,path_length(37));
		assert_eq!(5,path_length(38));
	}

	#[test]
	fn test_middle(){
		assert_eq!(3,path_length(10));
		assert_eq!(2,path_length(11));
		assert_eq!(3,path_length(12));
		assert_eq!(4,path_length(13));
		assert_eq!(3,path_length(14));
	}

	#[test]
	fn test_simple(){
		assert_eq!(0,path_length(1));
		assert_eq!(1,path_length(2));
		assert_eq!(2,path_length(3));
		assert_eq!(1,path_length(4));
		assert_eq!(2,path_length(5));
		assert_eq!(1,path_length(6));
		assert_eq!(2,path_length(7));
		assert_eq!(1,path_length(8));
		assert_eq!(2,path_length(9));
	}
}

fn toix(x:usize,y:usize)->usize{
	y*1000+x
}

fn main() {
	let mut grid = vec![0u32;1_000_000];
	let mut x = 500;
	let mut y = 500;
	grid[toix(x,y)] = 1;
	x += 1;

	loop{
		// calc current
		let current =
		grid[toix(x-1,y-1)] +
		grid[toix(x-1,y)] +
		grid[toix(x-1,y+1)] +
		grid[toix(x+1,y-1)] +
		grid[toix(x+1,y)] +
		grid[toix(x+1,y+1)] +
		grid[toix(x,y-1)] +
		grid[toix(x,y+1)];

		if current > 265149 {
			println!( "{}", current );
			break;
		}

		grid[toix(x,y)] = current;

		let blocked_up = grid[toix(x,y+1)] > 0;
		let blocked_right = grid[toix(x+1,y)] > 0;
		let blocked_down = grid[toix(x,y-1)] > 0;
		let blocked_left = grid[toix(x-1,y)] > 0;

		if blocked_up && blocked_left {
			x += 1;
		}
		else if blocked_up && blocked_right {
			y -= 1;
		}
		else if blocked_down && blocked_right {
			x -= 1;
		}
		else if blocked_down && blocked_left {
			y += 1;
		}
		else if blocked_up {
			x += 1;
		}
		else if blocked_right {
			y -= 1;
		}
		else if blocked_down {
			x -= 1;
		}
		else if blocked_left {
			y += 1;
		} else {
			panic!("oh oh");
		}
	}
}