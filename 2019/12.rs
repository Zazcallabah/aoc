use regex::Regex;
use std::{collections::HashMap, fs, time::SystemTime};
use primal::Sieve;
#[cfg(test)]
mod tests {

    use regex::Captures;

    use super::*;
    #[test]
    fn test_move() {
        let mut europa = Moon::new(MoonLabel::Europa, 1, 2, 3);
        europa.vel = (-2, 0, 3);
        europa.move_step();
        assert_eq!((-1, 2, 6), europa.pos);
    }
    #[test]
    fn test_apply_grav() {
        let mut v = vec![
            Moon::new(MoonLabel::Io, 4, 3, 15),
            Moon::new(MoonLabel::Ganymede, -11, 10, 13),
        ];
        let (a, b) = v.split_at_mut(1);
        a[0].apply_grav(&mut b[0]);
        assert_eq!((-1, 1, -1), v[0].vel);
        assert_eq!((1, -1, 1), v[1].vel);
    }
    #[test]
    fn test_apply_tick() {
        let mut v = vec![
            Moon::new(MoonLabel::Io, 4, 3, 15),
            Moon::new(MoonLabel::Ganymede, -11, 10, 13),
        ];
        let (a, b) = v.split_at_mut(1);
        a[0].apply_grav(&mut b[0]);
        v[0].move_step();
        v[1].move_step();

        assert_eq!((3, 4, 14), v[0].pos);
        assert_eq!((-10, 9, 14), v[1].pos);
    }

    #[test]
    fn test_iterate_many() {
        let mut moons = test_moons();
        simulate_step(&mut moons);
        assert_eq!((2, -1, 1), moons[0].pos);
        assert_eq!((3, -1, -1), moons[0].vel);
    }

    #[test]
    fn test_bulk() {
        let mut moons = test_moons();
		for _ in 0..10000000{
                simulate_step(&mut moons);
        }
    }
    #[test]
    fn test_iterations() {
        let mut moons = test_moons();
        let testdata: &String = &std::fs::read_to_string("2019/12a.txt").unwrap();
        let parser = Regex::new(r"pos=<x= *([0-9-]+), y= *([0-9-]+), z= *([0-9-]+)>, vel=<x= *([0-9-]+), y= *([0-9-]+), z= *([0-9-]+)>").unwrap();
        let testcaps = parser.captures_iter(&testdata);
        let capvec: Vec<Captures> = testcaps.into_iter().collect();

        for (step, chunk) in capvec.chunks(4).enumerate() {
            for (ix, cap) in chunk.into_iter().enumerate() {
                assert_eq!(cap[1].parse::<i32>().unwrap(), moons[ix].pos.0);
                assert_eq!(cap[2].parse::<i32>().unwrap(), moons[ix].pos.1);
                assert_eq!(cap[3].parse::<i32>().unwrap(), moons[ix].pos.2);
                assert_eq!(cap[4].parse::<i32>().unwrap(), moons[ix].vel.0);
                assert_eq!(cap[5].parse::<i32>().unwrap(), moons[ix].vel.1);
                assert_eq!(cap[6].parse::<i32>().unwrap(), moons[ix].vel.2);
                print!(".");
            }
            if step < 10 {
                simulate_step(&mut moons);
            }
        }
        let energy: i32 = moons.iter().map(|m| m.total_energy()).sum();
        assert_eq!(179, energy);
    }

    #[test]
    fn test_more_iterations() {
        let mut moons = test_moons2();
        let testdata: &String = &std::fs::read_to_string("2019/12b.txt").unwrap();
        let parser = Regex::new(r"pos=<x= *([0-9-]+), y= *([0-9-]+), z= *([0-9-]+)>, vel=<x= *([0-9-]+), y= *([0-9-]+), z= *([0-9-]+)>").unwrap();
        let testcaps = parser.captures_iter(&testdata);
        let capvec: Vec<Captures> = testcaps.into_iter().collect();

        for (step, chunk) in capvec.chunks(4).enumerate() {
            for (ix, cap) in chunk.into_iter().enumerate() {
                assert_eq!(cap[1].parse::<i32>().unwrap(), moons[ix].pos.0);
                assert_eq!(cap[2].parse::<i32>().unwrap(), moons[ix].pos.1);
                assert_eq!(cap[3].parse::<i32>().unwrap(), moons[ix].pos.2);
                assert_eq!(cap[4].parse::<i32>().unwrap(), moons[ix].vel.0);
                assert_eq!(cap[5].parse::<i32>().unwrap(), moons[ix].vel.1);
                assert_eq!(cap[6].parse::<i32>().unwrap(), moons[ix].vel.2);
                print!(".");
            }
            if step < 10 {
                for _ in 0..10 {
                    simulate_step(&mut moons);
                }
            }
        }
        let energy: i32 = moons.iter().map(|m| m.total_energy()).sum();

		assert_eq!(1940, energy)
    }

    #[test]
    fn test_getp_1d(){

        let moons = test_moons();
        let periods : Vec<u128> = vec![1i8,2,3]
            .into_iter()
            .map(|dim| get_period(&moons, dim))
            .map(|u| u as u128)
            .collect();
        assert_eq!(18,periods[0]);
        assert_eq!(28,periods[1]);
        assert_eq!(44,periods[2]);
    }

    #[test]
    fn test_getp_1d_factorize(){

        let moons = test_moons2();
        let periods : Vec<u64> = vec![1i8,2,3]
            .into_iter()
            .map(|dim| get_period(&moons, dim))
            .collect();

        assert_eq!(4686774924u128,factorize(&periods));
    }

    fn test_moons() -> Vec<Moon> {
        vec![
            Moon::new(MoonLabel::Io, -1, 0, 2),
            Moon::new(MoonLabel::Ganymede, 2, -10, -7),
            Moon::new(MoonLabel::Callisto, 4, -8, 8),
            Moon::new(MoonLabel::Europa, 3, 5, -1),
        ]
    }
    fn test_moons2() -> Vec<Moon> {
        vec![
            Moon::new(MoonLabel::Io, -8, -10, 0),
            Moon::new(MoonLabel::Ganymede, 5, 5, 10),
            Moon::new(MoonLabel::Callisto, 2, -7, 3),
            Moon::new(MoonLabel::Europa, 9, -8, -3),
        ]
    }
}

fn factorize(periods: &[u64]) -> u128 {

    let sieve = Sieve::new(100000);

    let result : Vec<(usize,usize)> = periods.iter()
        .map(|p| sieve.factor(*p as usize).unwrap())
        .flatten()
        .collect();

    let mut factorcounts = HashMap::new();
    for (fac,n) in result {
        let mut candidate = factorcounts.entry(fac).or_default();
        *candidate = n.max(*candidate);
    }

    let finaln = factorcounts.drain()
        .map(|(a,b)| a.pow(b as u32))
        .reduce(|a,b| a*b)
        .unwrap();
    finaln as u128
    }

fn simulate_step1d(moons: &mut [Moon1D]) {
    for pivot in 1..moons.len() {
        let (head, tail) = moons.split_at_mut(pivot);
        let last = head.last_mut().unwrap();
        for m in tail {
            last.apply_grav(m);
        }
    }

    for moon in moons.iter_mut() {
        moon.move_step();
    }
}

fn simulate_step(moons: &mut [Moon]) {
    for pivot in 1..moons.len() {
        let (head, tail) = moons.split_at_mut(pivot);
        let last = head.last_mut().unwrap();
        for m in tail {
            last.apply_grav(m);
        }
    }

    for moon in moons.iter_mut() {
        moon.move_step();
    }
}
enum MoonLabel {
    Io,
    Ganymede,
    Europa,
    Callisto,
}

type Vec3 = (i32, i32, i32);


#[derive(PartialEq, Eq)]
struct Moon1D {
    pos: i32,
    vel: i32,
}

impl Moon1D {
    fn new( x: i32) -> Moon1D {
        Moon1D {
            pos: x,
            vel: 0,
        }
    }
    fn move_step(&mut self) {
        self.pos += self.vel;
    }
    fn apply_grav(&mut self, other: &mut Moon1D) {
        if self.pos > other.pos {
            self.vel -= 1;
            other.vel += 1;
        } else if self.pos < other.pos {
            self.vel += 1;
            other.vel -= 1;
        }
    }
}

#[derive(PartialEq, Eq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}



impl Moon {
    fn new(name: MoonLabel, x: i32, y: i32, z: i32) -> Moon {
        Moon {
            pos: (x, y, z),
            vel: (0, 0, 0),
        }
    }
    fn potential(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }
    fn kinetic(&self) -> i32 {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }
    fn total_energy(&self) -> i32 {
        self.potential() * self.kinetic()
    }
    fn get_1d(&self,d:i8) -> Moon1D {
        match d {
            1 => Moon1D::new(self.pos.0),
            2 => Moon1D::new(self.pos.1),
            3 => Moon1D::new(self.pos.2),
            _ => panic!("invalid dimonsion"),
        }
    }

    fn move_step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }
    fn apply_grav(&mut self, other: &mut Moon) {
        if self.pos.0 > other.pos.0 {
            self.vel.0 -= 1;
            other.vel.0 += 1;
        } else if self.pos.0 < other.pos.0 {
            self.vel.0 += 1;
            other.vel.0 -= 1;
        }
        if self.pos.1 > other.pos.1 {
            self.vel.1 -= 1;
            other.vel.1 += 1;
        } else if self.pos.1 < other.pos.1 {
            self.vel.1 += 1;
            other.vel.1 -= 1;
        }
        if self.pos.2 > other.pos.2 {
            self.vel.2 -= 1;
            other.vel.2 += 1;
        } else if self.pos.2 < other.pos.2 {
            self.vel.2 += 1;
            other.vel.2 -= 1;
        }
    }
}
fn get_moons() -> Vec<Moon> {
    vec![
        Moon::new(MoonLabel::Io, -4, 3, 15),
        Moon::new(MoonLabel::Ganymede, -11, -10, 13),
        Moon::new(MoonLabel::Callisto, 2, 2, 18),
        Moon::new(MoonLabel::Europa, 7, -1, 0),
    ]
}

fn equals1d(m1:&[Moon1D], m2:&[Moon1D]) -> bool {
	return m1[0] == m2[0] &&
	m1[1] == m2[1] &&
	m1[2] == m2[2] &&
	m1[3] == m2[3]
}

fn equals(m1:&[Moon], m2:&[Moon]) -> bool {
	return m1[0] == m2[0] &&
	m1[1] == m2[1] &&
	m1[2] == m2[2] &&
	m1[3] == m2[3]
}

fn get_period( fullmoons: &[Moon], dim:i8) -> u64 {
    let initstate: Vec<Moon1D> = fullmoons.iter().map(|m| m.get_1d(dim)).collect();
    let mut moons: Vec<Moon1D> = fullmoons.iter().map(|m| m.get_1d(dim)).collect();
    simulate_step1d(&mut moons);

	for n in 1.. {
		if equals1d( &initstate, &moons ) {
			return n;
		}
        simulate_step1d(&mut moons);
	}
    return 0;
}

fn main() {

	let now = SystemTime::now();
    let mut moons = get_moons();
    for n in 0..1000 {
        simulate_step(&mut moons);
    }
    let energy: i32 = moons.iter().map(|m| m.total_energy()).sum();
    println!("after 1000: {}", energy);

    let moons = get_moons();
    let periods : Vec<u64> = vec![1i8,2,3]
        .into_iter()
        .map(|dim| get_period(&moons, dim))
        .collect();

    println!("periods: {:?}",periods);
    let fact = factorize(&periods);
    println!("total: {}",fact);
	if let Ok(t) = now.elapsed() {
		println!("took {}", t.as_millis());
	}

}
