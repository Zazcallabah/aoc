// Wandering around a secure area, you come across a datalink port to a new part of the network.
// After briefly scanning it for interesting files, you find one file in particular that catches
// your attention. It's compressed with an experimental format, but fortunately, the documentation
// for the format is nearby.

// The format compresses a sequence of characters. Whitespace is ignored. To indicate that some
// sequence should be repeated, a marker is added to the file, like (10x2). To decompress this
// marker, take the subsequent 10 characters and repeat them 2 times. Then, continue reading the
//  file after the repeated data. The marker itself is not included in the decompressed output.

// If parentheses or other characters appear within the data referenced by a marker, that's okay
//  - treat it like normal data, not a marker, and then resume looking for markers after the
//  decompressed section.
fn decompress(input:&str)->String{
	let mut output = String::new();
	let mut marker = 0usize;
	let re = regex::Regex::new(r"\((?P<len>\d+)x(?P<times>\d+)\)").unwrap();
	for cap in re.captures_iter(&input) {
		let capmarker = cap.get(0).unwrap();
		let start = capmarker.start();
		let end = capmarker.end();
		if start >= marker {
			let len : usize = cap.name(&"len").unwrap().as_str().parse().unwrap();
			let times : usize = cap.name(&"times").unwrap().as_str().parse().unwrap();
			if len == 0 || times == 0 {
				panic!("zero found in marker");
			}
			// push from marker to start of capture
			output.push_str(&input[marker..start]);
			for i in 0..times {
				output.push_str(&input[end..end+len]);
			}
			marker = end + len;
		}
	}
	// push trailing
	output.push_str(&input[marker..input.len()]);
	output
}

fn count_decompressed(input:&str)->usize{
	let mut output = 0usize;
	let mut marker = 0usize;
	let re = regex::Regex::new(r"\((?P<len>\d+)x(?P<times>\d+)\)").unwrap();
	for cap in re.captures_iter(&input) {
		let capmarker = cap.get(0).unwrap();
		let start = capmarker.start();
		let end = capmarker.end();
		if start >= marker {
			let len : usize = cap.name(&"len").unwrap().as_str().parse().unwrap();
			let times : usize = cap.name(&"times").unwrap().as_str().parse().unwrap();
			if len == 0 || times == 0 {
				panic!("zero found in marker");
			}
			output+=start-marker;
			output+= len*times;
			marker = end + len;
		}
	}
	// push trailing
	output += input.len() - marker;
	output
}

// fn count(input:&str)->usize{
// 	let re = regex::Regex::new(r"\((?P<len>\d+)x(?P<times>\d+)\)").unwrap();
// 	let mut invalid_up_to = 0usize;
// 	let mut counter = 0usize;
// 	for cap in re.captures_iter(&input) {
// 		let capmarker = cap.get(0).unwrap();
// 		if capmarker.start() >= invalid_up_to {

// 			// count invalidmarker to start
// 			counter += capmarker.start() - invalid_up_to;
// 			// count length of added data
// 			counter += times*len;
// 			// set invalid to end of counted length
// 			invalid_up_to += capmarker.end() - 1 + len;
// 		}
// 	}
// 	// count tail

// For example:

//     ADVENT contains no markers and decompresses to itself with no changes, resulting in a decompressed length of 6.
//     A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
//     (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.

//      doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
//      simply becomes  - the (1x3) looks like a marker, but because it's within a
// 		data section of another marker, it is not treated any differently from the A that comes after it. It has a decompressed length of 6.
//      becomes  (for a decompressed length of 18), because the
// 		 decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.

// What is the decompressed length of the file (your puzzle input)? Don't count whitespace.

fn main() {
	let inp = std::fs::read_to_string("9.txt").unwrap();
	println!("len {}",decompress(&inp).len());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decompress() {
		assert_eq!("ADVENT",decompress("ADVENT"));
		assert_eq!("ABBBBBC",decompress("A(1x5)BC"));
		assert_eq!("XYZXYZXYZ",decompress("(3x3)XYZ"));
		assert_eq!("ABCBCDEFEFG",decompress("A(2x2)BCD(2x2)EFG"));
		assert_eq!("(1x3)A",decompress("(6x1)(1x3)A"));
		assert_eq!("X(3x3)ABC(3x3)ABCY",decompress("X(8x2)(3x3)ABCY"));
		assert_eq!("XY",decompress("X(1x1)Y"));
	}

	#[test]
	fn test_count() {
		assert_eq!(9,count_decompressed("(3x3)XYZ"));
		assert_eq!(20,count_decompressed("X(8x2)(3x3)ABCY"));
		assert_eq!(241920,count_decompressed("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
		assert_eq!(445,count_decompressed("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));


	}
}