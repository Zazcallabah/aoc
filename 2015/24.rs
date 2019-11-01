
fn quantum( list: &[i32] ) -> u64 {
	let mut quantum = 1u64;
	for w in list {
		quantum *= *w as u64;
	}
	quantum
}

fn quantumi( list: &[i32], item:i32 ) -> u64 {
	let mut quantum = item as u64;
	for w in list {
		quantum *= *w as u64;
	}
	quantum
}

fn find_group_sum( source: &[i32], n: i32 ) -> i32 {
	let mut sum = 0;
	for term in source {
		sum += term;
	}
	sum / n
}

fn find_first_group( source: &[i32], collected: &[i32], current_sum: i32, required_sum: i32, final_group: bool ) -> Option<Vec<i32>> {
	let mut head = source.to_vec();
	let mut tail = Vec::new();

	loop {
		if let Some(item) = head.pop() {
			let new_sum = current_sum + item;
			if new_sum < required_sum {
				let mut h = head.clone();
				h.extend_from_slice(&tail);
				let mut g = collected.to_vec();
				g.extend_from_slice(&[item]);
				if let Some(val) = find_first_group( &h, &g, new_sum, required_sum, final_group ) {
					return Some(val)
				}
			}
			else if new_sum == required_sum {
				if final_group {
					let mut g = collected.to_vec();
					g.extend_from_slice(&[item]);
					return Some(g)
				}
				else {
					let mut h = head.clone();
					h.extend_from_slice(&tail);
					if find_first_group(&h, &[], 0, required_sum, true ).is_some() {
						let mut g = collected.to_vec();
						g.extend_from_slice(&[item]);
						return Some(g)
					}
				}
			}
			tail.insert(0, item);
		}
		else {
			return None
		}
	}
}

fn find_groups_with_length( source: &[i32], collected: &[i32], current_sum: i32, current_prod: u64, required_sum: i32, required_len: usize, mut best_quantum: u64 ) -> Option<(u64,Vec<i32>)> {
	let mut head = source.to_vec();
	let mut tail = Vec::new();

	let mut best_vec : Option<Vec<i32>> = None;

	loop {
		if let Some(item) = head.pop() {
			let new_sum = current_sum + item;
			let new_prod : u64 = current_prod*item as u64;
			if new_prod > best_quantum {
				return None
			}
			if collected.len() + 1 == required_len {
				if new_sum == required_sum && new_prod < best_quantum {
					let mut h = head.clone();
					h.extend_from_slice(&tail);
					if find_first_group(&h, &[], 0, required_sum, true ).is_some() {
						let mut g = collected.to_vec();
						g.extend_from_slice(&[item]);
						return Some((new_prod,g))
					}
				}
				return None
			}
			if new_sum > required_sum {
				return None
			}

			let mut h = head.clone();
			h.extend_from_slice(&tail);
			let mut g = collected.to_vec();
			g.extend_from_slice(&[item]);
			if let Some((q,val)) = find_groups_with_length( &h, &g, new_sum, new_prod, required_sum, required_len, best_quantum ) {
				best_quantum = q;
				best_vec = Some(val);
			}
			tail.insert(0, item);
		}
		else if let Some(v) = best_vec {
			return Some((best_quantum,v))
		}
		else {
			return None
		}
	}
}

fn brute(
		source: &[i32],
		current_sum: i32,
		required_sum: i32,
		current_prod: u64,
		mut best_quantum: u64,
		from: usize,
		depth: usize,
		indexes: &[usize]
	) -> Option<(u64,Vec<usize>)>{
	let t = source.len();
	let mut best : Option<(u64,Vec<usize>)> = None;

	for ix in from..t {
		let s = current_sum + source[ix];
		let p = current_prod * source[ix] as u64;
		if depth == 1 {
			if s == required_sum && p < best_quantum {
				let mut v = source.to_vec();
				for i in indexes.iter().rev() {
					v.remove(*i);
				}
				if find_first_group(&v, &[], 0, required_sum, false ).is_some() {
					best_quantum = p;
					best = Some((p,vec![ix]));
				}
			}
		}
		else if s < required_sum && p < best_quantum {
			let mut ivec = indexes.to_vec();
			ivec.push(ix);
			let result = brute( &source, current_sum + source[ix], required_sum, current_prod * source[ix] as u64, best_quantum, ix+1,depth-1,&ivec);
			if let Some((q,mut v)) = result {
				best_quantum = q;
				v.push(ix);
				best = Some((q,v));
			}
		}
	}
	best
}

fn ix_to_value( source: &[i32], indices: &[usize] ) -> Vec<i32> {
	indices.iter().map( |&i| source[i] ).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_find_avg_group_weight() {
		let v = vec![1,2,3,4,5,7,8,9,10,11];
		assert_eq!(20,find_group_sum(&v,3));
	}

	#[test]
	fn test_find_first_group() {
		let v = vec![1,2,3,4,5,7,8,9,10,11];
		assert_eq!(Some(vec![11,9]),find_first_group(&v, &[], 0, find_group_sum(&v,3), true));
	}

	#[test]
	fn test_ix_lookup() {
		let v = vec![1,2,3,4,5,7,8,9,10,11];
		let conv = ix_to_value(&v, &[9,7]);
		assert_eq!(vec![11,9],conv);
	}

	#[test]
	fn test_find_brute() {
		let v = vec![1,2,3,4,5,7,8,9,10,11];
		let r = brute(&v,0,20,1,std::u64::MAX,0,2,&[]);
		assert_eq!(Some((99,vec![9,7])),r);
	}

	#[test]
	fn test_find_best_group_when_size_is_known() {
		let v = vec![1,2,3,4,5,7,8,9,10,11];
		let r = find_groups_with_length(&v,&[],0,1,20,2,std::u64::MAX);
		assert_eq!(Some((99,vec![9,11])),r);
	}

	#[test]
	fn test_part1() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let group_sum = find_group_sum(&v,4);
		//assert_eq!(520,group_sum);

		let g1 = find_first_group(&v, &[], 0, group_sum, true);
		assert!(g1.is_some());

		if let Some(r) = g1 {

			let q = quantum(&r);
			assert_eq!(67_601_337_186,q);
			// 23 538 056 666 also too high
			assert_eq!(7, r.len());

			let ixr = brute(&v,0,group_sum,1,q,0,r.len()-1,&[]);
			assert!(ixr.is_some());
			assert_eq!(Some((11846773891,vec![28, 27, 26, 24, 22, 0])),ixr);

			if let Some((q2,ix)) = ixr {
				assert_eq!(vec![109, 107, 103, 101, 97, 2, 1],ix_to_value(&v,&ix));

			}			assert_eq!(vec![0],r);

		}
	}
}



// Group 1;             Group 2; Group 3
// 11 9       (QE= 99); 10 8 2;  7 5 4 3 1
// 10 9 1     (QE= 90); 11 7 2;  8 5 4 3
// 10 8 2     (QE=160); 11 9;    7 5 4 3 1
// 10 7 3     (QE=210); 11 9;    8 5 4 2 1
// 10 5 4 1   (QE=200); 11 9;    8 7 3 2
// 10 5 3 2   (QE=300); 11 9;    8 7 4 1
// 10 4 3 2 1 (QE=240); 11 9;    8 7 5
// 9 8 3      (QE=216); 11 7 2;  10 5 4 1
// 9 7 4      (QE=252); 11 8 1;  10 5 3 2
// 9 5 4 2    (QE=360); 11 8 1;  10 7 3
// 8 7 5      (QE=280); 11 9;    10 4 3 2 1
// 8 5 4 3    (QE=480); 11 9;    10 7 2 1
// 7 5 4 3 1  (QE=420); 11 9;    10 8 2

// Of these, although 10 9 1 has the smallest quantum entanglement (90), the configuration with only two packages, 11 9,
// in the passenger compartment gives Santa the most legroom and wins. In this situation, the quantum entanglement
// for the ideal configuration is therefore 99. Had there been two configurations with only two packages in the first group,
// the one with the smaller quantum entanglement would be chosen.

// What is the quantum entanglement of the first group of packages in the ideal configuration?

