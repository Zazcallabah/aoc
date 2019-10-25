pub fn to26(s: &str) -> u64 {
	let mut sum: u64 = 0;
	let mut place = 0;
	let bytes = s.as_bytes().into_iter().rev();
	for i in bytes {
		let c = (i - 97) as u64;
		sum = sum + c * 26u64.pow(place);
		place = place + 1
	}
	sum
}

pub fn from26(mut sum: u64) -> String {
	let mut s = String::with_capacity(8);
	while sum >= 1 {
		let rest: u8 = (sum % 26) as u8;
		s.push((rest + 97u8) as char);
		sum = sum / 26;
	}
	s.chars().rev().collect()
}

const _POW: [u64; 9] = [
	1,
	26,
	676,
	17576,
	456976,
	11881376,
	308915776,
	8031810176,
	208827064576,
];

pub fn get_val(num: u64, ix: usize) -> u64 {
	num / _POW[ix] % 26u64
}

pub fn no_illegal_chars(num: u64) -> bool {
	for ix in 0..=7 {
		let val = get_val(num, ix);
		if val == 8 || val == 11 || val == 14 {
			return false;
		}
	}
	true
}

pub fn has_two_pairs(num: u64) -> bool {
	let mut hasone = false;
	let mut first = get_val(num,0);
	let mut ix = 1;
	while ix <= 7 {
		let val = get_val(num,ix);
		if first == val {
			if hasone {
				return true
			}
			ix=ix+1;
			hasone = true;
			first = get_val(num,ix);
		}
		else
		{
			first = val
		}
		ix = ix+1;
	}
	false
}

pub fn has_straight(num: u64) -> bool {
	let mut first = get_val(num,0);
	let mut second = get_val(num,1);
	for ix in 2..=7 {
		let third = get_val(num,ix);
		//remember we index backwards in num
		let s = second + 1;
		if first == s && s == third + 2 {
			return true
		}
		first = second;
		second = third;
	}
	false
}

pub fn is_valid(num: u64) -> bool {
	no_illegal_chars(num) && has_straight(num) && has_two_pairs(num)
}

pub fn get_next(mut num: u64) -> u64 {
	while !is_valid(num) {
		num = num + 1;
	}
	num
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getnext() {
		assert_eq!(get_next(to26("xbcdffaa")),to26("xbcdffaa"));
		assert_eq!(get_next(to26("xbcdefgh")),to26("xbcdffaa"));
	}

	#[test]
	fn test_isvalid() {
		assert!(!is_valid(to26("hijklmmn")));
		assert!(!is_valid(to26("abbceffg")));
		assert!(!is_valid(to26("abbcegjk")));

		assert!(is_valid(to26("vzbxxyzz")));
	}

	#[test]
	fn test_hasstraight() {
		assert!(has_straight(to26("xxabcxxx")));

		assert!(!has_straight(to26("zyxdabdx")));
	}

	#[test]
	fn test_twopairs() {
		assert!(!has_two_pairs(to26("abcdefgh")));
		assert!(!has_two_pairs(to26("abcccfgh")));
		assert!(!has_two_pairs(to26("abccxcgh")));

		assert!(has_two_pairs(to26("abccccgh")));
	}

	#[test]
	fn test_to26() {
		assert_eq!(to26("ba"), 26);
		assert_eq!(to26("a"), 0);
	}

	#[test]
	fn test_from26() {
		let s = "vzbxkghb";
		assert_eq!(from26(to26(s)), s);
		assert_eq!(from26(to26("aaaaab")), "b");
		assert_eq!(from26(to26("baaaaa")), "baaaaa");
	}

	#[test]
	fn test_getval() {
		assert_eq!(get_val(to26("xxxxxxxa"), 0), 0);
		assert_eq!(get_val(to26("bxxxxxxx"), 7), 1);
		assert_eq!(get_val(to26("bbxbbbbb"), 5), 23);
	}

	#[test]
	fn test_illegals() {
		assert!(!no_illegal_chars(to26("xxixxxxa")));
		assert!(!no_illegal_chars(to26("xxoxxxa")));
		assert!(!no_illegal_chars(to26("xxlxxa")));

		assert!(no_illegal_chars(to26("xxxxxxxa")));
	}

	#[test]
	fn test_increment() {
		assert_eq!(from26( to26("xxxxyyyy") + 1 ),"xxxxyyyz");
		assert_eq!(from26( to26("xxxxyyyz") + 1 ),"xxxxyyza");
		assert_eq!(from26( to26("xxxxzzzz") + 1 ),"xxxyaaaa");
	}
}

fn main() {
	let data1 = "vzbxkghb";
	let first = get_next(to26(data1));
	println!("first pass {}", from26(first));

	let second = get_next(first+1);

	println!("second pass {}", from26(second));
}