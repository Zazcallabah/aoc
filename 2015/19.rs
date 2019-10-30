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

fn get_rules(s:&str) -> (Vec<Rule>,HashMap<Atom,&str>) {
	let r = parse(s);
	let mut v = Vec::new();
	let mut map = HashMap::new();
	map.insert(hash("C"), "C");
	map.insert(hash("Y"), "Y");
	map.insert(hash("Rn"), "Rn");
	map.insert(hash("Ar"), "Ar");
	for (f,t) in r {
		let key = hash(f);
		if !map.contains_key(&key) {
			map.insert(key,f);
		}
		v.push((key,tokenize(t)));
	}
	(v,map)
}

pub fn minimize(rules: &str, data: &str) -> i32 {
	let (input,_) = get_rules(&rules);
	let molecule = tokenize(&data);
	let (s,count) = minimize_internal(&input,&molecule);
	if s == tokenize("e") {
		count
	}
	else {
		-1
	}
}

fn minimize_y(rules: &Vec<Rule>,data: &[Atom]) -> (Vec<Atom>,i32) {
	if let Some(y) = find_y_no_rnar(data) {
		let (mut r,o) = minimize_brute(rules, &data[..y]);
		let (mut r2,o2) = minimize_y(rules, &data[y+1..]);
		r.push(89);
		r.append(&mut r2);
		(r,o+o2)
	}
	else {
		minimize_brute(rules,data)
	}
}

fn minimize_internal(rules: &Vec<Rule>,data: &[Atom]) -> (Vec<Atom>,i32) {
	let mut ops = 0i32;
	let result = if let Some((x,y)) = find_rnar(&data) {
		let (mut r1,o1) = minimize_internal(rules,&data[x+1..y]);
		let (mut r2,o2) = minimize_internal(rules, &data[y+1..]);
		ops += o1 + o2;
		let head = &data[0..=x];
		let mut rec = Vec::with_capacity(1+head.len()+r1.len()+r2.len());
		rec.extend_from_slice(head);
		rec.append(&mut r1);
		rec.push(7410);
		rec.append(&mut r2);
		rec
	}
	else {
		data.to_vec()
	};

	let (r3,o3) = minimize_y(rules, &result);

	(r3,ops+o3)
}


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

fn minimize_brute(rules: &Vec<Rule>,data: &[Atom]) -> (Vec<Atom>,i32) {
	for (from,to) in rules {
		if to.len() <= data.len() {
			for ix in 0..=data.len()-to.len() {
				if &data[ix..ix+to.len()] == &to[..] {
					let mut v = Vec::with_capacity(data.len() - to.len() + 1 );
					v.extend_from_slice(&data[..ix]);
					v.push(*from);
					v.extend_from_slice(&data[ix+to.len()..]);
					let (n,i) = minimize_brute(rules,&v);
					return (n,i+1)
				}
			}
		}
	}
	(data.to_vec(),0)
}

fn reverse(lookup:HashMap<Atom,&str>,list : Vec<Atom> )-> String {
	let mut r = String::with_capacity(list.len()*2);
	for a in list {
		r.push_str(lookup[&a]);
	}
	r
}

#[cfg(test)]
mod tests {
	use super::*;

	fn ho(s:&str,o:i32) -> (Vec<Atom>,i32) {
		(tokenize(s),o)
	}

	fn rule(f:&str,t:&str) -> Rule {
		(hash(f),tokenize(t))
	}

	#[test]
	fn test_part2() {
		let rules = std::fs::read_to_string("19-rules.txt").unwrap();



		assert_eq!(3, minimize(&rules, "CRnSiRnFYCaRnFArArFArAl"));


	//	let (_,lookup) = get_rules(&rules);
	//	let data = std::fs::read_to_string("19.txt").unwrap();
//		assert_eq!("CRnSiRnFYCaRnFArArFArAl",reverse(lookup, vec![67, 9020, 8715, 9020, 70, 89, 6499, 9020, 70, 7410, 7410,70,7410,7020]));

//		assert_eq!(509, minimize(&rules, &data));
	}

	// #[test]
	// fn test_part1() {
	// 	let rules = std::fs::read_to_string("19-rules.txt").unwrap();
	// 	let data = std::fs::read_to_string("19.txt").unwrap();
	// 	let map = run(&rules.replace("\r\n", "\n"), &data);
	// 	assert_eq!(509, map.len());
	// }

	// #[test]
	// fn test_minimizebrute() {
	// 	assert_eq!(
	// 		ho("F",6),
	// 		minimize_brute(
	// 			&vec![
	// 				rule("Ca","CaCa"),
	// 				rule("Ca","SiRnMgAr"),
	// 				rule("F","CaF"),
	// 				rule("Ca","SiTh"),
	// 			],
	// 			&tokenize("SiThCaCaSiRnMgArF")));
	// }

	// #[test]
	// fn test_minimizebrute_different_rule_order() {
	// 	assert_eq!(
	// 		ho("F",6),
	// 		minimize_brute(
	// 			&vec![
	// 				rule("Ca","SiRnMgAr"),
	// 				rule("F","CaF"),
	// 				rule("Ca","SiTh"),
	// 				rule("Ca","CaCa"),
	// 			],
	// 			&tokenize("SiThCaCaSiRnMgArF")));
	// }

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
