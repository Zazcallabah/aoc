use regex::Regex;
use std::collections::HashMap;
use permutohedron::heap_recursive;

pub struct Entry {
	name: String,
	next: String,
	gain: i32,
}

pub fn get_data() -> Result<Vec<Entry>,std::io::Error> {
	let data = std::fs::read_to_string("13.txt")?;
	let re = Regex::new(r"([^ ]+) would (lose|gain) ([^ ]+) happiness units by sitting next to ([^.]+).(\r\n)?").unwrap();
	let mut v: Vec<Entry> = Vec::new();
	for cap in re.captures_iter(&data) {
		let name: String = cap[1].to_string();
		let gain: i32 = &cap[3].parse().unwrap() * if &cap[2]=="lose" { -1 } else { 1 };
		let next: String = cap[4].to_string();
		v.push(Entry{ name, next, gain });
	}
	Ok(v)
}

pub fn combine_data() -> HashMap<String,HashMap<String,i32>> {
	let mut entries = get_data().unwrap();
	let mut h :HashMap<String,HashMap<String,i32>> = HashMap::new();

	loop {
		match entries.pop() {
			Some(entry) => {
				if !h.contains_key(&entry.name) {
					h.insert(entry.name.clone(), HashMap::new());
				}
				if let Some(inner) = h.get_mut(&entry.name) {
					inner.insert(entry.next.clone(),entry.gain);
				}
			},
			None => return h,
		}
	}
}

pub fn lookup(map:&HashMap<String,HashMap<String,i32>>,a:&str,b:&str) -> Result<i32,String> {
	if a == "You" || b == "You" {
		return Ok(0i32)
	}
	if let Some(inner) = map.get(a) {
		if let Some(val) = inner.get(b) {
			return Ok(*val)
		}
	}
	Err("bad ix".to_owned())
}

pub fn get_cost(map:&HashMap<String,HashMap<String,i32>>,list:&mut [&str]) -> i32 {
	let mut sum = lookup(&map,list[0],list[list.len()-1]).unwrap() + lookup(&map,list[list.len()-1],list[0]).unwrap();
	for i in 1..list.len() {
		sum += lookup(&map,list[i],list[i-1]).unwrap();
		sum += lookup(&map,list[i-1],list[i]).unwrap();
	}
	sum
}

pub fn do_perm(map:&HashMap<String,HashMap<String,i32>>,mut list:&mut [&str]) -> i32 {
	let mut highscore = -10000i32;
	heap_recursive(&mut list, |permutation| {
		let score = get_cost(&map,permutation);
		if score > highscore {
			highscore = score
		}
	});
	highscore
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test(){
		let map = combine_data();
		assert_eq!(&8usize,&map.len());
		assert_eq!(2,lookup(&map,"Alice","Bob").unwrap());
		assert_eq!(-89,lookup(&map,"Mallory","George").unwrap());
		assert_eq!(65i32,get_cost(&map,&mut ["Alice","Bob","Carol","David","Eric","Frank","George","Mallory"]));
		assert_eq!(733,do_perm(&map,&mut ["Alice","Bob","Carol","David","Eric","Frank","George","Mallory"]));
		assert_eq!(0,lookup(&map,"Alice","You").unwrap());
		assert_eq!(725,do_perm(&map,&mut ["Alice","Bob","Carol","David","Eric","Frank","George","Mallory","You"]));
	}
}
