fn is_tls(line: &str) -> bool {
	let mut bracket = false;
	let mut found = false;
	let chars: Vec<char> = line.chars().collect();
	for i in 0..chars.len() - 3 {
		if chars[i] == '[' {
			bracket = true;
		} else if chars[i] == ']' {
			bracket = false;
		} else if chars[i] != chars[i + 1]
			&& chars[i] == chars[i + 3]
			&& chars[i + 1] == chars[i + 2]
		{
			if bracket {
				return false;
			}
			found = true;
		}
	}
	found
}

fn is_ssl(line: &str) -> bool {
	let mut outer_aba : Vec<(char,char)> = Vec::new();
	let mut inner_aba : Vec<(char,char)> = Vec::new();

	let mut is_inner = false;
	let chars : Vec<char> = line.chars().collect();

	for i in 0..line.len()-2 {
		if chars[i] == '[' {
			is_inner = true;
		}
		else if chars[i] == ']' {
			is_inner = false;
		}
		else if chars[i] == chars[i+2] && chars[i] != chars[i+1] {
			if is_inner {
				inner_aba.push((chars[i],chars[i+1]));
			}
			else {
				outer_aba.push((chars[i],chars[i+1]));
			}
		}
	}

	for (a,b) in &outer_aba {
		for (x,y) in &inner_aba {
			if x == b && y == a {
				return true
			}
		}
	}

	// An IP supports SSL if it has an Area-Broadcast Accessor,
	//or ABA, anywhere in the supernet sequences (outside any square bracketed sections),
	//and a corresponding Byte Allocation Block, or BAB, anywhere in the hypernet sequences.
	// An ABA is any three-character sequence which consists of the same character twice with
	//a different character between them, such as xyx or aba. A corresponding BAB is the same
	//characters but in reversed positions: yxy and bab, respectively.

	false
}

fn main() {
	let count = std::fs::read_to_string("7.txt")
		.unwrap()
		.lines()
		.filter(|l| is_tls(&l))
		.count();
	println!("count tls {}", count);
	let count = std::fs::read_to_string("7.txt")
		.unwrap()
		.lines()
		.filter(|l| is_ssl(&l))
		.count();
	println!("count ssl {}", count);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_ssl() {
		assert!(is_ssl("aba[bab]xyz"));
		assert!(!is_ssl("xyx[xyx]xyx"));
		assert!(is_ssl("aaa[kek]eke"));
		assert!(is_ssl("zazbz[bzb]cdb"));
	}

	#[test]
	fn test_is_tls() {
		assert!(is_tls("abba[mnop]qrst"));
		assert!(!is_tls("abcd[bddb]xyyx"));
		assert!(!is_tls("aaaa[qwer]tyui"));
		assert!(is_tls("ioxxoj[asdfgh]zxcvbn"));
		assert!(is_tls("abcd[bdadb]xyyx"));
	}

	#[test]
	fn test_data() {
		assert!(!is_tls(
			"rhamaeovmbheijj[hkwbkqzlcscwjkyjulk]ajsxfuemamuqcjccbc"
		));
		assert!(!is_tls("gdlrknrmexvaypu[crqappbbcaplkkzb]vhvkjyadjsryysvj[nbvypeadikilcwg]jwxlimrgakadpxu[dgoanojvdvwfabtt]yqsalmulblolkgsheo"));
		assert!(!is_tls("dqpthtgufgzjojuvzvm[eejdhpcqyiydwod]iingwezvcbtowwzc[uzlxaqenhgsebqskn]wcucfmnlarrvdceuxqc[dkwcsxeitcobaylhbvc]klxammurpqgmpsxsr"));
		assert!(!is_tls(
			"gmmfbtpprishiujnpdi[wedykxqyntvrkfdzom]uidgvubnregvorgnhm"
		));
		assert!(!is_tls(
			"txxplravpgztjqcw[txgmmtlhmqpmmwp]bmhfgpmafxqwtrpr[inntmjmgqothdzfqgxq]cvtwvembpvdmcvk"
		));
		assert!(!is_tls("gkxjhpayoyrrpcr[mwyoahlkqyhtznyzrm]mvmurvsrgjunjjepn[mkoumuohilpcfgbmsmh]hpwggyvjkusjxcyojyr[wqxyuzbewpjzlyqmkhw]nniczueulxtdsmkniex"));
		assert!(!is_tls("vuzyoofrvaanszwndyt[mzcbhmabgnetrpje]tqnygwhmwrbyosbke[gehqzyhlnyufknqmueo]ngendggbjcvazwol"));
		assert!(!is_tls("vdnploylmxnipfudw[pbkxlaozhqhlbzz]kpxnzwjhybgcenyw[fpukiqrjraomftt]rosyxtsdltbsmhykxu[wrucjfwuyypmiic]ydnbgvicfnmwzuatudd"));
		assert!(is_tls("lknaffpzamlkufgt[uvdgeatxkofgoyoi]ajtqcsfdarjrddrzo[bxrcozuxifgevmog]rlyfschtnrklzufjzm"));
	}
}
