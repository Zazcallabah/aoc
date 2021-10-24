use std::{collections::HashMap, convert::TryInto};


#[derive(Clone)]
struct Unit {
	name: String,
	quantity: u32,
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
	fn cost( &self, q: u32, lookup: &mut Lookup) -> u32 {

		// do we have anything stored?

		let existing = lookup.store.get(&self.result.name);

		// we want q. we have existing
		// we need to create q-existing
		let need = q - existing;


		// each run gives result.quantity
		// rq * x > need
		// x > need/rq, round up


		let rq = &self.result.quantity;
		let runs = (need + rq -1) / rq;
		let totalgain = rq*runs;
		let remainder = totalgain - need;
		if remainder > 0 {
			lookup.store.set(&self.result.name, remainder)
		}

		let mut sum = 0;
		for s in &self.source {
			if s.name == "ORE" {
				sum += runs*s.quantity;
			} else {
				let b = lookup.best(&s.name,runs*s.quantity);
				sum += b.1.cost(b.0, lookup);
			}
		}
		return sum;
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

fn main() {
	&std::fs::read_to_string("2019/14.txt").unwrap();
//
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_table() {
		let t = Table::new("27 SRLP, 12 KWQSC, 14 ZNBSN, 33 HQTPN, 3 HWFQ, 23 QZCZ, 6 ZPDN, 32 RJQW, 3 GDXG => 1 FUEL");
		assert_eq!("FUEL",t.entries[0].result.name);
	}

	#[test]
	fn test_extract() {
		let mut t = Table::new(
r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL");
		let ore = t.extract(vec!["ORE".to_owned()]);
		assert_eq!(2,ore.len());
		assert_eq!(4,t.entries.len());
	}

	#[test]
	fn test_best() {
		let mut t = Table::new(
r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL");
		let mut l = Lookup::new();
		for r in t.entries {
			l.add(r)
		}
		let orecost = l.best("ORE", 1);
		assert_eq!(1,orecost.0);
		let bcost = l.best("B", 1);
		assert_eq!(1,bcost.0);
		let bcost = l.best("B", 10);
		assert_eq!(10,bcost.0);
		let acost = l.best("A", 1);
		assert_eq!(10,acost.0);
		let acost = l.best("A", 10);
		assert_eq!(10,acost.0);
		let acost = l.best("C", 1);
		assert_eq!(11,acost.0);
		let acost = l.best("C", 2);
		assert_eq!(22,acost.0);
		let acost = l.best("D", 1);
		assert_eq!(22,acost.0);
	}
}
struct Store {
	list: HashMap<String,u32>
}
impl Store {
	fn new()->Store{
		Store { list: HashMap::new() }
	}
	fn add(&mut self, name: &str) {
		self.list.entry(name.to_owned()).or_default();
	}
	fn get(&self, name:&str) -> u32 {
		self.list.get(name).unwrap().clone()
	}
	fn set(&mut self, name: &str, amount: u32) {
		let e: &mut u32 = self.list.get_mut(name).unwrap();
		*e = amount;
	}
}
struct Lookup {
	paths: HashMap<String,Vec<React>>,
	store: Store
}

impl Lookup {
	fn best( &mut self, want: &str, q: u32 ) -> (u32,React) {
		if want == "ORE" {
			(
				q,
				React {
				result: Unit {
					name: "ORE".to_owned(),
					quantity: q
				},
				source: Vec::new()
				}
			)
		} else {
			let options = &self.paths.get(want).unwrap();
			if options.len() == 1 {
				let choice = options.last().unwrap();
				let cost = choice.cost(q,self);
				(cost,choice.clone())
			}
			else {
				let mut cost = u32::MAX;
				let choice = options.iter().reduce(|best,current| {
					let c = &current.cost(q, self);
					if *c < cost {
						current
					} else {
						best
					}
				});
				(
					cost,
					choice.unwrap().clone()
				)
			}
		}

	}

	fn add( &mut self, r: React ) {
		self.store.set(&r.result.name,0);
		let list = self.paths.entry(r.result.name.clone()).or_default();
		list.push(r);
	}

	fn new() -> Lookup {
		Lookup { paths: HashMap::new(), store: Store::new() }
	}
}
