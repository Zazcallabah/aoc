fn file() -> Vec<i32> {
	std::fs::read_to_string("2018/1.txt").unwrap().lines().map(|t| t.parse::<i32>().unwrap()).collect()
}
fn first(track: Vec<i32>)->i32{
	let mut seen : std::collections::HashSet<i32> = std::collections::HashSet::new();
	let mut current = 0;
	let mut ix = 0;
	loop {
		if seen.contains(&current) {
			return current;
		}
		seen.insert(current);
		current += track[ ix % track.len() ];
		ix += 1;
	}
}

fn main() {
	let r = file();
	let s : i32 = r.iter().sum();
	println!("part 1: {}",s);

	let t = first(r);
	println!("part 2: {}",t);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getfilecontent() {
		let r = file();
		assert_eq!(-12,r[0]);
		assert_eq!(-6,r[1]);
	}

	#[test]
	fn test_part1(){
		let r = file();
		assert_eq!(502,r.iter().sum());
	}

	#[test]
	fn test_first1(){
		let r = first(vec![1,-1]);
		assert_eq!(0,r);
	}


	#[test]
	fn test_first2(){
		let r = first(vec![3,3,4,-2,-4]);
		assert_eq!(10,r);
	}

	#[test]
	fn test_first3(){
		let r = first(vec![-6, 3, 8, 5, -6]);
		assert_eq!(5,r);
		let r2 = first(vec![7, 7, -2, -7, -4]);
		assert_eq!(14,r2);
	}

}