use std::{collections::HashMap};
use std::collections::VecDeque;


#[derive(Clone)]
struct Unit {
	name: String,
	quantity: u64,
}
impl Unit {
	fn new( data: &str ) -> Unit {
		let r: Vec<&str> = data.trim().split(' ').collect();
		Unit { name: r[1].to_owned(), quantity: r[0].to_owned().parse().unwrap() }
	}
}

#[derive(Clone)]
struct React {
	result: Unit,
	source: Vec<Unit>,
}

impl React {
	fn new( data: &str ) -> React {
		let r: Vec<&str> = data.split(" => ").collect();
		let s: Vec<&str> = r[0].split(',').collect();
		React { result: Unit::new(r[1]), source: s.iter().map(|f| Unit::new(f)).collect() }
	}
}

struct Table {
	entries: Vec<React>
}
impl Table {
	fn new(input: &str) -> Table {
		let entries = input.lines().map( |s| {
			React::new(s)

		}).collect();
		Table{ entries: entries }
	}
	// extract all entries that can be satisfied by the names in param
	// remove them from table and return
	fn extract(&mut self,names: Vec<String>) -> Vec<React> {
		let mut v = Vec::new();
		&self.entries.retain(|r| {
			let matching = r.source.iter()
					.filter( |s| names.contains(&s.name) )
					.count() == r.source.len();
			if matching {
				v.push(r.clone())
			}
			return !matching;
		});

		v
	}
}


struct Lookup {
	storage: HashMap<String,React>,
}

impl Lookup {
	fn get(&self,name:&str) -> &React {
		self.storage.get(name).unwrap()
	}

	fn new(t:Table) -> Lookup {
		let mut storage : HashMap<String,React> = HashMap::new();
		for entry in t.entries {
			storage.insert(entry.result.name.clone(),entry);
		}

		Lookup { storage }
	}
}

fn get_cost_count(input:&str, name: &str, q: u64) -> u64 {
	get_cost(input,name,q).0
}

fn get_cost(input:&str, name: &str, q: u64) -> (u64,HashMap<String,u64>) {

	let table = Table::new(input);
	let l = Lookup::new(table);

	let mut requirements:VecDeque<(String,u64)> = VecDeque::new();
	let mut extras:HashMap<String,u64> = HashMap::new();
	requirements.push_back((name.to_owned(),q));
	let mut ore = 0;

	loop {
		if let Some(req) = requirements.pop_front() {
			if req.0 == "ORE" {
				ore += req.1;
			}
			else {
				let producer = l.get(&req.0);

				let existing = extras.get(&req.0).unwrap_or(&0).to_owned() as i64;
				// we want req.1. we have existing
				// we need to create req.1-existing
				let need : i64 = req.1 as i64 - existing;
				if need <= 0 {
					// we already have all we need, and just push back the remainder
					extras.insert(req.0, existing as u64 - req.1);
				}
				else {
					// each run gives result.quantity
					// rq * x > need
					// x > need/rq, round up

					let rq = producer.result.quantity as i64;
					let runs = (need + rq -1) / rq;
					let totalgain = rq*runs;
					let remainder = totalgain - need;
					assert!( remainder >= 0 );
					extras.insert(req.0, remainder as u64);

					for component in producer.source.iter() {
						requirements.push_back((component.name.to_owned(),runs as u64 * component.quantity));
					}
				}
			}
		}
		else {
			return (ore,extras);
		}
	}
}
fn main() {
	let t =&std::fs::read_to_string("2019/14.txt").unwrap();

	let ore = get_cost_count(t,"FUEL",1);
	println!("ore: {}", ore);

	let mut fuelcount = 1u64;
	let limit = 1_000_000_000_000u64;


	println!("find limit");
	loop {
		fuelcount *= 2;
		let result = get_cost_count(t,"FUEL", fuelcount);
		println!("{} fuel -> {} ore", fuelcount, result);
		if result > limit {
			break;
		}
	}

	let mut max = fuelcount;
	let mut min = fuelcount / 2;

	println!("bisect search");
	loop {
		let pivot = (max-min)/2 + min;
		if pivot == min {
			println!("{} fuel",pivot);
			break;
		} else if pivot == max {
			println!("{} fuel",pivot-1);
			break;
		}
		let result = get_cost_count(t,"FUEL", pivot);
		println!("{}-{} {} fuel -> {} ore", min,max,pivot, result);
		if result > limit {
			max = pivot;
		} else if result < limit {
			min = pivot;
		} else { panic!("aaaaaah"); }
	}
	//

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
	#[test]
	fn test_count_ore(){

		let t = r"10 ORE => 10 A
		1 ORE => 1 B
		7 A, 1 B => 1 C
		7 A, 1 C => 1 D
		7 A, 1 D => 1 E
		7 A, 1 E => 1 FUEL";

		assert_eq!(1,get_cost_count(t,"B", 1));
		assert_eq!(10,get_cost_count(t,"A", 1));
		assert_eq!(10,get_cost_count(t,"A", 10));
		assert_eq!(11,get_cost_count(t,"C", 1));
		assert_eq!(22,get_cost_count(t,"C", 2));
		assert_eq!(21,get_cost_count(t,"D", 1));
		assert_eq!(31,get_cost_count(t,"FUEL", 1));
	}

	#[test]
	fn test_count2(){

		let t = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

		assert_eq!(165,get_cost_count(t,"FUEL", 1));
	}
	#[test]
	fn test_count3(){

		let t = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

		assert_eq!(2210736,get_cost_count(t,"FUEL", 1));
	}

    #[test]
    fn test_table() {
		let t = Table::new("27 SRLP, 12 KWQSC, 14 ZNBSN, 33 HQTPN, 3 HWFQ, 23 QZCZ, 6 ZPDN, 32 RJQW, 3 GDXG => 1 FUEL");
		assert_eq!("FUEL",t.entries[0].result.name);
	}

	#[test]
	fn test_unique() {
		let t = Table::new(	&std::fs::read_to_string("2019/14.txt").unwrap());
		let mut testset = HashSet::new();
		for entry in t.entries{
			testset.insert( entry.result.name.to_owned() );
		}
	}
}
