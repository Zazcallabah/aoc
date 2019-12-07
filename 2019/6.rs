use std::collections::HashMap;

type Map = HashMap<String,Stellar>;

struct Stellar {
	name:String,
	parent:String,
	children:Vec<String>,
}

impl Stellar {
	fn new(name:String) -> Stellar {
		Stellar{name,children: Vec::new(),parent:"".to_owned()}
	}
}

fn map(data:&str) -> Map {
	let mut objects : Map = HashMap::new();
	for mut l in data.lines().map(|l| l.split(')') ) {
		let parentstr = l.next().unwrap();
		let childstr = l.next().unwrap();
		if objects.contains_key(childstr) {
			let mut c = objects.get_mut(childstr).unwrap();
			c.parent = parentstr.to_owned();
		}
		else {
			let mut child = Stellar::new(childstr.to_owned());
			child.parent = parentstr.to_owned();
			objects.insert(child.name.clone(), child);
		}

		let parent = objects.entry(parentstr.to_owned()).or_insert_with(|| Stellar::new(parentstr.to_owned()) );
		parent.children.push(childstr.to_owned());
	}
	objects
}

fn ancestor(map:&Map)-> String {
	let (_,anc) = map.iter().find(|(_,s)| s.parent == "" ).unwrap();
	anc.name.clone()
}

fn ancestry(map:&Map,key:&str) -> Vec<String> {
	let mut v = Vec::new();
	let mut k = key.to_owned();

	loop {
		let n = map.get(&k).unwrap();
		v.push(n.name.clone());
		if n.parent == "" {
			return v
		}
		k = n.parent.clone();
	}
}

fn count(map:&Map,level:usize,start:&str) -> usize {
	let mut sum = level;
	for c in &map.get(start).unwrap().children {
		sum += count(map,level+1,c);
	}
	sum
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_make_map(){
let m = map(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");
		let c = m.get("COM").unwrap();
		assert_eq!("COM",c.name);
		assert_eq!("",c.parent);
		assert_eq!(vec!["B"],c.children);
	}
	#[test]
	fn test_find_ancestor(){
let m = map(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");
		assert_eq!("COM",ancestor(&m));
	}
	#[test]
	fn test_count(){
let m = map(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");
		assert_eq!(42,count(&m,0,&"COM"));
	}

	#[test]
	fn test_ancestry(){
let m = map(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");
		assert_eq!(vec!["J","E","D","C","B","COM"],ancestry(&m,&"J"));
	}

	#[test]
	fn test_travel(){
let m = map(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN");
		assert_eq!(4,transfer(&m,"YOU","SAN"));
	}}




fn main(){
	let m = map(&std::fs::read_to_string("2019/6.txt").unwrap());

	println!("part 1: {}",count(&m,0,&ancestor(&m)));
	println!("part 2: {}",transfer(&m,"YOU","SAN"));
}

fn transfer(map:&Map,from:&str,to:&str) -> usize {
	let anc_a = ancestry(&map,&from);
	let anc_b = ancestry(&map,&to);

	for (i,a) in anc_a.iter().enumerate() {
		for (j,b) in anc_b.iter().enumerate() {
			if a == b {
				return i+j - 2
			}
		}
	}
	panic!("no common ancestor found");
}
