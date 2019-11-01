
fn find_group_sum( source: &[i32], n: u8 ) -> i32 {
	let mut sum = 0;
	for term in source {
		sum += term;
	}
	sum / n as i32
}

fn find_first_group( source: &[i32], collected: &[i32], current_sum: i32, required_sum: i32, extra_group_count: u8 ) -> Option<Vec<i32>> {
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
				if let Some(val) = find_first_group( &h, &g, new_sum, required_sum, extra_group_count ) {
					return Some(val)
				}
			}
			else if new_sum == required_sum {
				if extra_group_count == 0 {
					let mut g = collected.to_vec();
					g.extend_from_slice(&[item]);
					return Some(g)
				}
				else {
					let mut h = head.clone();
					h.extend_from_slice(&tail);
					if find_first_group(&h, &[], 0, required_sum, extra_group_count - 1 ).is_some() {
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

fn brute(
		source: &[i32],
		current_sum: i32,
		required_sum: i32,
		current_prod: u64,
		mut best_quantum: u64,
		from: usize,
		size: usize,
		groups: u8,
		indexes: &[usize]
	) -> Option<(u64,Vec<usize>)> {
	let t = source.len();
	let mut best : Option<(u64,Vec<usize>)> = None;

	for ix in from..t {
		let s = current_sum + source[ix];
		let p = current_prod * source[ix] as u64;
		if size == 1 {
			if s == required_sum && p < best_quantum {
				let mut v = source.to_vec();
				for i in indexes.iter().rev() {
					v.remove(*i);
				}
				if find_first_group(&v, &[], 0, required_sum, groups - 2 ).is_some() {
					best_quantum = p;
					best = Some((p,vec![ix]));
				}
			}
		}
		else if s < required_sum && p < best_quantum {
			let mut ivec = indexes.to_vec();
			ivec.push(ix);
			let result = brute( &source, current_sum + source[ix], required_sum, current_prod * source[ix] as u64, best_quantum, ix+1,size-1,groups,&ivec);
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

fn find_with_size( source: &[i32], size: usize, groups: u8 ) ->Option<(u64,Vec<usize>)>{
	let group_sum = find_group_sum(&source,groups);
	brute(&source,0,group_sum,1,std::u64::MAX,0,size,groups,&[])
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
		assert_eq!(Some(vec![11,9]),find_first_group(&v, &[], 0, find_group_sum(&v,3), 0));
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
		let r = brute(&v,0,20,1,std::u64::MAX,0,2,3,&[]);
		assert_eq!(Some((99,vec![9,7])),r);
	}

	#[test]
	fn test_can_find_len_4() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let ixr =find_with_size(&v,4,3);
		assert_eq!(None,ixr);
	}

	#[test]
	fn test_can_find_len_5() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let ixr =find_with_size(&v,5,3);
		assert_eq!(None,ixr);
	}

	#[test]
	fn test_can_find_len_6() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let ixr =find_with_size(&v,6,3);
		assert!(ixr.is_some());
		assert_eq!(Some((11846773891,vec![28, 27, 26, 24, 22, 0])),ixr);
	}

	#[test]
	fn test_can_find_len_3_size4() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let ixr =find_with_size(&v,3,4);
		assert_eq!(None,ixr);
	}

	#[test]
	fn test_can_find_len_4_size4() {
		let v = vec![1i32, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
		let ixr =find_with_size(&v,4,4);
		assert!(ixr.is_some());
		assert_eq!(Some((80393059, vec![28, 27, 26, 16])),ixr);
	}
}
