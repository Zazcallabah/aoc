use std::{collections::HashMap, str::Lines};
#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn make_nodes(lines: Lines) -> HashMap<String, (String, String)> {
    lazy_static! {
        static ref LINE: Regex =
            Regex::new("([0-9A-Z]{3}) = \\(([0-9A-Z]{3}), ([0-9A-Z]{3})\\)").unwrap();
    }
    let mut map = HashMap::new();
    for line in lines {
        //FSH = (CGN, NDK)

        match LINE.captures(line) {
            None => panic!("invalid input"),
            Some(c) => {
                let addr = c.get(1).unwrap().as_str().to_string();
                let left = c.get(2).unwrap().as_str().to_string();
                let right = c.get(3).unwrap().as_str().to_string();
                map.insert(addr, (left, right));
            }
        }
    }
    return map;
}

fn walk(path: &str, map: HashMap<String, (String, String)>) -> u32 {
    let mut took = 0;
    let mut at = "AAA";
    let mut pathpointer = 0;
    loop {
        if at == "ZZZ" {
            return took;
        }

        let curr = map.get(at).unwrap();

        let slicedpath = &path[pathpointer..=pathpointer];

        if slicedpath == "L" {
            at = curr.0.as_str();
        } else {
            at = curr.1.as_str();
        }

        took += 1;
        pathpointer = (pathpointer + 1) % path.len()
    }
}
fn is_start(n: &str) -> bool {
    &n[2..=2] == "A"
}
fn is_end(n: &str) -> bool {
    &n[2..=2] == "Z"
}
fn get_starts(n: Vec<&String>) -> Vec<String> {
    n.iter()
        .filter(|s| is_start(s))
        .map(|s| s.to_string())
        .collect()
}
fn all_end(n: &[String]) -> bool {
    n.iter().all(|s| is_end(s))
}
fn walk_all(path: &str, map: HashMap<String, (String, String)>) -> u32 {
    let mut took = 0;
    let mut at_many: Vec<String> = get_starts(map.keys().collect());
    let mut pathpointer = 0;
    loop {
        if took % 100000 == 0 {
            println!("progress: {}", took);
        }
        if all_end(&at_many) {
            return took;
        }

        for a in 0..at_many.len() {
            let at = at_many.get_mut(a).unwrap();
            let curr = map.get(at).unwrap();

            let slicedpath = &path[pathpointer..=pathpointer];

            if slicedpath == "L" {
                at_many[a] = curr.0.to_string();
            } else {
                at_many[a] = curr.1.to_string();
            }
        }

        took += 1;
        pathpointer = (pathpointer + 1) % path.len()
    }
}
fn walk_to_any_z(path: &str, start: &str, map: &HashMap<String, (String, String)>) -> u32 {
    let mut took = 0;
    let mut at = start.to_owned();
    let mut pathpointer = 0;
    loop {
        if is_end(&at) {
            return took;
        }

        let curr = map.get(&at).unwrap();

        let slicedpath = &path[pathpointer..=pathpointer];

        if slicedpath == "L" {
            at = curr.0.to_owned();
        } else {
            at = curr.1.to_owned();
        }

        took += 1;
        pathpointer = (pathpointer + 1) % path.len()
    }
}
fn all_path_lengths(path: &str, map: HashMap<String, (String, String)>) -> Vec<u64> {
    let mut at_many: Vec<String> = get_starts(map.keys().collect());

    let mut r: Vec<u64> = Vec::new();
    for a in 0..at_many.len() {
        let at = at_many.get_mut(a).unwrap();
        let took = walk_to_any_z(path, at, &map);
        r.push(took as u64);
    }
    r
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut r = 0;
    let mut a = first;
    let mut b = second;
    while (a % b) > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    return b;
}
fn lcm(first: u64, second: u64) -> u64 {
    let gcd = gcd(first, second);
    return (first * second) / gcd;
}
fn gcd_many(n: &[u64]) -> u64 {
    let mut r = n[0];
    for i in 1..n.len() {
        r = gcd(r, n[i]);
    }
    r
}
fn lcm_many(n: &[u64]) -> u64 {
    let mut r = n[0];
    for i in 1..n.len() {
        r = lcm(r, n[i]);
    }
    r
}
fn main() {
    let data = std::fs::read_to_string("2023/8.txt").unwrap();
    let mut lines = data.lines().into_iter();
    let path = lines.next().unwrap();
    let _ = lines.next();

    let nodes = make_nodes(lines);
    let result = walk(path, nodes);
    println!("single: {}", result);

    let data = std::fs::read_to_string("2023/8.txt").unwrap();
    let mut lines = data.lines().into_iter();
    let path = lines.next().unwrap();
    let _ = lines.next();
    let nodes = make_nodes(lines);
    let result = all_path_lengths(path, nodes);
    println!("paths: {:?}", result);
    let gcd = gcd_many(&result);
    println!("gcd: {:?}", gcd);
    let lcm = lcm_many(&result);
    println!("lcm: {:?}", lcm);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_walk() {
        let path = "LLR";
        let m = "AAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        let n = make_nodes(m.lines());
        let w = walk(path, n);
        assert_eq!(6, w)
    }
    #[test]
    fn test_can_walk_all() {
        let path = "LR";
        let m ="11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let n = make_nodes(m.lines());
        let w = walk_all(path, n);
        assert_eq!(6, w)
    }
    #[test]
    fn test_all_paths_length() {
        let path = "LR";
        let m ="11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let n = make_nodes(m.lines());
        let w = all_path_lengths(path, n);
        assert_eq!(2, w.len());
        assert_eq!(2, w[0]);
        assert_eq!(3, w[1]);
    }
}
