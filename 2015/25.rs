//    | 1   2   3   4   5   6
// ---+---+---+---+---+---+---+
//  1 |  1   3   6  10  15  21
//  2 |  2   5   9  14  20
//  3 |  4   8  13  19
//  4 |  7  12  18
//  5 | 11  17
//  6 | 16

//    | 1   2   3   4
// ---+---+---+---+---+
//  1 |  1   3   6  10
//  2 |             14
//  3 |             19

//    | 1   2   3   4
// ---+---+---+---+---+
//  1 |  1  +2  +3  +4
//  2 |             +4
//  3 |             +5

// => sum 1..5 + 4
// => sum 1..5 + col
// => sum 1..(col+row-2) + col

fn cord_to_ix(row:usize,col:usize) -> usize {
	let end = col+row-2;
	let sum = (1+end)*end/2;
	col + sum
}

fn next_code(code:u64) -> u64 {
	code * 252533 % 33554393
}

fn get_code(row:usize,col:usize) -> u64 {
	let mut ix = cord_to_ix(row,col);
	let mut code = 20151125;
	while ix > 1 {
		ix -= 1;
		code = next_code(code);
	}
	code
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cix_1() {
		let conv : Vec<usize> = (1..=8).map(|x| cord_to_ix(1,x)).collect();
		assert_eq!(vec![1,3,6,10,15,21,28,36],conv);
	}

	#[test]
	fn test_cix_2() {
		let conv : Vec<usize> = (1..=7).map(|x| cord_to_ix(2,x)).collect();
		assert_eq!(vec![2,5,9,14,20,27,35],conv);
	}

	#[test]
	fn test_cix_3() {
		let conv : Vec<usize> = (1..=6).map(|x| cord_to_ix(3,x)).collect();
		assert_eq!(vec![4, 8, 13, 19, 26, 34],conv);
	}

	#[test]
	fn test_next_code() {
		assert_eq!(31916031,next_code(20151125));
	}

	#[test]
	fn test_get_code() {
		let conv : Vec<u64> = (1..=6).map(|x| get_code(5,x) ).collect();
		assert_eq!(vec![77061, 17552253, 28094349, 6899651, 9250759, 31663883],conv);
	}

	#[test]
	fn test_part1() {
		// To continue, please consult the code grid in the manual.  Enter the code at row 2978, column 3083.
		assert_eq!(2650453,get_code(2978,3083));
	}
}
