// ditch the arithmetic for now

struct Data {
	data: Vec<bool>,
}

impl Data {
	fn make(s:&str)-> Data{
		let data = s.chars().map(|c| c == '1').collect();
		Data{data}
	}
	fn expand(&mut self) -> &[bool] {
		let init_len = self.data.len();
		self.data.push(false);
		let slice = &self.data[0..init_len];
		let mut tmp_storage = Vec::with_capacity(init_len);
		for bit in slice.iter().rev(){
			tmp_storage.push(!bit);
		}
		self.data.extend(tmp_storage);
		&self.data
	}
	fn expand_to(&mut self, len: usize) {
		while self.len() < len {
			self.expand();
		}
	}
	fn len(&self)->usize{
		self.data.len()
	}
	fn chksm(&self, len: usize)->String{
		let slice = &self.data[0..len];

		let mut result : Vec<bool> = slice.chunks(2).map(|c| c[0] == c[1] ).collect();

		while result.len() % 2 == 0  {
			result = result.chunks(2).map(|c| c[0] == c[1] ).collect();
		}

		result.iter().map(|&c| if c { '1' } else { '0' } ).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_make() {
		let d = Data::make("100");
		assert_eq!(vec![true,false,false], d.data);
	}

	#[test]
	fn test_expand(){
		let mut d = Data::make("1");
		d.expand();
		assert_eq!(Data::make("100").data,d.data);
		assert_eq!(Data::make("100").data,Data::make("1").expand());
		assert_eq!(Data::make("001").data,Data::make("0").expand());
		assert_eq!(Data::make("11111000000").data,Data::make("11111").expand());
		assert_eq!(Data::make("1111000010100101011110000").data,Data::make("111100001010").expand());
	}

	#[test]
	fn test_chksm(){
		let d = Data::make("110010110100");
		assert_eq!("100",d.chksm(d.len()));
	}

	#[test]
	fn test_expand_to() {
		let mut d = Data::make("10000");
		d.expand_to(20);
		assert_eq!("01100",d.chksm(20));
	}
}

fn main() {
	let mut d = Data::make("00101000101111010");
	d.expand_to(272);
	println!("checksum: {}",d.chksm(272));

	let mut d = Data::make("00101000101111010");
	d.expand_to(35651584);
	println!("checksum 2: {}",d.chksm(35651584));
}