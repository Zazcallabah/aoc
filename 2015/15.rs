
pub fn main() {
	{
		let mut max = 0i64;

		for s in (0..=100).rev() {
		for p in (0..=100-s).rev() {
		for f in (0..=100-s-p).rev() {
		for u in (0..=100-s-p-f).rev() {
			let cap = s*5 + p*-1 + u*-1;
			if cap < 0 { break; }
			let dur = s*-1 + p*3 + f*-1;
			if dur < 0 { break; }
			let fla = f*4;
			let tex = u*2;
			let score = cap*dur*fla*tex;
			if score > max
			{
				max = score;
			}
		}}}}
		println!("sc {}",max);
	}
	let mut max = 0i64;

	for s in (0..=100).rev() {
	for p in (0..=100-s).rev() {
	for f in (0..=100-s-p).rev() {
	for u in (0..=100-s-p-f).rev() {
		let cal = s*5 + p*1 + f*6 + u*8;
		if cal != 500 { break; }
		let cap = s*5 + p*-1 + u*-1;
		if cap < 0 { break; }
		let dur = s*-1 + p*3 + f*-1;
		if dur < 0 { break; }
		let fla = f*4;
		let tex = u*2;
		let score = cap*dur*fla*tex;
		if score > max
		{
			max = score;
		}
	}}}}
	println!("sc {}",max);
}

