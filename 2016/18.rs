
struct Floor {
	rows : Vec<Row>
}

struct Row {
	traps_data: Vec<bool>
}

impl Floor {
	fn count_safe(&self)->usize{
		self.rows.iter()
			.map(|r| r.traps() )
			.flatten()
			.filter(|&t| !t)
			.collect::<Vec<&bool>>()
			.len()
	}

	fn new(start:&str,size:usize) -> Floor {
		let mut rows = Vec::with_capacity(size);
		let mut last = Row::new(start);

		for _ in 0..size-1 {
			let next = last.next();
			rows.push(last);
			last = next;
		}
		rows.push(last);
		Floor{rows}
	}
}

impl std::fmt::Display for Floor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let row : String = self.rows.iter().map(|r| {let mut s = r.to_string(); s.push('\n'); s}).collect();
		write!(f, "{}", row)
	}
}

impl std::fmt::Display for Row {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let row : String = self.traps().iter().map(|&t| if t { '^' } else { '.' }).collect();
		write!(f, "{}", row)
	}
}

impl Row {
	fn traps(&self)->&[bool] {
		&self.traps_data[1..&self.traps_data.len()-1]
	}
	fn new( data:&str ) -> Row {
		Row::from(data.chars().map(|c| c == '^').collect())
	}

	fn from( mut traps_data:Vec<bool> ) -> Row {
		traps_data.insert(0, false);
		traps_data.push(false);

		Row{traps_data}
	}

	fn next(&self) -> Row {
		let mut v = Vec::with_capacity(self.traps_data.len());
		for i in 0..self.traps_data.len()-2 {
			let parents = &self.traps_data[i..i+3];
			v.push( Row::is_trap(parents) );
		}
		Row::from(v)
	}

	fn is_trap(parents:&[bool]) -> bool {
		let left = parents[0];
		let center = parents[1];
		let right = parents[2];
		 left &&  center && !right ||
		!left &&  center &&  right ||
		 left && !center && !right ||
		!left && !center &&  right
	}

	fn count_safe(&self) -> usize {
		self.traps().iter()
			.filter(|&t|!t)
			.collect::<Vec<&bool>>()
			.len()
	}

	fn count_smart(&self,size:usize) -> usize {
		let mut last = Row{traps_data:self.traps_data.clone()};
		let mut sum = self.count_safe();
		for _ in 0..size-1 {
			last = last.next();
			sum += last.count_safe();
		}
		sum
	}

	// rust doesnt have tce, here we stand
	// fn count_smart(last:Row,depth:usize,sum:usize) -> usize {
	// 	let lastcount = last.count_safe();
	// 	let new_sum = sum + lastcount;
	// 	if depth > 1 {
	// 		let current = last.next();
	// 		Row::count_smart(current,depth-1,new_sum)
	// 	}
	// 	else {
	// 		new_sum
	// 	}
	// }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_can_print() {
		assert_eq!("..^^.",Row::new("..^^.").to_string());
	}

	#[test]
	fn test_next_row() {
		let row = Row::new("..^^.");
		assert_eq!(".^^^^",row.next().to_string());
		assert_eq!("^^..^",row.next().next().to_string());
	}

	#[test]
	fn test_floor() {
		let floor = Floor::new("..^^.", 3);
		assert_eq!("..^^.\n.^^^^\n^^..^\n",floor.to_string());
	}

	#[test]
	fn test_count_safe() {
		let floor = Floor::new("..^^.", 3);
		assert_eq!(6,floor.count_safe());
	}
	#[test]
	fn test_count_safe_large() {
		let floor = Floor::new(".^^.^.^^^^", 10);
		assert_eq!(38,floor.count_safe());
	}
	#[test]
	fn test_part_1() {
		let floor = Floor::new(".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^", 40);
		assert_eq!(1989,floor.count_safe());
	}
	#[test]
	fn test_smart_count() {
		let row = Row::new(".^^.^.^^^^");
		assert_eq!(38,row.count_smart(10));
	}
	#[test]
	fn test_part_2() {
		let row = Row::new(".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^");
		assert_eq!(19999894,row.count_smart(400000));
	}
}