fn file() -> String {
	std::fs::read_to_string("2.txt").unwrap()
}

fn coord_to_nr(x:i32,y:i32) -> i32 {
	y*3+x + 1
}

fn coord_to_char(x:i8,y:i8) -> char {
	const KEYPAD : [char;25] = [
		' ', ' ', '1', ' ', ' ',
		' ', '2', '3', '4', ' ',
		'5', '6', '7', '8', '9',
		' ', 'A', 'B', 'C', ' ',
		' ', ' ', 'D', 'E', ' ',
	];
	let ix = y*5+x;
	KEYPAD[ix as usize]
}
fn get_coords(input : &str) -> Option<(i32,i32)> {
	// see x and y separately
	// only possible end patterns are
	// aaba -> 0
	// aab  -> 1
	// aa   -> 0
	// ending up 0 or 1 steps away from the border
	//

	let mut x : i32 = 0;
	let mut y : i32 = 0;
	let mut xd : Option<i32> = None;
	let mut yd : Option<i32> = None;

	for c in input.chars().rev() {
		match c {
			'R' => {
				if x.abs() < 2 {
					x += 1;
					if xd.is_none() {
						xd = Some(1);
					}
				}
			},
			'L' => {
				if x.abs() < 2 {
					x -= 1;
					if xd.is_none() {
						xd = Some(0);
					}
				}
			},
			'D' => {
				if y.abs() < 2 {
					y += 1;
					if yd.is_none() {
						yd = Some(1);
					}
				}
			},
			'U' => {
				if y.abs() < 2 {
					y -= 1;
					if yd.is_none() {
						yd = Some(0);
					}
				}
			},
			_ => panic!("bad input"),
		};

		if x.abs() >= 2 && y.abs() >= 2 {
			return Some((
				if x < 0 { 0 } else { 1 } + xd.unwrap(),
				if y < 0 { 0 } else { 1 } + yd.unwrap()
			))
		}
	}
	None
}

fn inbounds(x:i8,y:i8)->bool{
	const BOUNDS : [bool;25] = [
		false, false, true, false, false,
		false, true,  true, true,  false,
		true,  true,  true, true,  true,
		false, true,  true, true,  false,
		false, false, true, false, false
	];
	if x < 0 || x > 4 || y < 0 || y > 4 {
		false
	}
	else {
		let ix = y*5+x;
		BOUNDS[ix as usize]
	}
}

fn simulate(input:&str,start:(i8,i8))->(i8,i8){
	let (mut x, mut y) = start;
	for c in input.chars() {
		match c {
			'R' => {
				if inbounds(x + 1,y) {
					x += 1;
				}
			},
			'L' => {
				if inbounds(x - 1,y) {
					x -= 1;
				}
			},
			'D' => {
				if inbounds(x,y + 1) {
					y += 1;
				}
			},
			'U' => {
				if inbounds(x,y - 1) {
					y -= 1;
				}
			},
			_ => panic!("bad input"),
		};
	}
	(x,y)
}

fn get_complex(data: &str) -> Vec<char> {
	let mut complex : Vec<char> = Vec::new();
	let mut start = (0,2);
	for line in data.lines() {
		start = simulate(line,start);
		complex.push(coord_to_char(start.0,start.1));
	}
	complex
}

fn main() {
	let data : &str = &file();
	let mut simple : Vec<i32> = Vec::new();
	for line in data.lines(){
		let coords = get_coords(line).unwrap();
		let button =coord_to_nr(coords.0, coords.1);
		simple.push(button);
	}
	println!("1: {:?}",simple);

	let complex = get_complex(data);
	println!("2: {:?}",complex);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getcomplex() {
		let r = get_complex("ULL\nRRDDD\nLURDL\nUUUUD");
		assert_eq!(vec!['5','D','B','3'],r);
	}

	#[test]
	fn test_simulate() {
		let mut r = simulate("ULL",(0,2));
		assert_eq!((0,2),r);
		r = simulate("RRDDD",r);
		assert_eq!((2,4),r);
		r = simulate("LURDL",r);
		assert_eq!((2,3),r);
		r = simulate("UUUUD",r);
		assert_eq!((2,1),r);
	}

	#[test]
	fn test_nr() {
		assert_eq!(1,coord_to_nr(0,0));
		assert_eq!(2,coord_to_nr(1,0));
		assert_eq!(3,coord_to_nr(2,0));
		assert_eq!(4,coord_to_nr(0,1));
		assert_eq!(5,coord_to_nr(1,1));
	}

	#[test]
	fn test_get_coord_reads_minimal_tail() {
		let r = get_coords("doesnt read this part DDLL");
		assert_eq!(Some((0,2)),r);
	}

	#[test]
	fn test_get_coord_handles_irregular_input() {
		let r = get_coords("RUUUDRURL");
		assert_eq!(Some((1,0)),r);
	}

	#[test]
	fn test_get_coord_corners_ddll() {
		let r = get_coords("DDLL");
		assert_eq!(Some((0,2)),r);
	}
	#[test]
	fn test_get_coord_corners_rrdd() {
		let r = get_coords("RRDD");
		assert_eq!(Some((2,2)),r);
	}
	#[test]
	fn test_get_coord_corners_rruu() {
		let r = get_coords("RRUU");
		assert_eq!(Some((2,0)),r);
	}
	#[test]
	fn test_get_coord_corners_uull() {
		let r = get_coords("UULL");
		assert_eq!(Some((0,0)),r);
	}

}