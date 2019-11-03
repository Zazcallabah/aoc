fn check(v:&mut Vec<i32>) -> bool {
	v.sort();
	v[0] + v[1] > v[2]
}

fn main() {
	let input = std::fs::read_to_string("3.txt").unwrap();
	let mut count = 0;
	let mut count2 = 0;

	let mut c1 : Vec<i32> = Vec::new();
	let mut c2 : Vec<i32> = Vec::new();
	let mut c3 : Vec<i32> = Vec::new();

	for line in input.lines() {
		let mut nr :Vec<i32> = line.split(' ').filter(|t| t != &"" ).map(|t| t.parse::<i32>().unwrap() ).collect();
		c1.push(nr[0]);
		c2.push(nr[1]);
		c3.push(nr[2]);
		if check(&mut nr) {
			count+=1;
		}
		if c1.len() == 3 {
			if check(&mut c1) {
				count2 +=1;
			}
			if check(&mut c2) {
				count2 +=1;
			}
			if check(&mut c3) {
				count2 +=1;
			}
			c1.clear();
			c2.clear();
			c3.clear();
		}
	}


	println!("1 possible: {}",count);
	println!("2 possible: {}",count2);
}