#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};

use regex::Regex;
struct WorkSet {
    ratings: Vec<Rating>,
    workflows: HashMap<String, Workflow>,
}
struct Rule {
    op: Option<Op>,
    result: String,
    category: Option<Category>,
    limit: u32,
}
struct Limits {
    x: Vec<Range>,
    m: Vec<Range>,
    a: Vec<Range>,
    s: Vec<Range>,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Range {
    from: u32,
    to: u32,
}
impl Range {
    fn new(from: u32, to: u32) -> Range {
        Range { from, to }
    }
    fn delta(&self) -> u64 {
        self.to as u64 - self.from as u64
    }
}
impl Limits {
    fn new() -> Limits {
        Limits {
            x: Vec::new(),
            m: Vec::new(),
            a: Vec::new(),
            s: Vec::new(),
        }
    }
}

struct Workflow {
    name: String,
    accept_all: bool,
    reject_all: bool,
    rules: Vec<Rule>,
}
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Lt,
    Gt,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
impl Category {
    fn from(m: Option<regex::Match>) -> Option<Category> {
        if let Some(mm) = m {
            match mm.as_str() {
                "x" => Some(Category::X),
                "m" => Some(Category::M),
                "a" => Some(Category::A),
                "s" => Some(Category::S),
                _ => panic!("invalid category"),
            }
        } else {
            None
        }
    }
}
impl Op {
    fn from(m: Option<regex::Match>) -> Option<Op> {
        if let Some(mm) = m {
            match mm.as_str() {
                ">" => Some(Op::Gt),
                "<" => Some(Op::Lt),
                _ => panic!("invalid op"),
            }
        } else {
            None
        }
    }
}

lazy_static! {
    static ref PARSE_W: Regex = Regex::new(r"([^{]+)\{([^}]+)\}").unwrap();
    static ref PARSE_R: Regex = Regex::new(r"([xmas])([><])([0-9]+):(\w+)").unwrap();
    static ref PARSE_RATING: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
}

impl std::fmt::Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let d: String = if self.accept_all {
            "A".to_owned()
        } else if self.reject_all {
            "R".to_owned()
        } else {
            " ".to_owned()
        };
        let deps: Vec<String> = self.rules.iter().map(|r| r.result.to_owned()).collect();

        write!(f, "{} ({}) {{{}}}", self.name, d, deps.join(","))
    }
}
impl Workflow {
    fn new(data: &str) -> Workflow {
        let cap = PARSE_W.captures(data).unwrap();
        let name = cap.get(1).unwrap().as_str().to_owned();
        let rules_str = cap.get(2).unwrap().as_str();
        let rules: Vec<Rule> = rules_str.split(",").map(Rule::new).collect();

        Workflow {
            name,
            rules,
            accept_all: false,
            reject_all: false,
        }
    }
    fn find_early_outs(&mut self) {
        self.accept_all = self.rules.iter().all(|r| r.result == "A");
        self.reject_all = self.rules.iter().all(|r| r.result == "R");
    }
    fn run(&self, rating: &Rating) -> String {
        for rule in &self.rules {
            if let Some(op) = rule.op {
                let limit = match rule.category {
                    Some(Category::X) => rating.x,
                    Some(Category::M) => rating.m,
                    Some(Category::A) => rating.a,
                    Some(Category::S) => rating.s,
                    None => panic!("invalid configuration"),
                };
                if op == Op::Gt && limit > rule.limit {
                    return rule.result.clone();
                } else if op == Op::Lt && limit < rule.limit {
                    return rule.result.clone();
                }
            } else {
                return rule.result.clone();
            }
        }
        panic!("invalid workflow end");
    }
}

impl Rule {
    fn new(data: &str) -> Rule {
        if let Some(cap) = PARSE_R.captures(data) {
            let op = Op::from(cap.get(2));
            let result = cap.get(4).unwrap().as_str().to_owned();
            let category = Category::from(cap.get(1));
            let limit: u32 = cap.get(3).unwrap().as_str().parse().unwrap();

            Rule {
                op,
                result,
                category,
                limit,
            }
        } else {
            Rule {
                op: None,
                result: data.to_owned(),
                category: None,
                limit: 0,
            }
        }
    }
}
impl Rating {
    fn new(data: &str) -> Rating {
        let cap = PARSE_RATING.captures(data).unwrap();
        let x: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let m: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let a: u32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let s: u32 = cap.get(4).unwrap().as_str().parse().unwrap();
        Rating { x, m, a, s }
    }
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
impl std::fmt::Display for WorkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // for row in &self.workflows {
        //     if let Err(e) = writeln!(f, "{}", row.1) {
        //         return Err(e);
        //     }
        // }
        let counts = self.count_early_outs();
        writeln!(f, "A:{} R:{}", counts.0, counts.1)
    }
}
impl WorkSet {
    fn new(data: &str) -> WorkSet {
        let mut spl = data.split("\n\n").into_iter();
        let workflows = spl
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let w = Workflow::new(l);
                return (w.name.clone(), w);
            })
            .collect();
        let ratings = spl
            .next()
            .unwrap()
            .lines()
            .map(|l| Rating::new(l))
            .collect();

        WorkSet { ratings, workflows }
    }
    fn run(&self, rating: &Rating) -> bool {
        let mut check = self.workflows.get("in").unwrap();

        loop {
            let result = check.run(rating);
            if result == "R" {
                return false;
            } else if result == "A" {
                return true;
            }
            if let Some(c) = self.workflows.get(&result) {
                check = c;
            } else {
                panic!("invalid workflow received")
            }
        }
    }
    fn summarize(&self) -> u32 {
        let mut sum = 0;
        for r in &self.ratings {
            if self.run(r) {
                sum += r.sum();
            }
        }
        sum
    }
    fn count_early_outs(&self) -> (u32, u32) {
        let mut accept = 0;
        let mut reject = 0;
        for wf in &self.workflows {
            if wf.1.accept_all {
                accept += 1;
            } else if wf.1.reject_all {
                reject += 1
            }
        }
        (accept, reject)
    }
    fn find_early_outs(&mut self) {
        for wf in self.workflows.values_mut() {
            wf.find_early_outs();
        }
    }
    fn minimize(&mut self) {
        self.find_early_outs();
        let accept_early: HashSet<String> = self
            .workflows
            .iter()
            .filter_map(|f| {
                if f.1.accept_all {
                    Some(f.0.clone())
                } else {
                    None
                }
            })
            .collect();
        let reject_early: HashSet<String> = self
            .workflows
            .iter()
            .filter_map(|f| {
                if f.1.reject_all {
                    Some(f.0.clone())
                } else {
                    None
                }
            })
            .collect();
        for wf in self.workflows.values_mut() {
            for r in wf.rules.iter_mut() {
                if accept_early.contains(&r.result) {
                    r.result = "A".to_owned();
                } else if reject_early.contains(&r.result) {
                    r.result = "R".to_owned();
                }
            }
        }
    }
    fn find_ranges(rules: &[&Rule], cat: Category) -> Vec<Range> {
        let mut head = 0;
        let mut ranges = Vec::new();
        for r in rules.iter().filter(|f| f.category == Some(cat)) {
            if head == r.limit {
                panic!("zero length limit");
            }
            if r.op == Some(Op::Lt) {
                ranges.push(Range::new(head, r.limit - 1));
                head = r.limit;
            } else if r.op == Some(Op::Gt) {
                ranges.push(Range::new(head, r.limit));
                head = r.limit + 1;
            }
        }
        if head < 4000 {
            ranges.push(Range::new(head, 4000));
        }
        ranges
    }
    fn find_limits(&self) -> Limits {
        let mut l = Limits::new();
        let mut all_rules = Vec::new();
        for wf in &self.workflows {
            for rule in wf.1.rules.iter() {
                if let Some(_) = rule.category {
                    all_rules.push(rule);
                }
            }
        }
        all_rules.sort_unstable_by_key(|f| f.limit);
        l.x = WorkSet::find_ranges(&all_rules, Category::X);
        l.m = WorkSet::find_ranges(&all_rules, Category::M);
        l.a = WorkSet::find_ranges(&all_rules, Category::A);
        l.s = WorkSet::find_ranges(&all_rules, Category::S);
        l
    }
    fn crunch(&self) -> u64 {
        let mut count: u64 = 0;
        let limits = self.find_limits();
        for x_range in &limits.x {
            let dx = x_range.delta();
            for m_range in &limits.m {
                let dm = m_range.delta();
                for a_range in &limits.a {
                    let da = a_range.delta();
                    for s_range in &limits.s {
                        let ds = s_range.delta();
                        let rating = Rating {
                            x: x_range.from,
                            m: m_range.from,
                            a: a_range.from,
                            s: s_range.from,
                        };
                        let is_accepted = self.run(&rating);
                        if is_accepted {
                            count += dx * dm * da * ds;
                        }
                    }
                }
            }
        }
        count
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/19.txt").unwrap();
    let mut w = WorkSet::new(&data);

    println!("sum: {}", w.summarize());

    println!("start: {}", w);
    w.minimize();
    println!("m once: {}", w);
    w.minimize();
    println!("m twice: {}", w);
    w.minimize();
    println!("m thrice: {}", w);

    //   let c = w.crunch(); // 40000 hours with first attempt minimize
    //println!("crunch: {}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_run_workflow() {
        let r = Rating::new("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!("qqz", Workflow::new("in{s<1351:px,qqz}").run(&r));
        assert_eq!("qs", Workflow::new("qqz{s>2770:qs,m<1801:hdj,R}").run(&r));
        assert_eq!("lnx", Workflow::new("qs{s>3448:A,lnx}").run(&r));
        assert_eq!("A", Workflow::new("lnx{m>1548:A,A}").run(&r));
    }
    #[test]
    fn test_make_rule() {
        let w = Rule::new("x>10:one");
        assert_eq!(Some(Category::X), w.category);
        assert_eq!(Some(Op::Gt), w.op);
        assert_eq!(10, w.limit);
        assert_eq!("one", w.result);
    }
    #[test]
    fn test_make_rule2() {
        let w = Rule::new("A");
        assert_eq!(None, w.category);
        assert_eq!(None, w.op);
        assert_eq!("A", w.result);
    }
    #[test]
    fn test_make_rating() {
        let r = Rating::new("{x=1579,m=399,a=19,s=226}");
        assert_eq!(1579, r.x);
        assert_eq!(399, r.m);
        assert_eq!(19, r.a);
        assert_eq!(226, r.s);
    }

    #[test]
    fn test_make_workflow() {
        let w = Workflow::new("ex{x>10:one,m<20:two,a>30:R,A}");
        assert_eq!("ex", w.name);
        assert_eq!(4, w.rules.len());
    }
    #[test]
    fn test_can_sum_input() {
        let w = WorkSet::new(TEST_DATA);
        assert_eq!(19114, w.summarize());
    }
    #[test]
    fn test_can_parse_input() {
        let w = WorkSet::new(TEST_DATA);
        assert_eq!(11, w.workflows.len());
        assert_eq!(5, w.ratings.len());
        assert_eq!(true, w.run(&w.ratings[0]));
        assert_eq!(false, w.run(&w.ratings[1]));
        assert_eq!(true, w.run(&w.ratings[2]));
        assert_eq!(false, w.run(&w.ratings[3]));
        assert_eq!(true, w.run(&w.ratings[4]));
    }
    #[test]
    fn test_can_count_all_range() {
        let w = WorkSet::new(TEST_DATA);
        let count = w.crunch();
        assert_eq!(167409079868000, count);
    }
    #[test]
    fn test_can_collect_rule_limits() {
        let w = WorkSet::new(TEST_DATA);
        let l = w.find_limits();
        assert_eq!(
            vec![
                Range::new(0, 1415),
                Range::new(1416, 2440),
                Range::new(2441, 2662),
                Range::new(2663, 4000),
            ],
            l.x
        );
        assert_eq!(
            vec![
                Range::new(0, 838),
                Range::new(839, 1548),
                Range::new(1549, 1800),
                Range::new(1801, 2090),
                Range::new(2091, 4000),
            ],
            l.m
        )
    }
    static TEST_DATA: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
}
