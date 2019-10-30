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

type Atom = u32;

fn hash(s: &str ) -> Atom {
	let bytes = s.as_bytes();
	if bytes.len() > 1 {
		bytes[0] as Atom * bytes[1] as Atom
	}
	else {
		bytes[0] as Atom
	}
}

fn tokenize(s: &str) -> Vec<Atom> {
	let bytes = s.as_bytes();
	let mut v = Vec::new();
	let mut i = 0usize;
	while i < bytes.len() {
		if i+1 < bytes.len() && bytes[i+1] >= 97 && bytes[i+1] <= 122 {
			v.push( bytes[i] as Atom * bytes[i+1] as Atom );
			i += 2
		}
		else{
			v.push( bytes[i] as Atom );
			i += 1;
		}
	}
	v
}

type Rule = (Atom, Vec<Atom>);

fn get_rules(s:&str) -> Vec<Rule> {
	let r = parse(s);
	let mut v = Vec::new();
	for (f,t) in r {
		v.push((hash(f),tokenize(t)));
	}
	v
}

pub fn minimize(rules: &str, data: &str) -> i32 {
	let input = get_rules(&rules);
	let molecule = tokenize(&data);
	let (s,count) = minimize_internal(&input,&molecule[..]);
	if s == tokenize("e") {
		count
	}
	else {
		-1
	}
}

fn minimize_internal(rules: &Vec<Rule>,data: &[Atom]) -> (Vec<Atom>,i32) {
	let mut ops = 0i32;
	let mut result = if let Some((x,y)) = find_rnar(&data[..]) {
		let (mut r1,o1) = minimize_internal(rules,&data[x+1..y]);
		let (mut r2,o2) = minimize_internal(rules, &data[y+1..]);
		ops += o1 + o2;
		let head = &data[0..=x];
		let mut rec = Vec::with_capacity(1+head.len()+r1.len()+r2.len());
		rec.append(&mut head.to_vec());
		rec.append(&mut r1);
		rec.push(7410);
		rec.append(&mut r2);
		rec
	}
	else {
		data.to_vec()
	};

	let mut workslice = &result[..];
	workslice = &workslice[1..2];

	let a = result.split_at(4);



	// x () , () y

	(result,ops)
}


// fn minimize_rnar(rules: &Vec<(&str, &str)>) -> (String,i32) {
// }

fn find_y_no_rnar(data:&[Atom]) -> Option<usize> {
	let rn = 9020;
	let ar = 7410;
	let mut level = 1;
	for x in 0..data.len() {
		let c = data[x];
		if c == rn {
			level += 1;
		} else if level == 1 && c == 89 {
			return Some(x)
		} else if c == ar {
			level -= 1;
		}
	}
	None
}

fn find_rnar(data:&[Atom]) -> Option<(usize,usize)> {
	let rn = 9020;
	let ar = 7410;
	for x in 0..data.len() {
		if data[x] == rn {
			let mut level = 1i32;
			for y in x+1..data.len() {
				let c = data[y];
				if c == rn {
					level += 1;
				} else if level == 1 && c == ar {
					return Some((x,y))
				} else if c == ar {
					level -= 1;
				}
			}
			panic!("unparsable Rn ... Ar");
		}
	}
	None
}

// Y 89

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rnar() {
		let data = tokenize("ThRnFRnTArAr");
		assert_eq!(Some((1,6)),find_rnar(&data));
	}

	#[test]
	fn test_ynornar() {
		let data = tokenize("ThRnYArAYBYC");
		assert_eq!(Some(5),find_y_no_rnar(&data));
	}
	#[test]
	fn test_tokenize() {
		assert_eq!(vec![82*110], tokenize("Rn"));
		assert_eq!(vec![67,82*110,65*108,65*114], tokenize("CRnAlAr"));
	}

	#[test]
	fn test_hash() {
		assert_eq!(82*110, hash("Rn"));
		assert_eq!(67, hash("C"));
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
