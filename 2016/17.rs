extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

struct Path {
	node: (i8, i8),
	steps: String,
}

impl Path {
	fn start() -> Path {
		Path::new(0, 0, &"", None)
	}
	fn new(x: i8, y: i8, path: &str, step: Option<char>) -> Path {
		let node = (x, y);
		let steps = if let Some(s) = step {
			let mut tmp = path.to_owned();
			tmp.push(s);
			tmp
		} else {
			path.to_owned()
		};
		Path { node, steps }
	}

	fn is_done(&self) -> bool {
		self.node.0 == 3 && self.node.1 == 3
	}

	fn next(&self, key: &str) -> Vec<Path> {
		let mut data = key.to_owned();
		data.push_str(&self.steps);
		let mut digest = Md5::new();
		digest.input_str(&data);
		let md5 = digest.result_str();
		let b: Vec<u8> = md5.bytes().collect();
		let mut ret = Vec::new();
		if b[0] > 97 && self.node.1 > 0 {
			ret.push(Path::new(
				self.node.0,
				self.node.1 - 1,
				&self.steps,
				Some('U'),
			));
		}
		if b[1] > 97 && self.node.1 < 3 {
			ret.push(Path::new(
				self.node.0,
				self.node.1 + 1,
				&self.steps,
				Some('D'),
			));
		}
		if b[2] > 97 && self.node.0 > 0 {
			ret.push(Path::new(
				self.node.0 - 1,
				self.node.1,
				&self.steps,
				Some('L'),
			));
		}
		if b[3] > 97 && self.node.0 < 3 {
			ret.push(Path::new(
				self.node.0 + 1,
				self.node.1,
				&self.steps,
				Some('R'),
			));
		}

		ret
	}
}

fn depth_longest(mut current: Vec<Path>, key: &str, mut longest: Option<Path>) -> Path {
	let mut next = Vec::new();

	if current.is_empty() {
		return longest.unwrap();
	}

	for p in current.into_iter() {
		if p.is_done() {
			longest = Some(p);
		} else {
			next.extend(p.next(key));
		}
	}

	depth_longest(next, key, longest)
}
fn depth_first(current: Vec<Path>, key: &str) -> Path {
	let mut next = Vec::new();

	for p in current.into_iter() {
		if p.is_done() {
			return p;
		}
		next.extend(p.next(key));
	}

	depth_first(next, key)
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_firststep() {
		let p = Path::new(0, 0, &"", None);
		let next = p.next("hijkl");
		assert_eq!(1, next.len());
		assert_eq!("D", next[0].steps);
		assert_eq!((0, 1), next[0].node);
	}

	#[test]
	fn test_secondstep() {
		let p = Path::new(0, 1, &"D", None);
		let next = p.next("hijkl");
		assert_eq!(2, next.len());
		assert_eq!("DU", next[0].steps);
		assert_eq!((0, 0), next[0].node);
		assert_eq!("DR", next[1].steps);
		assert_eq!((1, 1), next[1].node);
	}

	#[test]
	fn test_demopasses() {
		assert_eq!(
			"DDRRRD",
			depth_first(vec![Path::start()], &"ihgpwlah").steps
		);
		assert_eq!(
			"DDUDRLRRUDRD",
			depth_first(vec![Path::start()], &"kglvqrro").steps
		);
		assert_eq!(
			"DRURDRUDDLLDLUURRDULRLDUUDDDRR",
			depth_first(vec![Path::start()], &"ulqzkmiv").steps
		);
	}

	#[test]
	fn test_longest() {
		assert_eq!(
			370,
			depth_longest(vec![Path::start()], &"ihgpwlah", None).steps.len()
		);
		assert_eq!(
			492,
			depth_longest(vec![Path::start()], &"kglvqrro", None).steps.len()
		);
	}
}

fn main() {
	let p = depth_first(vec![Path::start()], &"hhhxzeay");
	println!("depth first: {}", p.steps);
	let p = depth_longest(vec![Path::start()], &"hhhxzeay", None);
	println!("longest: {} steps", p.steps.len());
}
