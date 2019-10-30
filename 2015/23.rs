fn main() {
	let mut a: u32 = 1;
	let mut b: u32 = 0;
	if a != 1 {
		a += 1;
		a *= 3;
		a += 1;
		a *= 3;
		a += 1;
		a *= 3;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
	}
	else {
		a *= 3;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a += 1;
		a *= 3;
		a += 1;
		a *= 3;
		a += 1;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a += 1;
		a += 1;
		a *= 3;
		a *= 3;
		a += 1;
	}

	while a != 1 {
		b += 1;
		if a % 2 == 0 {
			a /= 2;
		}
		else {
			a *= 3;
			a += 1;
		}
	}
		println!("b is {}",b);
}
