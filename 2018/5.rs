fn main(){
	let data = std::fs::read_to_string("2018/5.txt").unwrap();
	let result = minimize_str(&data);
	println!("part 1: {}",result.len());

	let mut shortest = usize::MAX;

	for x in 65..91 {
		let tmp = data.as_bytes().to_owned();
		let t2 = extract(&tmp,&x);
		let f = minimize(t2);

		if f.len() < shortest {
			shortest = f.len();
		}
	}

	println!("part 2: {}",shortest);
}

fn extract( input:&[u8], target:&u8 ) -> Vec<u8> {
	input
		.iter()
		.filter(|&i| i != target && i != &(target+32) )
		.map(|i| i.to_owned() )
		.collect()
}

fn minimize_str(input: &str ) -> String {
	std::str::from_utf8(&minimize(input.as_bytes().to_owned())).unwrap().to_string()
}

fn minimize( input: Vec<u8> ) -> Vec<u8> {
	let mut tmp = input;
	loop {
		let len = tmp.len();
		tmp = strip( tmp );
		if tmp.len() == len {
			return tmp;
		}
	}
}

fn strip(input:Vec<u8>) -> Vec<u8> {
	let mut out = Vec::with_capacity(input.len());
	let mut i = 0;
	while i < input.len() {
		let cur = input[i];
		if i+1 == input.len() {
			out.push(cur);
			i += 1;
		}
		else {
			let lookahead = input[i+1];
			if cur == lookahead + 32 || cur == lookahead - 32 {
				i += 2;
			}
			else {
				out.push(cur);
				i += 1;
			}
		}
	}
	out
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_strip(){
		let s = "dabAcCaCBAcCcaDA".as_bytes();
		let ss = strip(s.to_owned());
		assert_eq!("dabAaCBAcaDA",std::str::from_utf8(&ss).unwrap());
		let sss = strip(ss);
		assert_eq!("dabCBAcaDA",std::str::from_utf8(&sss).unwrap());
		let ssss = strip(sss);
		assert_eq!("dabCBAcaDA",std::str::from_utf8(&ssss).unwrap());
	}


	#[test]
	fn test_more_strip(){
		let s = strip("aA".as_bytes().to_owned());
		assert_eq!("",std::str::from_utf8(&s).unwrap());

		let s = strip("abBA".as_bytes().to_owned());
		assert_eq!("aA",std::str::from_utf8(&s).unwrap());
	}

	#[test]
	fn test_min(){
		let s = minimize_str("dabAcCaCBAcCcaDA");

		assert_eq!("dabCBAcaDA",s.to_owned());
	}
}