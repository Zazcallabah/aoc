use std::f64::consts::PI;
use std::fmt;
use std::collections::{HashMap, HashSet};

type Coord = (isize,isize);

type Marks = Vec<Vec<bool>>;

struct Map {
	rows: Vec<Row>
}

struct Row {
	quads: Vec<Quad>
}

#[derive(Copy,Clone)]
struct Quad {
	is_asteroid: bool,
	score: u16
}

#[derive(Copy,Clone)]
struct AsteroidInfo {
	absolute_coord: Coord,
	relative_coord: Coord,
	angle: i32,
	magnitude: i32,
}

impl Map{
	fn new(data:&str) -> Map {
		Map {
			rows:
			data
			.lines()
			.map(Row::new)
			.collect()
		}
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in &self.rows {
			let a = row.quads.iter().map(|q| if q.is_asteroid {b'0'+(q.score) as u8 } else { b'.'});
			let s =
				String::from_utf8(a.collect()).unwrap();
			if let Err(e) = writeln!(f, "{}", s) {
				return Err(e)
			}
		}
		Ok(())
	}
}

impl Row {
	fn new(row:&str) -> Row {
		Row {
			quads: row
			.bytes()
			.map(Quad::new)
			.collect()
		}
	}
}

impl Quad{
	fn new(a:u8) -> Quad {
		Quad {
			is_asteroid: a == b'#',
			score: 0
		}
	}
}

fn markermap(r:usize,c:usize)->Marks{
	vec![vec![false;c];r]
}
fn get_astr_ident(map: &Map) -> HashSet<Coord> {
	let mut set = HashSet::new();

	for (y,r) in map.rows.iter().enumerate() {
		for (x,_) in r.quads.iter().enumerate().filter(|q| q.1.is_asteroid ) {
			set.insert((x as isize,y as isize));
		}
	}
	set
}
fn stat_astr(data: &str) -> Map {

	let mut result = Map::new(&data);
	let is_astr = get_astr_ident(&result);  ////

	let rowcount = result.rows.len();
	let colcount = result.rows[0].quads.len();

	for (ru, row) in result.rows.iter_mut().enumerate() {
		let r = ru as isize;
		for (cu, col) in row.quads.iter_mut().enumerate().filter(|q|q.1.is_asteroid ) {
			let c = cu as isize;
			score_asteroids(col,r,c,rowcount,colcount,&is_astr);

		}
	}
	result
}

fn score_asteroids(col:&mut Quad,r_ix:isize,c_ix:isize,max_r:usize,max_c:usize,is_astr:&HashSet<Coord>) {

	let mut marks = markermap(max_r,max_c);
	for (x,y) in get_spiral(r_ix,c_ix,max_r as isize,max_c as isize){
		if !marks[y as usize][x as usize] && is_astr.contains(&(x,y)) {
			col.score += 1;
			let dx = x - c_ix;
			let dy = y - r_ix;
			mark_steps(&mut marks, x, y, dx, dy);
		}
	}
}

fn get_spiral(r:isize, c:isize, max_r:isize, max_c:isize) -> Vec<Coord> {
	let mut result = Vec::new();

	let mut d = 1isize;

	loop{
		let top = r - d;
		let bottom = r + d;
		let left = c - d;
		let right = c + d;

		let top_valid = top >= 0;
		let bottom_valid = bottom < max_r;
		let left_valid = left >= 0;
		let right_valid = right < max_c;

		for row in (top..=bottom).filter(|&fr| fr >= 0 && fr < max_r) {
			if left_valid {
				result.push((left,row));
			}
			if right_valid {
				result.push((right,row));
			}
		}

		for col in ((left+1)..(right)).filter(|&fc| fc >= 0 && fc < max_c) {
			if top_valid {
				result.push((col,top));
			}
			if bottom_valid {
				result.push((col,bottom));
			}
		}

		if !top_valid && !bottom_valid && !left_valid && !right_valid {
			break
		}
		d += 1;
	}

	result
}

fn mark_steps(map:&mut Marks,mut x:isize,mut y:isize,dx:isize,dy:isize){
	// find lowest common divisor for dx-dy
	let (lcdx,lcdy) = reduce(dx,dy);
	loop
	{
		map[y as usize][x as usize] = true;
		y += lcdy;
		x += lcdx;
		if y < 0 || x < 0 || y >= map.len() as isize || x >= map[0].len() as isize {
			return
		}
	}
}
// hardcoded primes means the reduce function is only defined for distances < 2500
const PRIMES : &[isize] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
fn reduce(dx:isize,dy:isize) -> (isize,isize) {
	if dx == 0 && dy == 0 {
		panic!("invalid step distance");
	}
	for prime in PRIMES {
		if dx % prime == 0 && dy % prime == 0 {
			return reduce(dx/prime,dy/prime)
		}
	}
	(dx,dy)
}


#[cfg(test)]
mod tests {
	use std::f64::consts::PI;

    use super::*;

	#[test]
	fn test_reduce(){
		//cardinal directions n steps
		for n in 1..5 {
			assert_eq!((1,0),reduce(n,0));
			assert_eq!((0,1),reduce(0,n));
			assert_eq!((0,-1),reduce(0,n*-1));
			assert_eq!((-1,0),reduce(n*-1,0));
		}

		//diagonals n steps
		for n in 1..5 {
			assert_eq!((1,1),reduce(n,n));
			assert_eq!((-1,-1),reduce(-n,-n));
			assert_eq!((-1,1),reduce(-n,n));
			assert_eq!((1,-1),reduce(n,-n));
		}


		// rationals
		assert_eq!((2,3),reduce(6,9));
		assert_eq!((2,3),reduce(2,3));

	}

	#[test]
	fn test_mark_steps(){
// .#...
// .....
// .#...
// .#...
// .#...
		let mut mx = markermap(5,5);
		mark_steps(&mut mx, 1, 2, 0, 2);
		let markd : usize  = mx.iter().map(|row| row.iter().filter(|&&quad|quad).collect::<Vec<&bool>>().len()).sum();
		assert_eq!(3,markd);
		assert!(mx[2][1]);
		assert!(mx[3][1]);
		assert!(mx[4][1]);
	}

	#[test]
	fn test_large_field() {
		let s =
r"
.#...
.....
.#...
.#...
.#...".trim();
		let m = Map::new(&s);
		let astr = get_astr_ident(&m);
		let mut col = m.rows[0].quads[1];

		score_asteroids(&mut col,0, 1,5,5,&astr);
		println!("{}",&m);
		assert_eq!(1,col.score);
	}

	#[test]
	fn test_small_field() {
		let s =
r".##
.#.
.#.";
		let m = Map::new(&s);
		let astr = get_astr_ident(&m);
		let mut col = m.rows[0].quads[1];

		score_asteroids(&mut col,0, 1,3,3,&astr);
		println!("{}",&m);
		assert_eq!(2,col.score);
	}

	#[test]
	fn test_spiraloutwards(){
		let r = get_spiral(0,0,2,2);

		assert!(r.contains(&(0,1)));
		assert!(r.contains(&(1,0)));
		assert!(r.contains(&(1,1)));
		assert_eq!(3,r.len());
	}

	#[test]
	fn test_spiraloutwards_big(){
		let r = get_spiral(1,1,4,4);
	//2223
	//2123
	//2223
	//3333
		assert_eq!(15,r.len());

		assert!(r[..8].contains(&(0,0)));
		assert!(r[..8].contains(&(0,1)));
		assert!(r[..8].contains(&(1,0)));
		assert!(r[..8].contains(&(0,2)));
		assert!(r[..8].contains(&(2,0)));
		assert!(r[..8].contains(&(2,1)));
		assert!(r[..8].contains(&(2,2)));
		assert!(r[..8].contains(&(1,2)));

		assert!(r[8..].contains(&(3,0)));
		assert!(r[8..].contains(&(3,1)));
		assert!(r[8..].contains(&(3,2)));
		assert!(r[8..].contains(&(3,3)));
		assert!(r[8..].contains(&(2,3)));
		assert!(r[8..].contains(&(1,3)));
		assert!(r[8..].contains(&(0,3)));
		assert!(!r.contains(&(1,1)));
	}


	#[test]
	fn test_can_mark_on_map(){
		let mut m = markermap(10,10);
		mark_steps(&mut m,3isize,2isize, 2isize, 1isize);
		let assertdata = r"..........
..........
...x......
.....x....
.......x..
.........x
..........";

		for l in assertdata.lines().enumerate(){
			for b in l.1.bytes().enumerate(){
				assert_eq!(m[l.0][b.0], (b.1 == b'x'));
			}
		}
	}

	#[test]
	fn test_get_astr_ident(){
		let m= Map::new(&"...#\n....\n###.\n#...");
		let a = get_astr_ident(&m);
		assert_eq!(5,a.len());
		assert!(a.contains(&(3,0)));
		assert!(a.contains(&(0,2)));
		assert!(a.contains(&(1,2)));
		assert!(a.contains(&(2,2)));
		assert!(a.contains(&(0,3)));

	}

	#[test]
	fn test_create() {
		let f = Map::new(
			r".#..#
.....
#####
....#
...##");
assert!(!f.rows[0].quads[0].is_asteroid);
assert!(f.rows[0].quads[1].is_asteroid);
	}
	#[test]
	fn test_getcoord() {
		let f = Map::new(
r".#..#
.....
#####
....#
...##");
		let c = get_coords(&f);
		assert_eq!(c[0].0,1isize);
		assert_eq!(c[0].1,0isize);

		assert_eq!(c[6].0,4isize);
		assert_eq!(c[6].1,2isize);
	}

	#[test]
	fn test_stat() {
		let s = r"
.#..#
.....
#####
....#
...##".trim();

		let m = stat_astr(&s);

		assert!(!m.rows[0].quads[0].is_asteroid);
		assert!(m.rows[0].quads[1].is_asteroid);
		assert_eq!(7,m.rows[0].quads[1].score);
		assert_eq!(8,m.rows[4].quads[3].score);

		let expectedstr = ".7..7\n.....\n67775\n....7\n...87\n";
		assert_eq!(expectedstr,format!("{}",m));
	}


	#[test]
	fn test_get_best() {
		let s = r"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####".trim();

		let a = stat_astr(&s);
		let winner = get_best(&a);
		assert_eq!((5,8,33),winner);
	}

	#[test]
	fn test_get_best2() {
		let s = r"
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.".trim();

		let a = stat_astr(&s);
		let winner = get_best(&a);
		assert_eq!((1,2,35),winner);
	}


	#[test]
	fn test_get_best3() {
		let s = r"
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..".trim();

		let a = stat_astr(&s);
		let winner = get_best(&a);
		assert_eq!((6,3,41),winner);
	}

	#[test]
	fn test_get_best4() {
		let s = r"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##".trim();

		let a = stat_astr(&s);
		let winner = get_best(&a);
		assert_eq!((11,13,210),winner);
	}

	#[test]
	fn test_transpose() {
		let v = vec![(0isize,0isize),(6,0),(6,3),(3,6),(3,4),(0,3)];
		let v = transpose_coords(v, &(3isize,3isize));
		assert_eq!(v[0].relative_coord.0,-3isize);
		assert_eq!(v[0].relative_coord.1,-3isize);

		assert_eq!(v[1].relative_coord.0,3isize);
		assert_eq!(v[1].relative_coord.1,-3isize);
		assert_eq!(v[4].relative_coord.0,0isize);
		assert_eq!(v[4].relative_coord.1,1isize);
	}

	//  0123456 -> x
	// 0a     b
	// 1
	// 2
	// 3f  x  c
	// 4   e
    // 5
	// 6   d
	// |
	// V
	// y

	// a 315
	// b 45
	// c 90
	// d 180
	// e 180
	// f 270

	fn r(n:i16) -> f64 {
		((n as f64) / 360f64) * 2f64 * PI
	}

	macro_rules! assert_feq (
		( $x:expr, $y:expr) => (
				assert!(
					f64::abs(
						$x
						-
						$y
					)
					<
					0.00001f64
				)
			)
	);

	macro_rules! assert_ieq (
		( $x:expr, $y:expr) => (
			assert_eq!(
					$x as i32
					,
					$y as i32
				)
			)
	);
	#[test]

	fn test_to_polar() {
		assert_feq!(to_polar(&(1,0)).1, r(0));
		assert_feq!(to_polar(&(1,1)).1, r(45));
		assert_feq!(to_polar(&(0,1)).1, r(90));
		assert_feq!(to_polar(&(-1,1)).1, r(135));
		assert_feq!(to_polar(&(-1,0)).1, r(180));
		assert_feq!(to_polar(&(-1,-1)).1, r(225));
		assert_feq!(to_polar(&(0,-1)).1, r(270));
		assert_feq!(to_polar(&(1,-1)).1, r(315));
	}
	#[test]

	fn test_angle() {
		let v = vec![(0isize,0isize),(6,0),(6,3),(3,6),(3,4),(0,3)];
		let v = transpose_coords(v, &(3isize,3isize));
		assert_ieq!(v[0].angle, r(315)*100_000_000.0);
		assert_ieq!(v[1].angle, r(45)* 100_000_000.0);
		assert_ieq!(v[2].angle, r(90)* 100_000_000.0);
		assert_ieq!(v[3].angle, r(180)*100_000_000.0);
		assert_ieq!(v[4].angle, r(180)*100_000_000.0);
		assert_ieq!(v[5].angle, r(270)*100_000_000.0);
	}
	#[test]
	fn test_magnitude() {
		let v = vec![(3isize,0isize),(1,3),(6,3),(3,6),(3,4),(0,3)];
		let v = transpose_coords(v, &(3isize,3isize));
		assert_eq!(v[0].magnitude, 3*100_000_000);
		assert_eq!(v[1].magnitude, 2*100_000_000);
		assert_eq!(v[2].magnitude, 3*100_000_000);
		assert_eq!(v[3].magnitude, 3*100_000_000);
		assert_eq!(v[4].magnitude, 1*100_000_000);
		assert_eq!(v[5].magnitude, 3*100_000_000);
	}}

fn to_polar(relative: &(isize, isize)) -> (f64,f64) {
	let x = relative.0 as f64;
	let y = relative.1 as f64;

	let h = ( x*x+y*y ).sqrt();

	let a= (x / h).acos();
	if y < 0f64 {
		(h,PI + (PI-a))
	} else {
		(h,a)
	}
}


fn get_best(map:&Map) -> (isize,isize,u16) {
	let mut winner : Coord = (0,0);
	let mut max = 0u16;
	for (r,row) in map.rows.iter().enumerate() {
		for (c,col) in row.quads.iter().enumerate() {
			if col.score > max {
				max = col.score;
				winner = (c as isize,r as isize);
			}
		}
	}
	(winner.0,winner.1,max)
}

fn get_coords(map:&Map)->Vec<Coord>{
	let mut v = Vec::with_capacity(map.rows.len());
	for (y,r) in map.rows.iter().enumerate() {
		for (x,_) in r.quads.iter().enumerate().filter(|q| q.1.is_asteroid ) {
			v.push((x as isize,y as isize));
		}
	}
	v
}

fn transpose_coords( coords:Vec<Coord>,base:&Coord) -> Vec<AsteroidInfo> {
	let mut v = Vec::with_capacity(coords.len());
	for c in coords{
		v.push(AsteroidInfo::new(c,base));
	}
	v
}
fn main() {
	let s = std::fs::read_to_string("2019/10.txt").unwrap();

	let a = stat_astr(&s);

	let winner = get_best(&a);


	println!("row,col,score: {},{},{}", winner.0,winner.1,winner.2);

	let coords = get_coords(&a);
	let infos = transpose_coords(coords,&(winner.0,winner.1));
	let mut set : HashMap<i32,Vec<AsteroidInfo>> = HashMap::new();
	for info in infos{
		let list:&mut Vec<AsteroidInfo> = set.entry(info.angle).or_insert_with(||Vec::new());
		list.push(info);
	}
	for k in set.values_mut(){
		k.sort_by(|a, b| b.magnitude.cmp(&a.magnitude));
	}
	let mut angles : Vec<i32> = set.keys().cloned().collect();
	angles.sort_by(|a, b| a.cmp(&b));

	let mut hit = 0;
	while hit < 100 {
		for angle in angles.iter() {
			if let Some(v) = set.get_mut(angle) {
				if let Some(a) = v.pop(){
					hit += 1;
					println!("{}: {},{}",hit,a.absolute_coord.0,a.absolute_coord.1);
					if hit == 200 {
						println!("x * 100 + y = {}",a.absolute_coord.0*100+a.absolute_coord.1);
					}
				}
			} else {
				panic!("aaaa");
			}
		}
	}


}
impl AsteroidInfo {
	fn new(absolute: Coord, base: &Coord) -> AsteroidInfo {
		let relative = (absolute.0-base.0,absolute.1-base.1);
		let ( magnitude,angle) = to_polar(&(relative.1*-1,relative.0));
		AsteroidInfo{
			absolute_coord: absolute,
			relative_coord: relative,
			angle:(angle*100_000_000.0) as i32,
			magnitude:(magnitude*100_000_000.0) as i32
		}
	}
}
