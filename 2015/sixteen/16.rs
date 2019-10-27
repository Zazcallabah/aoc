use regex::Regex;

pub struct Entry {
	id: u16,
	children: Option<u8>,
	cats: Option<u8>,
	samoyeds: Option<u8>,
	pomeranians: Option<u8>,
	akitas: Option<u8>,
	vizslas: Option<u8>,
	goldfish: Option<u8>,
	trees: Option<u8>,
	cars: Option<u8>,
	perfumes: Option<u8>,
}

pub fn get_match(cap:&regex::Captures,name:&str) -> Option<u8> {
	if let Some(m) = cap.name(&name) {
		Some( m.as_str().parse().unwrap() )
	}
	else {
		None
	}
}

pub fn get_data() -> Result<Vec<Entry>, std::io::Error> {
	let data = std::fs::read_to_string("16.txt")?;

	let re = Regex::new(r"(?x)
		Sue\s(?P<id>\d+):(\s(
			children:\s(?P<children>\d+)|
			cats:\s(?P<cats>\d+)|
			samoyeds:\s(?P<samoyeds>\d+)|
			pomeranians:\s(?P<pomeranians>\d+)|
			akitas:\s(?P<akitas>\d+)|
			vizslas:\s(?P<vizslas>\d+)|
			goldfish:\s(?P<goldfish>\d+)|
			trees:\s(?P<trees>\d+)|
			cars:\s(?P<cars>\d+)|
			perfumes:\s(?P<perfumes>\d+)
		),?){3}(\r\n)?").unwrap();
	let mut v: Vec<Entry> = Vec::new();
	for cap in re.captures_iter(&data) {
		let id: u16 = cap.name("id").unwrap().as_str().parse().unwrap();
		let children = get_match(&cap,"children");
		let cats = get_match(&cap,"cats");
		let samoyeds = get_match(&cap,"samoyeds");
		let pomeranians = get_match(&cap,"pomeranians");
		let akitas = get_match(&cap,"akitas");
		let vizslas = get_match(&cap,"vizslas");
		let goldfish = get_match(&cap,"goldfish");
		let trees = get_match(&cap,"trees");
		let cars = get_match(&cap,"cars");
		let perfumes = get_match(&cap,"perfumes");

		v.push(Entry{
			id,
			children,
			cats,
			samoyeds,
			pomeranians,
			akitas,
			vizslas,
			goldfish,
			trees,
			cars,
			perfumes
		});
	}
	Ok(v)
}

pub fn is_valid( entry: &Entry, outdated:bool ) -> bool {
	if let Some(x) = entry.children {
		if x != 3 {
			return false
		}
	}
	if let Some(x) = entry.cats {
		if outdated && x <= 7 || !outdated && x != 7 {
			return false
		}
	}
	if let Some(x) = entry.samoyeds {
		if x != 2 {
			return false
		}
	}
	if let Some(x) = entry.pomeranians {
		if outdated && x >= 3 || !outdated && x != 3 {
			return false
		}
	}
	if let Some(x) = entry.akitas {
		if x != 0 {
			return false
		}
	}
	if let Some(x) = entry.vizslas {
		if x != 0 {
			return false
		}
	}
	if let Some(x) = entry.goldfish {
		if outdated && x >= 5 || !outdated && x != 5 {
			return false
		}
	}
	if let Some(x) = entry.trees {
		if outdated && x <= 3 || !outdated && x != 3 {
			return false
		}
	}
	if let Some(x) = entry.cars {
		if x != 2 {
			return false
		}
	}
	if let Some(x) = entry.perfumes {
		if x != 1 {
			return false
		}
	}
	return true
}

fn main() {
	let data = get_data().unwrap();
	for sue in &data {
		if is_valid(&sue,false) {
			println!("Sue {}", &sue.id);
		}
	}
	for sue in &data {
		if is_valid(&sue,true) {
			println!("outdated Sue {}", &sue.id);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_isvalid() {
		assert!(is_valid(&Entry{
			id: 0,
			children: Some(3),
			cats: Some(7),
			samoyeds: Some(2),
			pomeranians: Some(3),
			akitas: Some(0),
			vizslas: Some(0),
			goldfish: Some(5),
			trees: Some(3),
			cars: Some(2),
			perfumes: Some(1)
		},false));
		assert!(is_valid(&Entry{
			id: 0,
			children: None,
			cats: None,
			samoyeds: None,
			pomeranians: None,
			akitas: None,
			vizslas: None,
			goldfish: Some(5),
			trees: Some(3),
			cars: Some(2),
			perfumes: Some(1)
		},false));
		assert!(!is_valid(&Entry{
			id: 0,
			children: None,
			cats: None,
			samoyeds: None,
			pomeranians: None,
			akitas: None,
			vizslas: None,
			goldfish: None,
			trees: Some(3),
			cars: Some(3),
			perfumes: Some(1)
		},false));
	}

	#[test]
	fn test_regex() {
		let data = get_data().unwrap();

		assert_eq!(1u16,data[0].id);
		assert_eq!(499u16,data[498].id);
		assert_eq!(500u16,data[499].id);

		assert_eq!(Some(2),data[499].cats);
		assert_eq!(Some(9),data[499].goldfish);
		assert_eq!(Some(8),data[499].children);
		assert_eq!(None,data[499].samoyeds);
	}
}
