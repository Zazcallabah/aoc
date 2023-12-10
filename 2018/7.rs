use std::collections::{HashMap, VecDeque};

struct Dependency {
    id: char,
    dependees: Vec<char>,
    dependencies: Vec<char>,
}
impl Dependency {
    fn new(id: char) -> Dependency {
        Dependency {
            id,
            dependees: Vec::new(),
            dependencies: Vec::new(),
        }
    }
    fn only_depends_on(&self, ids: &[char]) -> bool {
        for dep in self.dependencies.iter() {
            if !ids.iter().any(|d| d == dep) {
                return false;
            }
        }
        return true;
    }
}
struct Dependencies {
    map: HashMap<char, Dependency>,
}
impl Dependencies {
    fn new(data: &str) -> Dependencies {
        let mut map: HashMap<char, Dependency> = HashMap::new();
        for line in data.lines() {
            let mut chars = line.chars();
            let beforeid = chars.nth(5).unwrap();
            let afterid = chars.nth(30).unwrap();

            let before = map
                .entry(beforeid)
                .or_insert_with_key(|k| Dependency::new(*k));
            before.dependees.push(afterid);
            let after = map
                .entry(afterid)
                .or_insert_with_key(|k| Dependency::new(*k));
            after.dependencies.push(beforeid);
        }
        Dependencies { map }
    }
    fn only_depends_on(&self, ids: &[char]) -> Vec<char> {
        let mut v = Vec::new();
        for dep in self.map.values() {
            if !ids.iter().any(|&d| d == dep.id) && dep.only_depends_on(ids) {
                v.push(dep.id);
            }
        }
        return v;
    }
}
fn crunch(d: &Dependencies) -> String {
    let mut to_add = VecDeque::new();
    let mut collected = Vec::new();
    loop {
        let once = d.only_depends_on(&collected);
        for c in once {
            if !to_add.iter().any(|&d| d == c) {
                to_add.push_back(c);
            }
        }
        if to_add.len() == 0 {
            break;
        }
        to_add.make_contiguous().sort();
        collected.push(to_add.pop_front().unwrap());
    }
    return collected.iter().collect();
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Task {
    id: char,
    seconds: u32,
    started_at: u32,
    will_be_done_at: u32,
}
impl Task {
    fn new(id: char, task_time: u32, started_at: u32) -> Task {
        let seconds: u32 = (id as u32 - '@' as u32) + task_time;
        Task {
            id,
            seconds,
            started_at,
            will_be_done_at: seconds + started_at,
        }
    }
    fn is_done(&self, time: u32) -> bool {
        time >= self.will_be_done_at
    }
}
fn crunch2(d: &Dependencies, worker_count: u8, task_time: u32) -> u32 {
    let mut time = 0u32;
    let mut done: Vec<char> = Vec::new();
    let mut busy_worker_count = 0;
    let mut in_progress: Vec<Task> = Vec::new();
    loop {
        // queue if available
        let mut available: Vec<char> = d
            .only_depends_on(&done)
            .iter()
            .filter_map(|c| {
                if !in_progress.iter().any(|t| t.id == *c) && !done.iter().any(|t| t == c) {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect();

        if available.len() == 0 && in_progress.len() == 0 {
            break;
        }
        available.sort();
        for next in available {
            if busy_worker_count >= worker_count {
                break;
            }
            println!("starting task {} at {}", next, time);
            in_progress.push(Task::new(next, task_time, time));
            busy_worker_count += 1;
        }

        // progress time
        let mut min = u32::MAX;
        for task in &in_progress {
            if task.will_be_done_at < min {
                min = task.will_be_done_at;
            }
        }
        time = min;

        // finished tasks
        in_progress.retain(|t| {
            if !t.is_done(time) {
                true
            } else {
                println!("task {} complete at {}", t.id, time);
                busy_worker_count -= 1;
                done.push(t.id);
                false
            }
        });
    }

    time
}
fn main() {
    let data = std::fs::read_to_string("2018/7.txt").unwrap();
    let d = Dependencies::new(&data);
    let result = crunch(&d);
    println!("order: {}", &result);
    let seconds = crunch2(&d, 6, 60);
    println!("seconds: {}", &seconds); // 1683 is too high
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    #[test]
    fn test_can_crunch2() {
        let d = Dependencies::new(TEST_DATA);
        let result = crunch2(&d, 2, 0);
        assert_eq!(15, result);
    }
    #[test]
    fn test_can_make_deps() {
        let d = Dependencies::new(TEST_DATA);
        assert_eq!('A', d.map.get(&'A').unwrap().id);
        assert_eq!('B', d.map.get(&'B').unwrap().id);
        assert_eq!('C', d.map.get(&'C').unwrap().id);
        assert_eq!('D', d.map.get(&'D').unwrap().id);
        assert_eq!('E', d.map.get(&'E').unwrap().id);
        assert_eq!('F', d.map.get(&'F').unwrap().id);
    }

    #[test]
    fn test_can_find_start() {
        let d = Dependencies::new(TEST_DATA);
        let start = d.only_depends_on(&Vec::new());
        assert_eq!(vec!['C'], start);
    }
    #[test]
    fn test_can_iter_once() {
        let d = Dependencies::new(TEST_DATA);

        assert_eq!("CABDFE", crunch(&d))
    }
    #[test]
    fn test_task_end_calculation() {
        assert_eq!(1, Task::new('A', 0, 0).will_be_done_at);
        assert_eq!(73, Task::new('C', 60, 10).will_be_done_at);
    }
    #[test]
    fn test_task_is_done() {
        assert_eq!(true, Task::new('B', 0, 4).is_done(6));
    }
}
