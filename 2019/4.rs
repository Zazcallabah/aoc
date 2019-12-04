
fn getnum(p:u32,i:u32) -> u32 {
	(p / 10u32.pow(i)) % 10
}

fn ismatch(p:u32) -> bool {
	let mut doublefound = false;
	let mut last = 0u32;

	for i in (0..=5).rev() {
		let n = getnum(p, i);
		if n == last {
			doublefound = true;
		}
		else if n < last {
			return false;
		}
		last = n;
	}

	doublefound
}

fn has_singlepair(p:u32) -> bool {
	let mut runlength = 1;
	let mut last = 0;
	for i in (0..=5).rev() {
		let n = getnum(p, i);
		if n == last {
			runlength += 1;
		}
		else {
			if runlength == 2 {
				return true
			}
			runlength = 1;
		}
		last = n;
	}
	runlength == 2
}

fn main() {
	let mut c = Vec::new();
	for p in 359282..=820401 {
		if ismatch(p) {
			c.push(p);
		}
	}
	println!("count {}",c.len());

	let mut actually = Vec::new();
	for p in c {
		if has_singlepair(p) {
			actually.push(p);
		}
	}
	println!("{:?}\ncount {}",actually,actually.len());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getnum(){
		assert_eq!(3,getnum(359282,5));
		assert_eq!(2,getnum(359282,0));
	}
}