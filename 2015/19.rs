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

pub fn minimize(rules: &str, data: &str) -> i32 {
	let input = parse(&rules);
	let (s,count) = minimize_internal(&input,&data);
	if s == "e" {
		count
	}
	else {
		-1
	}
}

fn minimize_internal(rules: &Vec<(&str, &str)>,data: &str) -> (String,i32) {
	let mut ops = 0i32;
	if let Some((x,y)) = find_paren(data) {
		let (replacewith,opcount) = minimize_internal(rules,&data[x+1..y]);
		ops += opcount;
	}

	(String::from("e"),1)
}


// fn minimize_rnar(rules: &Vec<(&str, &str)>) -> (String,i32) {
// }

fn find_paren(data:&str) -> Option<(usize,usize)> {
		if let Some(x) = data.find("(") {
			let mut level = 0;
			if let Some(y) = data.find(|c:char| {
				if c == '(' {
					level += 1;
					false
				} else if level == 1 && c == ')' {
					true
				} else if c == ')' {
					level -= 1;
					false
				}
				else {
					false
				}
		}) {
			return Some((x,y))
		}
		else
		{
			panic!("unparsable Rn ... Ar");
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rnar() {

		let data = String::from("0123(5(7))");
		assert_eq!(Some((4,9)),find_paren(&data));

	}

	// #[test]
	// fn test_part2() {
	// 	let rules = std::fs::read_to_string("19-rules.txt")
	// 		.unwrap()
	// 		.replace("\r\n", "\n")
	// 		.replace("Rn", "(")
	// 		.replace("Ar", ")");
	// 	let data = std::fs::read_to_string("19.txt")
	// 		.unwrap()
	// 		.replace("Rn", "(")
	// 		.replace("Ar", ")");;
	// 	assert_eq!(509, minimize(&rules, &data));
	// }

	// #[test]
	// fn test_part1() {
	// 	let rules = std::fs::read_to_string("19-rules.txt").unwrap();
	// 	let data = std::fs::read_to_string("19.txt").unwrap();
	// 	let map = run(&rules.replace("\r\n", "\n"), &data);
	// 	assert_eq!(509, map.len());
	// }

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
