struct State {
	field: Vec<bool>,
	size: usize,
	alwayson: bool,
}

impl State {
	fn new(data: &str, size: usize, alwayson: bool) -> State {
		let clean: Vec<u8> = data.replace("\n", "").into_bytes();
		let mut state = State{ field:vec![false; clean.len()], size, alwayson };
		for (i, b) in clean.iter().enumerate() {
			state.field[i] = *b != '.' as u8;
		}
		state.light_corners();
		state
	}

	fn tick(&mut self) {
		let neighbours:Vec<u8> = (0..self.field.len()).map( |ix| {
			let (x,y) = self.get_coord(ix);
			self.count_neighbours(x,y)
		}).collect();

		for (ix,n) in neighbours.iter().enumerate() {
			if self.field[ix] {
				self.field[ix] = *n==2u8 || *n == 3u8;
			}
			else
			{
				self.field[ix] = *n == 3u8;
			}
		}
		self.light_corners();
	}

	fn as_str(&self) -> String {
		let dot = '.' as u8;
		let on = 'o' as u8;
		let mut strbytes = vec![dot;self.field.len()];
		for (i,b) in self.field.iter().enumerate() {
			if *b {
				strbytes[i] = on;
			}
		}
		String::from_utf8(strbytes).unwrap()
	}

	fn count_lights(&self) -> usize {
		let filtered : Vec<&bool> = self.field.iter().filter( |b| **b ).collect();
		filtered.len()
	}

	fn light_corners(&mut self) {
		if self.alwayson {
			let endix = self.size-1;
			self.field[0] = true;
			self.field[endix] = true;
			self.field[endix*self.size] = true;
			self.field[endix*self.size+endix] = true;
		}
	}

	fn get_index(&self,x:usize,y:usize) -> usize {
		y*self.size+x
	}

	fn get_coord(&self,index:usize) -> (usize,usize) {
		(
			index % self.size,
			index / self.size,
		)
	}

	fn is_on(&self,x:usize, y:usize) -> bool {
		self.field[self.get_index(x,y)]
	}
	fn count_neighbours(&self, x: usize, y:usize ) -> u8 {
		let mut count = 0u8;
		let size = self.size-1;
		if x > 0 && y > 0 && self.is_on(x-1,y-1) {
			count += 1
		}
		if y > 0 && self.is_on(x,y-1) {
			count += 1
		}
		if x < size && y > 0 && self.is_on(x+1,y-1) {
			count += 1
		}
		if x > 0 && self.is_on(x-1,y) {
			count += 1
		}
		if x < size && self.is_on(x+1,y) {
			count += 1
		}
		if  x > 0 && y < size && self.is_on(x-1,y+1) {
			count += 1
		}
		if  y < size && self.is_on(x,y+1) {
			count += 1
		}
		if  x < size && y < size && self.is_on(x+1,y+1) {
			count += 1
		}
		count
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	static INPUT_DATA : &str = r"o...oo......o......oo.oo..o...oo......oo.o.o.ooo.o.o..o..o......oooo..o......ooo.o.o....o..oo..ooo..oooo..o.o...o....o.ooooo.oo.oo.o..o.......o....o.oo...ooo.ooo..o.o.o........o..o.o.oo...oo..o.oooo.o...o..oo...o.o.ooo.o.ooo..o.oo.oooo.ooo...o...........o.ooo..oo.o.oo.o.ooo...o.o..ooo....o.ooo.o..o..o...oo...oooo.o..o.....o..o...o.o.oo...o...oo..o.o.ooo....o..ooo.....oo..o.ooo..ooo.....oo..ooo...o..oo.ooooo....oo..o.o..oo.oo..oooooo...o..ooo.oooooo.....o..oo...o.o..oo..oo..o..o..o..oo.o.o.o.o....ooo.ooo.ooo...oo...oo..ooo..oo.ooo.o.....oo..oo.o.ooooooooo...oo..oo.o..oo.o..oo..oooo..o.o.o.oooooo.ooooo..ooo.ooo.oo.oo.o...o.o.o.o..o.ooo...o..oo.ooo.o...oooo.o..o.o.....ooo..o..oooo..o.o.o...oo......o...oo.....o....oooo.oo.o.ooo..o.o.oo..o.o...oo.ooo.ooo..o.oo..o.o.oo..oo..o.oo.ooo..o.o.ooo.ooooo.oo...o.oo...o.o..o.o..o...ooo...ooo.o..o..o.ooooo..ooo.o......o.....ooo.ooooo.o.o..o.o.o.oo..o.o.o.o..o.....o.....oo.o..oo...ooo..oo...oo...ooo.o.ooo.o..o.o.ooo...oo..oo..o.ooo...o.ooooooo.o...o.o.o.o.....oooo.o..o.oo...o.oo....ooooo.ooo.o.....ooooo....ooo..o........oo..oooo...o...o.ooo....o..ooooo.o.oo..o.o.oo.o.....oo.o.....ooo.oooo.o..oooooo.....oooo.o.o..oo.o.oo...o..o.o.....o.oooo.o.......o..o..o.o..o.oooooo.oo..oo.oooo.....oo.o.oo.o.oooooo..o.o....o.o...o.o..o..o.o.ooo.o..o.o.o..o...ooooooo..oooo.o.o.ooo.....o.o.o.oo..o.oo.oo.oo.o..oo..oo.o.oo.....o.o..o.oooo.....ooo.o..o.oooo.o.o..ooooo.oo..oo.o.oo..o..oo...o.ooooo.oo.o....oo.oooo.o.oo....o..ooo.o.o.oo...o.....o.o.o.o.o..oo.o.o..o.......o..oooo...oo.oo...o.oo.oo...oo..o..oo.ooo..o...o..oo...o.o....ooo.oooo...o.oo.ooo.o.oo.oooo.oo..o...ooooo.o.o..o.oo....o..o...o..oooo.....ooo...oo.ooo....o..o.ooo...o........o.o.oo..o..o.o.....oooooooo.o.o.ooo.ooo..oooooo.oo..ooooo.oo.ooo.ooo....oooo.o..oo.oo...ooo.o..oooooooooooo.o.oo....oo.oo.o...oo.ooo.o.ooo..o.o.o.o.o.o..oo..oooo.o..oo.....o.oo..o.oo...oo.o..oo..o.o.o....oo....oo.o..o.o...o.o.oooo.....ooo..ooooooo.o.o.o.o...oo.ooooo.....oo...oo...oo.ooo..oooooo.ooo..o...oooo.o..ooo.ooo.o.oo....o.o.oo..oo.o.oo.oo..oooooo...o.....o..o.o.o.o.....o.o..oo.o.o.......ooooooo....o.......o.....ooo.oo.oo..oo....o.ooo...o.....oo..oo......ooo...oo..ooo.oo...oo.ooo.o.o.o.ooo.ooo.o.o...ooo..o....oo.o.o...o...oo.o.o...o..o..o.o...oo.o.oo...oo..o....o.o..oo.o..o.o..o.o.....o..o.o...ooooooo.o.oo....oooo....o.ooo.o..ooo..oo...oo..o.o.o.ooo...o..oo.oo.oo..oo.o...o..o.oo.....o.o........o..o.o.oooo.....oo..ooo...o....o.o.o.o...ooo.ooo...o.o...o.o.oooo....o..oooo...ooo..o..oooooo..oo.oo..ooo.oooooooooo.oo..o....ooo.ooo....oo.....o.o..o....o.ooooo.oo.o.oooo.o.oo...o..ooo...ooo..oo...o.ooo.ooooo..ooo.oo..........oooooooo.oooooo....oooo.ooo.o..oo...o.oo.oooo.o.....oo..ooooo..ooo...ooooo.....o.o.ooo..ooooo.oo.o.ooooo.o.oo.oo..o.oo....oooooooo.ooooo.o...o.ooo.oo...o.ooo.o.o..o....oo.o..o...o.o.o..oo.o....o..o...o..ooooo..o..oo.o......o..o....oooooooo...o..o...o.....oooo.o...oo...o.ooo.o.o..oo.o.oo.oo.o.oo.o.oo...o.o.o..oo.oo.ooo.o..oo..o...ooo.oo.ooo.ooooo.o.ooo..o..ooo.o...o.ooo.o...o..o.o.o.o..o..o.o..o..ooo..o....ooo.oooo.oo.o.ooo.o.oo.ooo.o.oo.ooo.ooo...ooo...ooo.o...oooo...o.oo.oo.o.o.ooo..oo...ooo...o..oo.o..o.o...oo....ooo.oo.oo..ooooo....ooo..o..o....o..ooo.ooo.o...o.oo...o.o.o..oo....o.......oo.....o.oo...o..o.ooo.o.oo..oo..o.oo..o.ooo..oo.oo...ooooo.o..ooooo..ooooo..ooooo....o.oooo.oooo....ooo..ooo.o.oo.oooo.oo.o...oooo.o.ooo.o.....o...oooo..ooooo.ooo..o.o.ooo.oo.oo...oo..o.oooo..oo...oo.oooooooo...oo..ooo..o..ooo.oo.o.o.o........o.ooooo.o...o.ooo.oooo.o..oooo..o.o.o....ooooo.o..o...ooo.o..o..o.ooo...oo..ooo.oo.o.o...o..o...oooo..oo....o.o..o..oo.o.o...ooooo.ooo.o..o.o.o...oo....o.ooo.o.o..oo...oo.ooo.o..o..o......o...o.o..oooo.o.oo..oooooo.oooo.o...o..o..o..oo.o.o.oo.oo.oooo.o...o..o.o.oo..oo.o.o.ooo..oo...oooo......o..oooooo.o......o.oo.o....oo...ooo.o.o..o......ooo.....o...ooooooo.oo.o..o.o...ooo.o..o.oooo....o.o.oo.o.oo...ooo..o...o.ooo.oo..o.ooo..o.oo...ooooo.ooooo.oo...o..o.o.o.......o.oo..ooooo..ooooo...ooo..oo.o.o..ooo.o.ooooo.oooo..o.o..oo...o.oo...o.ooo.oo.o..ooooooo.ooo.o.oooo.....oo...o.oo.o.o..o...oo....oooo......oooooo.o..oooooo.....oooooooooo.oo.oo...o.o..o.oo.ooo.o.o.o.oo.ooo.oo..oo.oo.oo...o.o..ooo.ooooooo..o.....ooooo..o....oooooo.o..oo..ooo.o.o.ooo.....o..oo..o.o..oo..o.ooo...ooo.o..oo...o...o.ooooo.o.ooooo..ooo.o..o...oo..o.o..o..oooo....o......oo..o.....oooo.ooo....oo.ooo.....ooo.oo........o.ooo.oo..o..o.ooooooo.o.oooooo..oo..ooo.......oo.o.o..o.oo...o.ooo.ooo...oooooo..o..o.o..o....ooo.o.o....o..........o...oo.oo.oo.o..oo..o.ooooo.ooo.ooo.o..o.oo..oo.o..o..oo.....oo.....o..ooooooo.o..o.o.o.oooo.ooo..ooo.o.o..o.oo.oo.oooo.ooo.ooooo.o.o..o....oooooooo.o..o..o...oo..o.oo..o.o..oo..oooo...oo.....o.oo.o.o...oooooooo..o.ooo.o..o.o.oo.oo.....o...o.o...oo.oo....ooo...oo..o.oooo...o..o.o..o..o.oo..o.ooo.oo.oooo.oo..oooo.....oo.o.....o....oooo.o.oo.o.oo.o..oo.o.oooooo.oo.oooo..o...oooo.o..ooo.o.o..o..oo.o.o.....oo.ooooo.o.oooo...o.o.o..ooooo.ooooo.....oo....oooooo..oo....o..o.o.ooo.ooooo.....oo.oo.oooo.o...oo...o.oo.o.ooooo.oo.o...oo.oooo..ooo.o....o...o.o.o.o.ooo.ooooo.o.oooo..oooo...oooo......oo..o..o..o.o.oo...oooooooo....o....ooo.o.o.o.o..oooo.oo.o..oooooo..o.o.ooo.....o.o......o.o.o.o..oooo.oo...oo.ooooo.o.oo..oo..o..o.o.......ooo...o...o.oooo.ooo.o.o.o.o.....o....o.oooo.ooo.oo.oo.oo.ooooooo......o.oooo......o....oo.....oo..o..o.o.oo..o...o..oo.oo.oo..ooo.o....oo.oo....oooo.o.oo.ooo....o.oo.o.o.oo...oo.ooo...o..o..oooo...o.o..oo..oo.o...oo.oo...o.o......o.o.oo..ooo....oooo.oo...o.o.ooo.o..o..o.oooo..oo..oo..ooooo.ooo.oo.oo..oooooooooo.oo...o.oo.oooo.o.ooooooo.oo.o.oo.oo..o...oo....oooooooo.ooo..oo.oo.oo.o..oo.o.o.oooooo.o....o.oo..o.....o......oo.oo..o.oo.ooo..oo.......ooo..oo.o.ooo.oo.ooo....oooo.o..o.ooo..o.o.o.o...o..o.oo....oo....o...oooo....o...o..o...oooo...oooooooo.ooo.o..oo.o.o.oo..ooo..o.o.ooo.....oo.ooo..oo.....ooo......o..ooo.oo.oooo.oo.oooo.o.o....o..o...o..o.o..o.ooo.o...o...o..oo.oo...o..ooooooo.....oo..ooo..oo...ooooo.o.o.....ooo.o.o..oooo...o.o.o..o..oooo..oo.o..ooo.oooo.o....oo..ooo....o..oo.o.oo.o....o.ooooo.o....oo...o...oo...oo....o.o.......o....o..o...ooo.ooo.o.oooo..oooo....o.oo.o.o...oo...oo..ooo.o.o.oo.o..o....o.o.....oo.ooo.o.ooo.ooo.....o...o.o..ooooooo.ooooo..o.ooo...oo......oo......ooo..o....o.o..o.ooo.oo.o...oo..ooo.oooo.o.o....o.oo..o.ooo..oo.o..ooooo..oo.ooo.....o..ooo..oo.o.oo..oo.ooo.o..oo.....o.oo.....ooo....oo.oooo.oooooo.o...o..ooo....o.o...o.oo.....ooo....o..o.o..oo.o.o.o.oo..o.o.o..oo..o.ooo.oooo....o..ooo.oooooo..oooo.o.....ooo.oo..o...ooo.o..oooooo.oo.o.oo.....oo.oooo.o..oo.ooooo.oo.o...oo..o..o...o.o.o.ooooo...o....o..ooo...o..o....o.o.oo.o.oooooo.o..oooo..o.o.o.o...o.oooooo.o.....o..o.o..ooo....o.o.oooooooo...o....o.o.oo..o...oo...o.o..o.o.ooo....oo...ooooo..o..oo..o..oo..o..o.o.oo.o....ooooo.oooo.oo.o.ooo..oo..oo....o.....o.ooooo.o...o.ooooo.oo.o.o.o.o..ooooo...oooo.ooo.ooo.....oooo.ooo.....oo...oo...o..o..ooooooo.o.oo....oo..oooo.....oo...o..o..oo.o.ooo.o.o..oo..o....o.o...o.o.oo.oo..o.oo.....oo...o.o..oo.......oo.o.ooo..ooooo.o.oo....o.oo........o.......o....o.o.oooo.o.ooo.ooo..o....o..oo.o..oooo........o.oo..o...o.o...ooo.o..o.o.o...o...o.....oo.ooooo.oo.o.ooo.oo.oo.o.oo..oo.o.o.o.o.o.oo.o..oo...oo.o.o..o..oo.oo.ooooo.o.ooo...ooooo..o..o.ooooooo.o..o..o....oo.o.o..oooo.o..o..ooo...o..o.......ooo.o.o.oooo....o.ooo...o.o.ooo.o.o.o.o..ooo...oo.oo.o.oo.ooo....ooo.oo.o.ooo.o...o....o.oooo..ooo..ooo.o.o..o...oo.o.o.o..oo.ooo..ooo.o.oo...ooooooooo..oooooo..oo..oo.o.o.oo.oo.o..oo..o.o.o.oo..o.o...o...o.o.o..oooooo.o..o.o.oooooo..o......oo.oo.ooooo.....o.......ooooooooo..ooo.oo...o...oo.o.o..o...ooooo...o...o..o.ooo.o..o.o...ooo.o.o.o...o.o....oo....ooo...oo.oo.o...oo.........oo.o.o..o.o.oo.o.oooooo.ooooo..o..ooo.ooo.o...o.o.oo.oooooo...o.o...ooo.o.ooo.oo.o.oooooo.ooooooo.ooo.oo..o.o.o...oooooo.oo.oooo.oo..o.o.o.o......oo..oo.........o..ooo..oo....o.....oo...o.o.ooo.o.o.....oo.o...ooo.oooo.o...o...oo..oo.o.o.oooo..ooo...oooooo....o.o..ooo.o.oo.oooo.o..o.oo....oo..ooooo....o..oo.oo.o..ooooooo...o.oooo...oo.o.o.oo.........o....o....o.oo.o...o.oooo..o.o...o.oo..oooooo.oo..oo.o.ooo.oo..ooo.ooo....oo..o.oo.oo..oo.o...ooo.oo.oo.ooo....o...ooo.ooo.o..o....o.......o..o.....ooo..o.ooo.oo.oo....o.oooo.o.oooo.oo..oo..o..o.....o....oo.o.o..oo.o..o..o.oo.......o.oooo.ooooooo.....o.oo.oo.o.....o.o..o....oooooo.o..ooo.oo.oo.....o.oooo..oo.oooo..o.ooo.o..oooo.....oo....o..oooo....o.oo.oo..o...oooooo.ooooooooo...o.o....oo...ooo.o..o.oo...o.o..ooo..oo..o.o.oo.ooo.o.o.oo...ooo.o...oo.oo..o.ooo....ooo..o.o...o.ooo..oooooo.o..o.ooo..o..o..o.o.o........oo.o.ooo..ooo.o.o.oo.....oo.oo.o.o...oo..o.oo....ooo..o.o.o.o.oo....o.oo..o.o...ooo...o...oooo.oooo..o....o.o.o..o..oo.......oo.oooo...ooo.oo..o.o.oo.o..oo..oooooo.......oo.o.oo..o...o.....o..o..o..ooo..oo.oo..oooooo.o..ooo..ooo.o.oo..oo.o..ooooo.o.o.o.oo..o.oo..oo.oo......oooo.o.............oo.oo..ooo.o...ooo....o.o.o.o.....o.oo.....oo...o...o......oooo...oo.oo....oo.o..o.oooo.o..ooo.o...o.....oooo.o.ooo.ooooo..o..ooo..o..o.o...ooooo...ooo.ooo....o.ooo..o...o..o..o.o..o.oo..oo.o.o......oo.ooooo...ooo.ooo.........o....oo.oooo.oo..o.o..o.o...o...oo.oo.oo..o.o.oo.oooooooo......ooooo......ooo.o.o..o...o.ooo.ooo.......oo.ooo.o..o.oooooooooo...o..o.o.o.oo.o.ooo...oooooo..o.o...ooo.oo....o.o.ooooooo.o..oo.oo..oo...o...oooo...o..ooooo.o..oo...ooo.o.o...o.oo...o......o..oo.oooo..o.....oo.oo.oo.o.o......ooooooo..ooo.....oo.o.oo..ooo......o....oooo...o.ooo.o.oo.o........o..o....oo.....ooo...o.ooo.o.oo...oo.oooo....o...o.ooo..o.o.....o.o....o.o.o.oo...o.o..ooooo.o.o..o..o..o....o...oooo.....oo...ooo......ooooo..oo.oo.oo...oo.o.ooooo..oo...o.o.o.o.ooo...ooo.oo.oooo..o.o..o.o..o.oooo.ooo..o..oo.o.oo.o.oo.o.o.o..ooo....ooo.oo.o.oo.o...o.o..o...o....ooo.o..o.o.oooooo.o...oooo..o..oo.o.oo..o.o..o...ooo.o..oo.o...o...oo.o......o...o..o..oooo..oo.....o.ooo...o.o..o.o....o.ooooo.oo.ooo...ooo....o.o..o.o..ooo..o.oo......o...o..o..oo.o..ooo..oo..o..o.oooo..o...oooooooo..oo.o.oo.o.o.o...o..o.o.oo.oo.ooo..o...o.o....o..o.oo..o.o.o.o.oo.oo.ooooo...o........oooo..ooo..oooo.ooooo..o.oo.o.oo.";

	#[test]
	fn test_runto100() {

		let mut data = State::new(INPUT_DATA,100,false);
		assert_eq!(INPUT_DATA,data.as_str());
		for _ in 0..100 {
			data.tick()
		}
		assert_eq!(814,data.count_lights());
	}


	#[test]
	fn test_runto100alwayson() {
		let mut data = State::new(INPUT_DATA,100,true);
		for _ in 0..100 {
			data.tick()
		}
		assert_eq!(924,data.count_lights());
	}

	#[test]
	fn test_getcoord() {
		let data = State::new(INPUT_DATA,100,false);

		let (x,y) = data.get_coord(0);
		assert_eq!(0,x);
		assert_eq!(0,y);
		let (x,y) = data.get_coord(99);
		assert_eq!(99,x);
		assert_eq!(0,y);
		let (x,y) = data.get_coord(100);
		assert_eq!(0,x);
		assert_eq!(1,y);
	}

	#[test]
	fn test_newstate() {
		let data = State::new(r".o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,false);
		assert!(data.field[1]);
		assert!(!data.field[6]);
	}

	#[test]
	fn test_countneighbours() {
		let data = State::new(r"
.o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,false);
		assert_eq!(1,data.count_neighbours(0,0));
		assert_eq!(0,data.count_neighbours(1,0));
		assert_eq!(3,data.count_neighbours(2,0));
		assert_eq!(2,data.count_neighbours(3,0));
		assert_eq!(4,data.count_neighbours(4,0));
		assert_eq!(1,data.count_neighbours(5,0));

		assert_eq!(2,data.count_neighbours(0,1));
		assert_eq!(2,data.count_neighbours(1,1));
		assert_eq!(3,data.count_neighbours(2,1));
		assert_eq!(2,data.count_neighbours(3,1));
		assert_eq!(4,data.count_neighbours(4,1));
		assert_eq!(3,data.count_neighbours(5,1));

		assert_eq!(6,data.count_neighbours(1,4));
		assert_eq!(1,data.count_neighbours(5,5));
	}

	#[test]
	fn test_tostr(){
		let data = State::new(r"
.o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,false);

		assert_eq!(".o.o.o...oo.o....o..o...o.o..ooooo..",data.as_str());

	}

	#[test]
	fn test_tick(){
		let mut data = State::new(r"
.o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,false);
		data.tick();
		assert_eq!("..oo....oo.o...oo.......o.....o.oo..",data.as_str());
		data.tick();
		assert_eq!("..ooo.........ooo........o.....o....",data.as_str());
		data.tick();
		assert_eq!("...o...........o....oo..............",data.as_str());
		data.tick();
		assert_eq!("..............oo....oo..............",data.as_str());
	}

	#[test]
	fn test_tostr_alwayson(){
		let data = State::new(r"
.o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,true);

		assert_eq!("oo.o.o...oo.o....o..o...o.o..ooooo.o",data.as_str());

	}

	#[test]
	fn test_tick_alwayson(){
		let mut data = State::new(r"
.o.o.o
...oo.
o....o
..o...
o.o..o
oooo..",6,true);
		data.tick();
		assert_eq!("o.oo.ooooo.o...oo.......o...o.o.oooo",data.as_str());
	}
}
