pub fn attac(damage: i32, armor: i32) -> i32 {
	let hit = damage - armor;
	if hit < 1 {
		1
	} else {
		hit
	}
}

pub fn battle(damage: i32, armor: i32) -> bool {
	let mut player_hp = 100i32;
	let mut boss_hp = 103i32;
	let boss_damage = 9i32;
	let boss_armor = 2i32;

	loop {
		boss_hp -= attac(damage, boss_armor);

		if boss_hp <= 0 {
			return true;
		}

		player_hp -= attac(boss_damage, armor);

		if player_hp <= 0 {
			return false;
		}
	}
}

type Item = (i32, i32, i32);

fn get_weapons() -> Vec<Item> {
	vec![
		(8, 4, 0),
		(10, 5, 0),
		(25, 6, 0),
		(40, 7, 0),
		(74, 8, 0),
	]
}

fn get_armor() -> Vec<Item> {
	vec![
		(0, 0, 0), // armor is optional
		(13, 0, 1),
		(31, 0, 2),
		(53, 0, 3),
		(75, 0, 4),
		(102, 0, 5),
	]
}

fn get_rings() -> Vec<Item> {
	vec![
		(0, 0, 0), // can have zero, one
		(0, 0, 0), // or two rings
		(25, 1, 0),
		(50, 2, 0),
		(100, 3, 0),
		(20, 0, 1),
		(40, 0, 2),
		(80, 0, 3),
	]
}

fn get_gear(evil_shop: bool, weapons: &Vec<Item>, armor: &Vec<Item>, rings: &Vec<Item>) -> i32 {
	let mut boundary = if evil_shop { 0 } else { 300 };

	for w in 0..weapons.len() {
		for a in 0..armor.len() {
			for r1 in 0..rings.len() {
				for r2 in (r1 + 1)..rings.len() {
					let cost = weapons[w].0 + armor[a].0 + rings[r1].0 + rings[r2].0;
					if evil_shop && cost <= boundary || !evil_shop && cost >= boundary {
						continue;
					}
					let battle = battle(
						weapons[w].1 + rings[r1].1 + rings[r2].1,
						armor[a].2 + rings[r1].2 + rings[r2].2,
					);
					if battle && !evil_shop || !battle && evil_shop {
						boundary = cost;
					}
				}
			}
		}
	}
	return boundary
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_attac() {
		assert_eq!(5, attac(8, 3));
		assert_eq!(1, attac(8, 500));
	}

	#[test]
	fn test_battle() {
		assert!(battle(100, 3));
		assert!(!battle(0, 0));
		assert!(!battle(3,8));
	}

	#[test]
	fn test_getgear_something_lose() {
		let weapons = vec![(0,0,0)];
		let armor = vec![(0,0,0)];
		let rings = vec![(0,0,0),(0,0,0),(100,3,0),(300,30,0)];

		assert_eq!(100, get_gear(true,&weapons,&armor,&rings));
	}

	#[test]
	fn test_getgear_nothing_lose() {
		let weapons = vec![(0,0,0)];
		let armor = vec![(0,0,0)];
		let rings = vec![(0,0,0),(0,0,0)];

		assert_eq!(0, get_gear(true,&weapons,&armor,&rings));
	}

	#[test]
	fn test_getgear() {
		let weapons = get_weapons();
		let armor = get_armor();
		let rings = get_rings();

		assert_eq!(121, get_gear(false,&weapons, &armor, &rings));
		assert_eq!(201, get_gear(true, &weapons, &armor, &rings));
	}
}
