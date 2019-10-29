
fn elfwalk(upto:usize) -> Vec<u64> {
	let mut houses = vec![10u64;upto];
	for elf in 2..houses.len() {
		for address in (elf..houses.len()).step_by(elf) {
			houses[address] += (elf as u64)*10;
		}
	}
	houses
}

fn lazyelfwalk(upto:usize) -> Vec<u64> {
	let mut houses = vec![10u64;upto];
	for elf in 2..houses.len() {
		let mut visits = 0;
		for address in (elf..houses.len()).step_by(elf) {
			houses[address] += (elf as u64)*11;
			visits += 1;
			if visits == 50 {
				break;
			}
		}
	}
	houses
}

fn findfirst(houses:Vec<u64>,limit:u64) -> usize {
	for (address,presents) in houses.iter().enumerate() {
		if presents >= &limit {
			return address
		}
	}
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_elfwalk() {
		let houses = elfwalk(10);

		assert_eq!(10, houses[1]);
		assert_eq!(30, houses[2]);
		assert_eq!(40, houses[3]);
		assert_eq!(70, houses[4]);
		assert_eq!(60, houses[5]);
		assert_eq!(120, houses[6]);
		assert_eq!(80, houses[7]);
		assert_eq!(150, houses[8]);
		assert_eq!(130, houses[9]);
	}

	#[test]
	fn test_canfindfirst() {
		let houses = elfwalk(3_000_000);
		let value = findfirst(houses, 33_100_000);

		assert_eq!(776160,value);
	}

	#[test]
	fn test_canfindlazyfirst() {
		let houses = lazyelfwalk(3_000_000);
		let value = findfirst(houses, 33_100_000);

		assert_eq!(786240,value);
	}
}
