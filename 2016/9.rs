#[macro_use]
extern crate lazy_static;

fn count_recurse(input: &str, deep: bool) -> usize {
	lazy_static! {
		static ref FINDER: regex::Regex = regex::Regex::new(r"\((\d+)x(\d+)\)").unwrap();
	}

	if let Some(cap) = FINDER.captures(input) {
		let bounds = cap.get(0).unwrap();
		let len: usize = cap.get(1).unwrap().as_str().parse().unwrap();
		let times: usize = cap.get(2).unwrap().as_str().parse().unwrap();

		let start = bounds.start();
		let end = bounds.end();

		let extracted = times * if deep {
			count_recurse(&input[end..end + len], deep)
		}
		else {
			len
		};
		let tail = count_recurse(&input[end + len..], deep);

		start + extracted + tail
	} else {
		input.len()
	}
}
fn main() {
	let inp = std::fs::read_to_string("9.txt").unwrap();
	println!("simple {}", count_recurse(&inp, false));
	println!("full {}", count_recurse(&inp, true));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decompress() {
		assert_eq!("ADVENT".len(), count_recurse("ADVENT", false));
		assert_eq!("ABBBBBC".len(), count_recurse("A(1x5)BC", false));
		assert_eq!("XYZXYZXYZ".len(), count_recurse("(3x3)XYZ", false));
		assert_eq!(
			"ABCBCDEFEFG".len(),
			count_recurse("A(2x2)BCD(2x2)EFG", false)
		);
		assert_eq!("(1x3)A".len(), count_recurse("(6x1)(1x3)A", false));
		assert_eq!(
			"X(3x3)ABC(3x3)ABCY".len(),
			count_recurse("X(8x2)(3x3)ABCY", false)
		);
		assert_eq!("XY".len(), count_recurse("X(1x1)Y", false));
	}

	#[test]
	fn test_count() {
		assert_eq!(9, count_recurse("(3x3)XYZ", true));
		assert_eq!(20, count_recurse("X(8x2)(3x3)ABCY", true));
		assert_eq!(
			241920,
			count_recurse("(27x12)(20x12)(13x14)(7x10)(1x12)A", true)
		);
		assert_eq!(
			445,
			count_recurse(
				"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
				true
			)
		);
	}
}
