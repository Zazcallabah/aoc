#[macro_use]
extern crate lazy_static;

struct Lens {
    id: String,
    focal: u8,
}
struct LensBox {
    id: u8,
    lenses: Vec<Lens>,
}
impl LensBox {
    fn new(id: u8) -> LensBox {
        LensBox {
            id,
            lenses: Vec::new(),
        }
    }
    fn add(&mut self, label: &str, focal: u8) {
        let lens = Lens {
            id: label.to_owned(),
            focal,
        };
        if let Some(l) = self.lenses.iter().position(|p| p.id == label) {
            self.lenses[l] = lens;
        } else {
            self.lenses.push(lens);
        }
    }
    fn remove(&mut self, label: &str) {
        if let Some(l) = self.lenses.iter().position(|p| p.id == label) {
            self.lenses.remove(l);
        }
    }
    fn get_power(&self) -> u32 {
        let idp = self.id as u32 + 1;
        let mut sum = 0;
        for (ix, lens) in self.lenses.iter().enumerate() {
            sum += idp * (ix as u32 + 1) * lens.focal as u32;
        }
        sum
    }
}
struct Op {
    hash: u8,
    label: String,
    op: char,
    focal: Option<u8>,
}
lazy_static! {
    static ref OP_PARSE: regex::Regex = regex::Regex::new(r"^(\w+)([-=])(\w*)$").unwrap();
}

impl Op {
    fn hash(data: &str) -> u8 {
        let mut value = 0u32;
        for b in data.bytes() {
            value += b as u32;
            value *= 17;
            value = value % 256;
        }
        return value as u8;
    }
    fn new_many(data: &str) -> Vec<Op> {
        let mut v = Vec::new();
        for d in data.split(',') {
            v.push(Op::new(d));
        }
        v
    }
    fn new(data: &str) -> Op {
        match OP_PARSE.captures(data) {
            None => panic!("invalid input"),
            Some(c) => {
                let label = c.get(1).unwrap().as_str().to_string();
                let hash = Op::hash(&label);
                let op = c.get(2).unwrap().as_str().as_bytes()[0] as char;
                let focal: Option<u8> = match c.get(3) {
                    Some(x) => {
                        let s = x.as_str();
                        if s.len() == 0 {
                            None
                        } else {
                            Some(s.as_bytes()[0] - 48)
                        }
                    }
                    None => None,
                };
                return Op {
                    hash,
                    op,
                    label,
                    focal,
                };
            }
        }
    }
}
struct Boxes {
    boxes: Vec<LensBox>,
}
impl Boxes {
    fn new() -> Boxes {
        let boxes = (0..=255).into_iter().map(|id| LensBox::new(id)).collect();
        Boxes { boxes }
    }
    fn run(&mut self, op: Op) {
        let lb = self.boxes.get_mut(op.hash as usize).unwrap();
        if op.op == '=' {
            lb.add(&op.label, op.focal.unwrap())
        } else {
            lb.remove(&op.label)
        }
    }
    fn power(&self) -> u32 {
        let mut sum = 0;
        for b in &self.boxes {
            sum += b.get_power();
        }
        sum
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/15.txt").unwrap();
    let s = sum(&data);
    println!("hash: {}", s);
    let mut boxes = Boxes::new();
    let ops = Op::new_many(&data);
    for op in ops {
        boxes.run(op);
    }
    let power = boxes.power();
    println!("power: {}", power);
}

fn sum(data: &str) -> u32 {
    let mut sum = 0u32;
    for x in data.split(',') {
        sum += Op::hash(x) as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_new() {
        let o = Op::new("rn=1");
        assert_eq!("rn", o.label);
        assert_eq!(Some(1), o.focal);
        assert_eq!('=', o.op);
        assert_eq!(0, o.hash);
        let o = Op::new("cm-");
        assert_eq!("cm", o.label);
        assert_eq!(None, o.focal);
        assert_eq!('-', o.op);
        assert_eq!(0, o.hash);
    }

    #[test]
    fn test_hash_sum() {
        let s = sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(1320, s);
    }
    #[test]
    fn test_lens_array() {
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let mut boxes = Boxes::new();
        let ops = Op::new_many(data);
        for op in ops {
            boxes.run(op);
        }
        let power = boxes.power();
        assert_eq!(145, power);
    }
}
