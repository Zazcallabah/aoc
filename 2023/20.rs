use std::collections::HashMap;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Signal {
    High,
    Low,
}

struct Transmission {
    from: String,
    to: String,
    signal: Signal,
}
impl Transmission {
    fn new(from: String, to: String, signal: Signal) -> Transmission {
        Transmission { from, to, signal }
    }
}

struct Module {
    name: String,
    typ: ModuleType,
    inputs: HashMap<String, Signal>,
    outputs: Vec<String>,
    high_signals_sent: u64,
    low_signals_sent: u64,
    flop_state_on: bool,
    conj_high_count: usize,
    stats: Vec<u64>,
}
lazy_static! {
    static ref PARSE: Regex = Regex::new(r"([^ ]+) -> (.+)").unwrap();
    static ref IMPORTANT: Vec<String> = vec!["rd", "bt", "fv", "pr"]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
}

impl Module {
    fn each(&mut self, signal: Signal) -> Vec<Transmission> {
        if signal == Signal::Low {
            self.low_signals_sent += self.outputs.len() as u64;
        } else {
            self.high_signals_sent += self.outputs.len() as u64;
        }
        let v: Vec<Transmission> = self
            .outputs
            .iter()
            .map(|o| Transmission::new(self.name.to_owned(), o.to_owned(), signal))
            .collect();
        v
    }
    fn handle(&mut self, t: &Transmission, button_count: u64) -> Vec<Transmission> {
        match self.typ {
            ModuleType::Broadcast => self.each(t.signal),
            ModuleType::Conjunction => {
                let entry = self.inputs.get_mut(&t.from).unwrap();
                if *entry != t.signal {
                    if t.signal == Signal::Low {
                        self.conj_high_count -= 1;
                    } else {
                        self.conj_high_count += 1;
                    }
                    *entry = t.signal;
                }
                let send_low = self.inputs.len() == self.conj_high_count;
                if !send_low && IMPORTANT.contains(&self.name) {
                    self.stats.push(button_count);
                }
                self.each(if send_low { Signal::Low } else { Signal::High })
            }
            ModuleType::FlipFlop => match t.signal {
                Signal::High => vec![],
                Signal::Low => {
                    self.flop_state_on = !self.flop_state_on;
                    self.each(if self.flop_state_on {
                        Signal::High
                    } else {
                        Signal::Low
                    })
                }
            },
        }
    }
    fn add_input(&mut self, source: String) {
        if !self.inputs.contains_key(&source) {
            self.inputs.insert(source, Signal::Low);
        }
    }
    fn name_and_type(n: &str) -> (String, ModuleType) {
        let mt = match n.chars().next().unwrap() {
            '&' => ModuleType::Conjunction,
            '%' => ModuleType::FlipFlop,
            _ => ModuleType::Broadcast,
        };
        let name = match mt {
            ModuleType::Broadcast => n.to_owned(),
            _ => n[1..].to_owned(),
        };
        (name, mt)
    }
    fn new(data: &str) -> Module {
        let cap = PARSE.captures(data).unwrap();
        let (name, typ) = Module::name_and_type(&cap.get(1).unwrap().as_str());
        let inputs = HashMap::new();
        let outputs: Vec<String> = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|f| f.to_owned())
            .collect();

        Module {
            name,
            typ,
            inputs,
            outputs,
            flop_state_on: false,
            conj_high_count: 0,
            high_signals_sent: 0,
            low_signals_sent: 0,
            stats: Vec::new(),
        }
    }
}

struct ModuleSet {
    modules: HashMap<String, Module>,
    button_press_count: u64,
    rx_sent: bool,
}
impl ModuleSet {
    fn sum(&self) -> (u64, u64) {
        let mut highs = 0;
        let mut lows = self.button_press_count;
        for m in &self.modules {
            highs += m.1.high_signals_sent;
            lows += m.1.low_signals_sent;
        }
        (highs, lows)
    }
    fn run(&mut self) {
        let mut next = vec![Transmission::new(
            "".to_owned(),
            "broadcaster".to_owned(),
            Signal::Low,
        )];
        self.button_press_count += 1;

        while next.len() > 0 {
            next = self.run_transmissions(&next, self.button_press_count);
        }
    }
    fn run_transmissions(
        &mut self,
        transmissions: &[Transmission],
        button_count: u64,
    ) -> Vec<Transmission> {
        let mut nexts = Vec::new();
        for t in transmissions {
            if let Some(to) = self.modules.get_mut(&t.to) {
                let mut nn = to.handle(t, button_count);
                nexts.append(&mut nn)
            } else {
                if t.to == "rx" && t.signal == Signal::Low {
                    self.rx_sent = true;
                }
            }
        }
        nexts
    }
    fn new(data: &str) -> ModuleSet {
        let mut modules = HashMap::new();
        for line in data.lines() {
            let m = Module::new(line);
            modules.insert(m.name.clone(), m);
        }
        let mut connections: Vec<Transmission> = Vec::new();
        for m in modules.keys() {
            let outputs = &modules.get(m).unwrap().outputs;
            for o in outputs {
                connections.push(Transmission::new(m.clone(), o.clone(), Signal::Low))
            }
        }
        for c in connections {
            if let Some(to) = modules.get_mut(&c.to) {
                to.add_input(c.from);
            }
        }
        ModuleSet {
            modules,
            button_press_count: 0,
            rx_sent: false,
        }
    }
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
    let data = std::fs::read_to_string("2023/20.txt").unwrap();
    let mut r = ModuleSet::new(&data);

    for _ in 1..=1000 {
        r.run();
    }
    let (highs, lows) = r.sum();
    println!("highs:{} lows:{} product: {}", highs, lows, highs * lows);

    let mut r = ModuleSet::new(&data);
    for _ in 0..50000 {
        r.run();
        if r.rx_sent {
            break;
        }
    }
    for imp in IMPORTANT.iter() {
        if let Some(module) = r.modules.get(imp) {
            println!("stats for {}", imp);
            println!("hits: {:?}", module.stats);
            let mut d0 = 0;
            let mut deltas: Vec<u64> = vec![];
            for st in &module.stats {
                deltas.push(st - d0);
                d0 = *st;
            }
            println!("deltas: {:?}", deltas);
        }
    }
    //  stats for rd
    // hits: [3911, 7822, 11733, 15644, 19555, 23466, 27377, 31288, 35199, 39110, 43021, 46932]
    // deltas: [3911, 3911, 3911, 3911, 3911, 3911, 3911, 3911, 3911, 3911, 3911, 3911]
    // stats for bt
    // hits: [3917, 7834, 11751, 15668, 19585, 23502, 27419, 31336, 35253, 39170, 43087, 47004]
    // deltas: [3917, 3917, 3917, 3917, 3917, 3917, 3917, 3917, 3917, 3917, 3917, 3917]
    // stats for fv
    // hits: [3929, 7858, 11787, 15716, 19645, 23574, 27503, 31432, 35361, 39290, 43219, 47148]
    // deltas: [3929, 3929, 3929, 3929, 3929, 3929, 3929, 3929, 3929, 3929, 3929, 3929]
    // stats for pr
    // hits: [3793, 7586, 11379, 15172, 18965, 22758, 26551, 30344, 34137, 37930, 41723, 45516, 49309]
    // deltas: [3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793, 3793]

    println!("gcd: {}", lcm_many(&vec![3911, 3917, 3929, 3793]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_count_signals_2() {
        let mut r = ModuleSet::new(
            r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        for _ in 1..=1000 {
            r.run();
        }
        let (highs, lows) = r.sum();
        assert_eq!(2750, highs);
        assert_eq!(4250, lows);
    }

    #[test]
    fn test_can_count_signals() {
        let mut r = ModuleSet::new(
            r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        r.run();
        let (highs, lows) = r.sum();
        assert_eq!(4, highs);
        assert_eq!(8, lows);
    }

    #[test]
    fn test_can_create_modules() {
        let r = ModuleSet::new(
            r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(5, r.modules.len());
    }

    static TEST_DATA: &str = r"";
}
