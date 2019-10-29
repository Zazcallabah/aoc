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

fn main() {
	let rules = r"Al => ThF
Al => ThRnFAr
B => BCa
B => TiB
B => TiRnFAr
Ca => CaCa
Ca => PB
Ca => PRnFAr
Ca => SiRnFYFAr
Ca => SiRnMgAr
Ca => SiTh
F => CaF
F => PMg
F => SiAl
H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
H => HCa
H => NRnFYFAr
H => NRnMgAr
H => NTh
H => OB
H => ORnFAr
Mg => BF
Mg => TiMg
N => CRnFAr
N => HSi
O => CRnFYFAr
O => CRnMgAr
O => HP
O => NRnFAr
O => OTi
P => CaP
P => PTi
P => SiRnFAr
Si => CaSi
Th => ThCa
Ti => BP
Ti => TiTi
e => HF
e => NAl
e => OMg";

	let data = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";
	let map = run(rules, data);
	println!("length {}", map.len());

	let input = parse(&rules);
	let shortest = backwards(&input, &mut String::from(data), 0, 10000000);

	println!("shortest {}", shortest);
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

fn backwards(rules: &Vec<(&str, &str)>, data: &mut String, depth: i32, mut minimum: i32) -> i32 {
	let newdepth = depth + 1;
	if depth % 10 == 9 {
		print!("{} ",depth);
		if( depth
	}
	for i in 0..data.len() {
		if newdepth >= minimum {
			break;
		}
		for (from, to) in rules {
			if newdepth >= minimum {
				break;
			}
			let endmark = i + to.len();
			if endmark > data.len() {
				continue;
			}
			let slice = &data[i..endmark];
			if &slice == to {
				data.replace_range(i..endmark, from);
				if data == "e" {
					if newdepth < minimum {
						minimum = newdepth;
						println!("{}", minimum);
						data.replace_range(i..i + from.len(), to);
						continue;
					}
				}
				let searchmin = backwards(&rules, data, newdepth, minimum);
				data.replace_range(i..i + from.len(), to);
				if searchmin < minimum {
					println!("{}", minimum);

					minimum = searchmin;
				}
			}
		}
	}
	minimum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_backwards() {
		let result = backwards(
			&vec![("a", "ab"), ("e", "a")],
			&mut String::from("ab"),
			0,
			1000000,
		);
		assert_eq!(2, result);
	}

	#[test]
	fn test_backwards_complex() {
		let rules = vec![
			("e", "O"),
			("e", "H"),
			("H", "HO"),
			("H", "OH"),
			("O", "HH"),
		];
		assert_eq!(
			6,
			backwards(&rules, &mut String::from("HOHOHO"), 0, 1000000)
		);
		assert_eq!(3, backwards(&rules, &mut String::from("HOH"), 0, 1000000));
	}

	#[test]
	fn test_backwards_finds_shortest2() {
		let rules = vec![("e", "HO"), ("H", "HH"), ("O", "OO")];
		assert_eq!(3, backwards(&rules, &mut String::from("HHOO"), 0, 1000000));
	}

	#[test]
	fn test_backwards_finds_shortest() {
		let rules = vec![
			("e", "O"),
			("e", "H"),
			("H", "HO"),
			("H", "OH"),
			("O", "HH"),
			("e", "HOHOHO"),
		];
		assert_eq!(
			1,
			backwards(&rules, &mut String::from("HOHOHO"), 0, 1000000)
		);
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
