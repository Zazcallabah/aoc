use std::collections::HashMap;

struct Node {
    start: usize,
    length: usize,
    children: Vec<Node>,
    metadata: Vec<u32>,
    sum: Option<u32>,
}

impl Node {
    fn new(data: &str) -> Node {
        let input: Vec<u32> = data.split(' ').map(|d| d.parse::<u32>().unwrap()).collect();
        Node::from(&input, 0)
    }
    fn from(data: &[u32], current: usize) -> Node {
        let mut pointer = current;
        if let Some(c_count) = data.get(current) {
            if let Some(m_count) = data.get(current + 1) {
                let mut node = Node {
                    start: pointer,
                    length: 0,
                    children: Vec::new(),
                    metadata: Vec::new(),
                    sum: None,
                };
                pointer += 2;
                for c in 0..(*c_count as usize) {
                    let r = Node::from(&data, pointer);
                    pointer += r.length;
                    node.children.push(r);
                }
                for m in 0..(*m_count as usize) {
                    if let Some(mm) = data.get(pointer) {
                        node.metadata.push(*mm);
                        pointer += 1;
                    } else {
                        panic!("invalid m range {}", pointer);
                    }
                }
                node.length = pointer - current;
                return node;
            } else {
                panic!("invalid node range 1 {}", current);
            }
        } else {
            panic!("invalid node range 0: {}", current)
        }
    }
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for m in &self.metadata {
            sum += m;
        }
        for c in &self.children {
            sum += c.sum();
        }
        sum
    }
    fn get_empty_sum(&mut self) -> u32 {
        let s = self.sum();
        self.sum = Some(s);
        s
    }
    fn part2(&mut self) -> u32 {
        if let Some(s) = self.sum {
            return s;
        }

        if self.children.is_empty() {
            return self.get_empty_sum();
        }
        let mut sum = 0;

        for &m in &self.metadata {
            if m > 0 {
                if let Some(c) = self.children.get_mut((m - 1) as usize) {
                    sum += c.part2();
                }
            }
        }
        self.sum = Some(sum);
        return sum;
    }
}

struct Nodes {
    root: u32,
    nodes: HashMap<u32, Node>,
}
impl Nodes {
    // fn new(data: &str) -> Nodes {
    //     let mut ix = 1u32;
    //     let input: Vec<u32> = data.split(' ').map(|d| d.parse()).collect();
    //     let mut start = 0usize;
    //     let mut v = Node::from(&input, start);
    //  }
}

fn main() {
    let data = std::fs::read_to_string("2018/8.txt").unwrap();
    let mut basenode = Node::new(&data);
    println!("metadata: {}", &basenode.sum());
    println!("part2: {}", &basenode.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_nodes() {
        let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        //          A----------------------------------
        //              B----------- C-----------
        //                               D-----

        // A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
        // B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
        // C, which has 1 child node (D) and 1 metadata entry (2).
        // D, which has 0 child nodes and 1 metadata entry (99).

        let mut basenode = Node::new(&data);
        assert_eq!(0, basenode.start);
        assert_eq!(16, basenode.length);
        assert_eq!(3, basenode.metadata.len());
        assert_eq!(138, basenode.sum());
        assert_eq!(66, basenode.part2());
    }
}
