// cpy a d          | intro
// cpy 9 c
// cpy 282 b
// inc d
// dec b
// jnz b -2
// dec c
// jnz c -5
// cpy d a          | reset
// jnz 0 0          | start
// cpy a b
// cpy 0 a
// cpy 2 c          | division
// jnz b 2
// jnz 1 6
// dec b
// dec c
// jnz c -4
// inc a
// jnz 1 -7
// cpy 2 b          | remainder
// jnz c 2
// jnz 1 4
// dec b
// dec c
// jnz 1 -4
// jnz 0 0          | print
// out b
// jnz a -19
// jnz 1 -21




// First part, lets call it "intro"

// cpy a d
// cpy 9 c
// cpy 282 b
// inc d
// dec b
// jnz b -2
// dec c
// jnz c -5
fn prep_facit(input: i32) -> i32 {
	let mut d = input;
	let mut c = 9;

	while c > 0 {
		let mut b = 282;
		while b > 0 {
			b -= 1;
			d += 1;
		}
		c -= 1;
	}
	d
}
fn guess(i: i32) -> i32 {
	9i32 * 282i32 + i
}

// is equivalent of just input += 2538
// registry contents when done: [_,0,0,2538+input]




// INTRO
//[_,0,0,2538+input]

// cpy d a          RESET  <- will end up [0,2538input,_,_]
// jnz 0 0          START  <- will end up [0,floor(2538input/2),0,_]
// cpy a b
// cpy 0 a


// unrolling the firstloop gives us
// firstpart:
// if b == 0 {
// 	GOTO secondpart with c = 2
// }
// b -= 1
// if b == 0 {
// 	GOTO secondpart with c = 1
// }
// b -= 1
// a += 1
// GOTO firstpart

// c was either 2 or 1 depending on what b was.
// b started as a big number.
// then a = b/2, b=0. remainder i guess in c? 2 even, 1 odd
// unrolling the second loop gives us
// secondpart:
// b = 2
// while c > 0 {
// 	b -= 1
// 	c -= 1
// }
// // [ a=floor(previous b/2), 0 if even 1 if odd, 0, _]
// println!(b);
// if a == 0
// 	GOTO RESET
// else
// 	GOTO START

// so finally we come to this interpretation

// let m = input + 2538;
// while m > 0 {
// 	let a = floor(m / 2);
// 	let b = m % 2;
// 	send(b);
//  m = a;
// }
// goto 10


// while we could maths this problem to the finish line,
// there is an interpretation that running the while loop once is enough,
// then we test that the contents are every other bit,
// and that the first and last elements are different

fn work(input: i32,add:i32) -> (Vec<i32>,Vec<i32>) {
	let mut collect = Vec::new();
	let mut debug = Vec::new();
	let mut m = input + add; // 2538
	while m > 0 {
		let a = m/2;
		let b = m%2;
		collect.push(b);
		debug.push(m);
		m = a;
	}
	(collect,debug)
}
fn main() {
	let w = work(32,0);
	println!("{:?}",w.1);
	println!("{:?}",w.0);
	let w = work(31,0);
	println!("{:?}",w.1);
	println!("{:?}",w.0);

	// wait this is suspicious

	// new theory, the result is just the bits of given number in reverse order?

	let w = work(0b0010_1010,0);
	println!("{:?}",w.1);
	println!("should be every other bit? {:?}",w.0);

	let w = work(0b0101,0);
	println!("{:?}",w.1);
	println!("does it work with trailing ones? {:?}",w.0);
	println!("no appearently not, so least significant bit must be 0");

	// so the final question is, what number following 2538 starts with 1 and alternates bits, ending on 0
	println!("0b1010 = 0xa");
	println!("0xa    = 10");
	println!("0xaa   = 170");
	println!("0xaaa  = 2730");

	// so with puzzle input
	// cpy 9 c
	// cpy 282 b

	// ge get
	let mul = 9*282;
	println!("\n\n{} - {} is your number: {}", 0xaaa,mul,0xaaa-mul);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_intro() {
		let g = (0..10).into_iter().map(|i| guess(i) ).collect::<Vec<i32>>();
		let f = (0..10)
			.into_iter()
			.map(|i| prep_facit(i) )
			.collect::<Vec<i32>>();
		assert_eq!(f, g);
	}
}
