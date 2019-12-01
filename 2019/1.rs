fn main () {
	let data = std::fs::read_to_string("2019/1.txt").unwrap();

	let modulemass = &data.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();

	let modulefuel : u64 = modulemass.iter().map( |m| fuel(*m) ).sum();

	let totalfuel : u64 = modulemass.iter().map( |m| fuelforfuel(*m) ).sum();

	println!("module fuel cost: {}", modulefuel);

	println!("total fuel cost: {}", totalfuel);
}

fn fuel(mass:u64)->u64{
	let d = mass/3;
	if d > 2 { d-2 } else { 0 }
}

fn fuelforfuel(mass:u64) -> u64 {
	let fuel_needed = fuel(mass);
	if fuel_needed > 0 {
		fuel_needed + fuelforfuel(fuel_needed)
	}
	else {
		fuel_needed
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_can_fuel(){
		assert_eq!(2,fuelforfuel(14));

		assert_eq!( 654, fuel(1969));
		assert_eq!(966,fuelforfuel(1969));
		assert_eq!(50346,fuelforfuel(100756));
	}
}