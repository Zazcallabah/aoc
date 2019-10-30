use std::collections::HashMap;

fn parse<'a>(s: &'a str) -> Vec<(&'a str, &'a str)> {
	s.split("\n").map(|line: &str| parse_line(line)).collect()
}

fn parse_line<'a>(s: &'a str) -> (&'a str, &'a str) {
	let result: Vec<&str> = s.split(" => ").collect();
	(result[0], result[1])
}

fn apply_rule(rule: (&str, &str), source: &str, map: &mut HashMap<String, ()>) {
	let (from, to) = rule;
	let cap: usize = source.len() + to.len() - from.len();
	let splits: Vec<&str> = source.split(from).collect();
	for i in 0..splits.len() - 1 {
		let mut entry = String::with_capacity(cap);

		for s in 0..splits.len() - 1 {
			entry.push_str(splits[s]);
			if s == i {
				entry.push_str(to);
			} else {
				entry.push_str(from);
			}
		}

		let last = splits[splits.len() - 1];
		if last != "" {
			entry.push_str(last);
		}

		if !map.contains_key(&entry) {
			map.insert(entry, ());
		}
	}
}

fn run(rules: &str, data: &str) -> Vec<String> {
	let input = parse(&rules);
	let mut map: HashMap<String, ()> = HashMap::new();

	for rule in input.iter() {
		apply_rule(*rule, data, &mut map);
	}

	let mut keys: Vec<String> = map.keys().cloned().collect();
	keys.sort();
	keys
}

fn main() {
	let rules = std::fs::read_to_string("19-rules.txt").unwrap();
	let data = std::fs::read_to_string("19.txt").unwrap();
	let map = run(&rules.replace("\r\n", "\n"), &data);
	println!("unique combinations: {}",map.len());

	let simplified_rule_set = get_rules();
	let mut tokenized_data = tokenize(&data);
	let opcount = run_length_pass(&simplified_rule_set,&mut tokenized_data);
	println!("operations needed: {}",opcount);
}

// rule set:
// convert any atoms not Rn Ar or Y into _
// _ => __
// _ => _Rn_Ar
// _ => _Rn_Y_Ar
// _ => _Rn_Y_Y_Ar
//
// when molecule has length 1, we're done

fn hash(s: &str ) -> u8 {
	if s == "Rn" {
		5
	}
	else if s == "Ar" {
		2
	}
	else if s == "Y" {
		1
	}
	else {
		0
	}
}

fn tokenize(s: &str) -> Vec<u8> {
	let bytes = s.as_bytes();
	let mut v = Vec::new();
	let mut i = 0usize;
	while i < bytes.len() {
		if i+1 < bytes.len() && bytes[i+1] >= 97 && bytes[i+1] <= 122 {
			v.push( hash(&s[i..=i+1]) );
			i += 2
		}
		else{
			v.push( hash(&s[i..=i]) );
			i += 1;
		}
	}
	v
}

type Rule = (u8, Vec<u8>);
// _ => __
// _ => _Rn_Ar
// _ => _Rn_Y_Ar
// _ => _Rn_Y_Y_Ar

fn get_rules() -> Vec<Rule> {
	vec![
		(0,vec![0,0]),
		(0,vec![0,5,0,2]),
		(0,vec![0,5,0,1,0,2]),
		(0,vec![0,5,0,1,0,1,0,2]),
	]
}

fn rule_match(rule: &Rule, data:&[u8], ix: usize) -> bool {
	if rule.1.len() + ix > data.len() {
		false
	}
	else {
		rule.1 == &data[ix..ix+rule.1.len()]
	}
}

fn run_length_pass(rules: &Vec<Rule>,data: &mut Vec<u8>) -> i32 {
	let mut ix = data.len();
	let mut opcount = 0;
	loop {
		if data.len() == 1 {
			return opcount
		}
		if ix > 0 {
			ix -= 1;
		}
		for rule in rules {
			if rule_match( &rule, data, ix ) {
				data.drain(ix+1..ix+rule.1.len());
				opcount += 1;
				ix += 1;
				break;
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize() {
		let t = tokenize("SiRnFYCaFArSi");
		assert_eq!(vec![0,5,0,1,0,0,2,0],t);
	}

	#[test]
	fn test_rulematch() {
		let data = vec![0,0,5,0,0,2];
		let rule = (0,vec![0,0]);

		assert!(rule_match(&rule, &data, 3));
		assert!(rule_match(&rule, &data, 0));

		assert!(!rule_match(&rule, &data, 1));
		assert!(!rule_match(&rule, &data, 2));
		assert!(!rule_match(&rule, &data, 4));
		assert!(!rule_match(&rule, &data, 5));
	}

	#[test]
	fn test_rlp() {
		let mut v = vec![0,0,5,0,0,2];
		let opcount = run_length_pass(&get_rules(),&mut v);
		assert_eq!(3, opcount);
		assert_eq!(vec![0],v);
	}

	#[test]
	fn test_rlp_trailing_zero() {
		let mut v = vec![0, 5, 0, 0, 0, 0, 2, 0, 5, 0, 0, 0, 2, 0];
		let opcount = run_length_pass(&get_rules(),&mut v);
		assert_eq!(9, opcount);
		assert_eq!(vec![0],v);
	}

	#[test]
	fn test_rlp_trailing_zero_within_rnar() {
		let mut v = vec![0, 5, 0, 5, 0, 2, 0, 2, 0, 5, 0, 0, 0, 2, 0];
		let opcount = run_length_pass(&get_rules(),&mut v);
		assert_eq!(8, opcount);
		assert_eq!(vec![0],v);
	}

	#[test]
	fn test_rlp_rnarrnar() {
		let mut v = vec![0, 5, 0, 5, 0, 0, 2, 5, 0, 2, 0, 2, 0];
		let opcount = run_length_pass(&get_rules(),&mut v);
		assert_eq!(6, opcount);
		assert_eq!(vec![0],v);
	}




	#[test]
	fn test_withtestinput() {
		let keys = run("H => HO\nH => OH\nO => HH", "HOH");
		assert_eq!(vec!["HHHH", "HOHO", "HOOH", "OHOH"], keys);
	}

	#[test]
	fn test_parseline() {
		let (from, to) = parse_line("H => HO");
		assert_eq!("H", from);
		assert_eq!("HO", to);
	}

	#[test]
	fn test_parse() {
		let res = parse("H => HO\nA => BC");
		let (from, to) = res[1];
		assert_eq!("A", from);
		assert_eq!("BC", to);
	}

	#[test]
	fn test_applyrule() {
		let mut map: HashMap<String, ()> = HashMap::new();
		apply_rule(("H", "HO"), "HOH", &mut map);
		apply_rule(("H", "OH"), "HOH", &mut map);
		assert_eq!(3, map.keys().len());
	}
}
