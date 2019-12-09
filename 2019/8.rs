
struct Layer {
	data: Vec<u8>,
}

impl Layer {
	fn new(data:Vec<u8>)->Layer{
		Layer{data}
	}

	fn from(s:&str)->Layer{
		let data = s.bytes().collect();
		Layer::new(data)
	}

	fn count(&self,c:u8)->usize {
		self.data.iter().filter( |&d| d == &c ).count()
	}
}

struct Image {
	layers: Vec<Layer>,
}

impl Image {
	fn new( s:&str, x:usize,y:usize) -> Image{
		let layers = s.as_bytes().chunks(x*y).map(|c| Layer::new(c.to_vec())).collect();
		Image{layers}
	}

	fn get_chklayer(&self) -> &Layer {
		let mut min = self.layers[0].count('0' as u8);
		let mut lowest = &self.layers[0];

		for l in &self.layers {
			let m = l.count('0' as u8);
			if m < min {
				min = m;
				lowest = &l;
			}
		}
		lowest
	}

	fn get_image(&self) -> Vec<bool> {
		let l = self.layers[0].data.len();
		let mut out = Vec::new();
		let transparent = '2' as u8;
		let white = '1' as u8;
		for i in 0..l {
			let mut d = 0;
			while self.layers[d].data[i] == transparent {
				d += 1;
			}
			out.push( self.layers[d].data[i] == white);
		}
		out
	}
}

fn tostr(b:&[bool],l:usize) -> String {
	let mut s = String::with_capacity(b.len());
	for row in b.chunks(l) {
		for &c in row {
			s.push( if c { '#' } else { ' ' } );
		}
		s.push('\n');
	}
	s
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getimage(){
		let i = Image::new("0222112222120000", 2,2);
		assert_eq!(vec![false,true,true,false],i.get_image());
	}

	#[test]
	fn test_layer() {
		let l = Layer::from("00000011");
		assert_eq!(6, l.count('0' as u8));
	}

	#[test]
	fn test_image(){
		let i = Image::new("123456789012",3,2);
		assert_eq!(2,i.layers.len());
		assert_eq!(0,i.layers[0].count('0' as u8));
		assert_eq!(1,i.layers[1].count('0' as u8));
	}

	#[test]
	fn test_count(){
		let i = Image::new("000111200000",3,2);
		assert_eq!(3,i.layers[0].count('0' as u8));
		assert_eq!(3,i.layers[0].count('1' as u8));
		assert_eq!(1,i.layers[1].count('2' as u8));
		assert_eq!(5,i.layers[1].count('0' as u8));
	}

	#[test]
	fn test_least(){
		let i = Image::new("000111200000",3,2);
		let c = i.get_chklayer();
		assert_eq!(3,c.count('1' as u8));
	}
}

fn main(){
	let i = Image::new(&std::fs::read_to_string("2019/8.txt").unwrap(), 25, 6);
	let l = i.get_chklayer();

	let ones = l.count('1' as u8);
	let twos = l.count('2' as u8);

	println!("part 1: {}",ones*twos);
	println!("part 2:");

	println!("{}", tostr(&i.get_image(),25));
}
