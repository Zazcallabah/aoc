
struct Range {
	from: u32,
	to: u32,
}

impl Range {
	fn new(line:&str) -> Range {
		let ix = line.find('-').unwrap();
		let from = line[..ix].parse().unwrap();
		let to = line[(ix+1)..].parse().unwrap();
		Range{from,to}
	}
}

fn first(blacklist:&[Range])->u32{
	let mut candidate = 0u32;
	for range in blacklist {
		if range.from > candidate {
			return candidate
		}
		if range.to > candidate {
			candidate = range.to + 1;
		}
	}
	panic!("no open ranges found");
}

fn main(){
	let mut blacklist = std::fs::read_to_string("20.txt").unwrap()
		.lines()
		.map(|l| Range::new(l) )
		.collect::<Vec<Range>>();

		blacklist.sort_by(|a,b| a.from.partial_cmp(&b.from).unwrap() );

		println!("first open: {}", first(&blacklist));

		println!("total count: {}",count(&blacklist));

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_newrange(){
		let r = Range::new("0-33");
		assert_eq!(0,r.from);
		assert_eq!(33,r.to);
	}

	#[test]
	fn test_allrange() {
		let r = vec![Range::new("0-4294967295")];
		assert_eq!(0,count(&r));
	}
	#[test]
	fn test_nogap() {
		let r = vec![Range::new("0-9"),Range::new("10-4294967295")];
		assert_eq!(0,count(&r));
	}
	#[test]
	fn test_norules() {
		assert_eq!(std::u32::MAX,count(&Vec::new()));
	}
	#[test]
	fn test_midgap() {
		let r = vec![Range::new("0-7"),Range::new("10-4294967295")];
		assert_eq!(2,count(&r));
	}
	#[test]
	fn test_initialgap() {
		let r = vec![Range::new("10-4294967295")];
		assert_eq!(10,count(&r));
	}
	#[test]
	fn test_trailinggap() {
		let r = vec![Range::new("0-4294967294")];
		assert_eq!(1,count(&r));
	}

	#[test]
	fn test_manygaps() {
		let r = vec![
			Range::new("0-7"),
			Range::new("10-15"),
			Range::new("21-4294967295")
		];
		assert_eq!(7,count(&r));
	}


	#[test]
	fn test_overlap() {
		let r = vec![
			Range::new("0-7"),
			Range::new("1-2"),
			Range::new("10-15"),
			Range::new("21-4294967295")
		];
		assert_eq!(7,count(&r));
	}

}


fn count(blacklist:&[Range])->u32{

	let mut candidate = 0u32;
	let mut counter = 0u32;
	let mut highest = 0u32;

	for range in blacklist {
		if range.from > candidate {
			counter += range.from-candidate;
		}
		if range.to > highest {
			highest = range.to;
		}
		if range.to == std::u32::MAX {
			break;
		}
		if range.to > candidate {
			candidate = range.to + 1;
		}
	}
	counter + (std::u32::MAX - highest)
}