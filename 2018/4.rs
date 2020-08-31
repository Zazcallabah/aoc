use std::collections::HashMap;
use regex::Regex;

fn main(){
	let guards = parse(read());
	let r = guards.values().enumerate().max_by_key(|p| p.1.sum() ).unwrap();
	println!("part1: {}", (r.1.id as i32) * r.1.max_ix()as i32);
	let r = guards.values().enumerate().max_by_key(|p| p.1.max_val() ).unwrap();
	println!("part2: {}", r.1.id*r.1.max_ix());
}

fn read()-> Vec<String>{
	let data = std::fs::read_to_string("2018/4.txt").unwrap();
	let mut lines : Vec<&str> = data.lines().collect();
	lines.sort_by(|a, b| a.partial_cmp(b).unwrap());
	let mut s : Vec<String> = Vec::new();
	for l in lines {
		s.push(l.to_owned());
	}
	s
}

fn parse(data:Vec<String>) -> HashMap<u16,Guard> {

	let re = Regex::new(r"\[\d+-\d\d-\d\d \d\d:(?P<minute>\d\d)\] (Guard #)?((?P<id>\d+)|(?P<stat>(wakes up|falls asleep)))").unwrap();
	let mut id = 0;
	let mut fell_asleep_at = 0;

	let mut map = HashMap::new();

	for line in data {
		let c = re.captures(&line).unwrap();
		match c.name("id") {
			Some(n) => {
				id = n.as_str().parse::<u16>().unwrap();
			},
			None =>{
				let min = c.name("minute").unwrap().as_str().parse::<u16>().unwrap();
				if c.name("stat").unwrap().as_str() == "falls asleep" {
					fell_asleep_at = min;
				}
				else {
					for m in fell_asleep_at..min {
						map.entry(id).or_insert_with(|| Guard::new(id)).minutes[m as usize] += 1;
					}
				}
			}
		}
	}

	map
}

fn testread()->Vec<String>{
let s = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

	s.lines().map(|l| l.to_owned() ).collect()
}

struct Guard {
	id : u16,
	minutes: Vec<u16>
}
impl Guard {
	fn new(id:u16)->Guard{
		Guard{id,minutes:vec!{0u16;60}}
	}
	fn sum(&self)->u16{
		self.minutes.iter().sum()
	}
	fn max_ix(&self)->u16{
		let mut ix = 0;
		let mut max = 0;

		for i in 0..60 {
			if max < self.minutes[i] {
				ix = i;
				max = self.minutes[i];
			}
		}
		ix as u16
	}
	fn max_val(&self)->u16{
		let mut ix = 0;
		let mut max = 0;

		for i in 0..60 {
			if max < self.minutes[i] {
				ix = i;
				max = self.minutes[i];
			}
		}
		max
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_parse(){
		let g = parse(testread());
		assert_eq!(2,g.get(&10).unwrap().minutes[24]);
	}

	#[test]
	fn test_sum(){
		let g = parse(testread());
		assert_eq!(50,g.get(&10).unwrap().sum());
	}

	#[test]
	fn test_max(){
		let g = parse(testread());
		assert_eq!(24,g.get(&10).unwrap().max_ix());
	}

	#[test]
	fn test_part1(){
		let guards = parse(testread());
		let r = guards.values().enumerate().max_by_key(|p| p.1.sum() ).unwrap();
		assert_eq!(10,r.1.id);
		assert_eq!(24,r.1.max_ix());
	}

	#[test]
	fn test_part2(){
		let guards = parse(testread());
		let r = guards.values().enumerate().max_by_key(|p| p.1.max_val() ).unwrap();
		assert_eq!(99,r.1.id);
		assert_eq!(45,r.1.max_ix());
	}
}
