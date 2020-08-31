
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part2() {
		assert!(isvalid("abcde fghij".split_whitespace().into_iter().collect()));
		assert!(!isvalid("abcde xyz ecdab".split_whitespace().into_iter().collect()));
		assert!(isvalid("a ab abc abd abf abj".split_whitespace().into_iter().collect()));
		assert!(isvalid("iiii oiii ooii oooi oooo".split_whitespace().into_iter().collect()));
		assert!(!isvalid("oiii ioii iioi iiio".split_whitespace().into_iter().collect()));
	}
}

fn isvalid(pw:Vec<&str>) -> bool {
	let mut set = std::collections::HashSet::new();
	for word in pw {
		let mut b = word.to_owned().into_bytes();
		b.sort();
		if set.contains(&b) {
			return false;
		}
		set.insert(b);
	}
	return true;
}
//For added security, yet another system policy has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.


fn main() {
	let data = std::fs::read_to_string("./2017/4.txt").unwrap();
	let mut valid = 0u32;
	'mainline: for line in data.lines() {
		let mut set = std::collections::HashSet::new();
		let words = line.split_whitespace();
		for word in words {
			if set.contains(&word) {
				continue 'mainline;
			}
			set.insert(word);
		}
		valid += 1;
	}

	println!("part 1: {}",valid);

	let mut part2 = 0;
	for line in data.lines() {
		if isvalid(line.split_whitespace().into_iter().collect()) {
			part2 += 1;
		}
	}
	println!("part 2: {}",part2);

}