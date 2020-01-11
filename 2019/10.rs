use std::collections::HashMap;

fn main() {
	let s = std::fs::read_to_string("2019/10.txt").unwrap();

	let a = get_all(&s);

//	for (x,y) in
}


fn is_same(a:Coord,b:Coord)-> bool{
	a.0 * b.1 == a.1 * b.0
}


fn get_all(map:&str) -> Vec<Coord> {
	let mut v = Vec::new();

	for (y,line) in map.lines().enumerate() {
		for (x,exists) in line.bytes().map(|b| b == b'#').enumerate() {
			if exists {
				v.push((x,y));
			}
		}
	}
	v
}

type Coord = (usize,usize);



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_same_quotient(){
		assert!(is_same((1,1),(2,2)));
		assert!(is_same((4,8),(2,4)));
		assert!(!is_same((4,8),(2,5)));
	}

	#[test]
	fn test_create() {
		let f = get_all(
			r".#..#
.....
#####
....#
...##",
		);
	}
}
// or example, consider the following map:

// .#..#
// .....
// #####
// ....#
// ...##

// The best location for a new monitoring station on this
// map is the highlighted asteroid at 3,4 because it can
// detect 8 asteroids, more than any other location.
// (The only asteroid it cannot detect is the one at 1,0;
// its view of this asteroid is blocked by the asteroid
// at 2,2.) All other asteroids are worse locations; they
// can detect 7 or fewer other asteroids. Here is the number
//  of other asteroids a monitoring station on each asteroid could detect:

// .7..7
// .....
// 67775
// ....7
// ...87
