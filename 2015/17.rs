
fn find( sum:u32, target:u32, arr:&[u32] ) -> u32 {
	let mut num_found = 0u32;
	for i in 0..arr.len() {
		let test = sum+arr[i];
		if test > target {
			break;
		}
		else if test == target {
			num_found += 1;
		}
		else if test < target {
			num_found += find(test,target,&arr[i+1..]);
		}
	}
	num_found
}

fn find_min( sum:u32, target:u32, depth:u32, min_found:u32, arr:&[u32] ) -> (u32,u32) {
	let mut num_found = 0u32;
	let mut new_min = min_found;
	for i in 0..arr.len() {
		let test = sum+arr[i];
		let new_depth = depth + 1;
		if test > target {
			break;
		}
		else if test == target {
			if new_depth < new_min {
				new_min = new_depth;
				num_found = 1;
			}
			else if new_depth == new_min {
				num_found += 1;
			}
		}
		else if test < target {
			let (tmpmin,tmpfound) = find_min(test,target,new_depth,new_min,&arr[i+1..]);
			if tmpmin == new_min {
				num_found += tmpfound;
			}
			if tmpmin < new_min {
				new_min = tmpmin;
				num_found = tmpfound;
			}
		}
	}
	(new_min,num_found)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_validate() {
		assert_eq!(1,find(0,25,&vec![5,20]));
		assert_eq!(4,find(0,9,&vec![3,3,3,6]));
		assert_eq!(4,find(0,25,&vec![5,5,10,15,20]));
	}

	#[test]
	fn test_validate2() {
		assert_eq!((2,1),find_min(0,25,0,2,&vec![5,20]));
		assert_eq!((2,3),find_min(0,9,0,4,&vec![3,3,3,6]));
		assert_eq!((2,3),find_min(0,25,0,5,&vec![5,5,10,15,20]));
	}

	#[test]
	fn test_crunch(){
		let arr = vec![1, 1, 3, 3, 3, 5, 11, 11, 15, 19, 26, 28, 30, 31, 32, 32, 36, 36, 46, 47];
		assert_eq!(4372,find(0,150,&arr));
	}

	#[test]
	fn test_crunch2(){
		let arr = vec![1, 1, 3, 3, 3, 5, 11, 11, 15, 19, 26, 28, 30, 31, 32, 32, 36, 36, 46, 47];
		assert_eq!((4,4),find_min(0,150,0,arr.len() as u32,&arr));
	}
}
