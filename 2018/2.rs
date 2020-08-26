use std::collections::HashMap;

fn tokenize(line:&str) -> HashMap<u8,u8> {
	let mut m = HashMap::<u8, u8>::new();
	for b in line.as_bytes() {
		let v = m.entry(*b).or_insert(0);
		*v += 1;
	}
	m
}
fn count(list:&[HashMap<u8,u8>]) -> (u32,u32) {
	let mut twos = 0;
		let mut threes = 0;
		for m in list.iter() {
			let mut hastwo = false;
			let mut hasthree = false;
			for v in m.values() {
				if *v == 2 {
					hastwo = true;
				}
				else if *v == 3 {
					hasthree = true;
				}
			}
			if hastwo { twos += 1; }
			if hasthree { threes += 1; }
		}
		(twos,threes)
	}
	fn equalenough(a:&[u8],b:&[u8])->bool{
		let mut misses = 0;
		for x in 0..a.len() {
			if a[x] != b[x] {
				misses += 1;
				if misses == 2 {
					return false
				}
			}
		}
		misses == 1
	}
fn main(){
	let dstring = std::fs::read_to_string("2018/2.txt").unwrap();
	let data : Vec<HashMap<u8,u8>> = dstring.clone().lines().map(|l| tokenize(l) ).collect();
	let (t2,t3) = count(&data);

	println!("part 1: {} * {} = {}",t2,t3,t2*t3);

	let mut d2 = Vec::new();
	for l in dstring.lines() {
		d2.push( l.as_bytes() );
	}

	for i in 0..d2.len()-1 {
		for j in i..d2.len() {
			if equalenough(&d2[i], &d2[j]) {
				println!("part2: {:?} {:?}",
					std::str::from_utf8(&d2[i]),
					std::str::from_utf8(&d2[j]));
			}
		}
	}


}


#[cfg(test)]
mod tests{
	use super::*;


	#[test]
	fn test_equalenough(){
		let a = "abcd".as_bytes();
		let b = "abcd".as_bytes();
		let c = "abce".as_bytes();
		let d = "aoeu".as_bytes();

		assert_eq!(false,equalenough(a, b));
		assert_eq!(true,equalenough(a, c));
		assert_eq!(false,equalenough(a, d));
	}

	#[test]
	fn test_tokenize(){
		let m = tokenize("abcdef");
		assert_eq!(*m.get(&100).unwrap(),1);
		let m = tokenize("bababc");
		assert_eq!(*m.get(&97).unwrap(),2);
		assert_eq!(*m.get(&98).unwrap(),3);
	}

	#[test]
	fn test_findmatch(){
		let a = vec!["abcde",
			"fghij",
			"klmno",
			"pqrst",
			"fguij",
			"axcye",
			"wvxyz"];

	}

	#[test]
	fn test_count(){
		let s = vec![
			"abcdef",
			"bababc",
			"abbcde",
			"abcccd",
			"aabcdd",
			"abcdee",
			"ababab"
		];
		let maps : Vec<HashMap<u8,u8>> = s.iter().map(|st| tokenize(st) ).collect();
		let (twos,threes) = count(&maps);
		assert_eq!(twos,4);
		assert_eq!(threes,3);
	}
}