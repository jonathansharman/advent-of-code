use std::collections::HashSet;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 3396);
aoc::test::test_part!(test2, part2, 2044);

fn get_drop() -> HashSet<[i32; 3]> {
	input!()
		.lines()
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_vec()
				.try_into()
				.unwrap()
		})
		.collect()
}

pub fn part1() -> usize {
	let drop = get_drop();
	drop.iter()
		.map(|&[x, y, z]| {
			!drop.contains(&[x + 1, y, z]) as usize
				+ !drop.contains(&[x - 1, y, z]) as usize
				+ !drop.contains(&[x, y + 1, z]) as usize
				+ !drop.contains(&[x, y - 1, z]) as usize
				+ !drop.contains(&[x, y, z + 1]) as usize
				+ !drop.contains(&[x, y, z - 1]) as usize
		})
		.sum()
}

fn get_neighbors([x, y, z]: [i32; 3]) -> [[i32; 3]; 6] {
	[
		[x + 1, y, z],
		[x - 1, y, z],
		[x, y + 1, z],
		[x, y - 1, z],
		[x, y, z + 1],
		[x, y, z - 1],
	]
}

pub fn part2() -> usize {
	let drop = get_drop();
	// The cube to the left of the leftmost droplet cube is exterior.
	let &[x, y, z] = drop.iter().min_by(|c1, c2| c1[0].cmp(&c2[0])).unwrap();
	let mut exterior = HashSet::new();
	let is_by_drop = |[x, y, z]: [i32; 3]| {
		drop.contains(&[x + 1, y, z])
			|| drop.contains(&[x - 1, y, z])
			|| drop.contains(&[x, y + 1, z])
			|| drop.contains(&[x, y - 1, z])
			|| drop.contains(&[x, y, z + 1])
			|| drop.contains(&[x, y, z - 1])
	};
	let mut queue = vec![[x - 1, y, z]];
	while let Some([x, y, z]) = queue.pop() {
		if !drop.contains(&[x, y, z]) {
			if (is_by_drop([x, y, z])) && exterior.insert([x, y, z]) {
				queue.extend(get_neighbors([x, y, z]));
			} else {
				// This case allows us to turn corners.
				for neighbor in get_neighbors([x, y, z]) {
					if !exterior.contains(&neighbor) && is_by_drop(neighbor) {
						queue.push(neighbor);
					}
				}
			}
		}
	}
	exterior
		.iter()
		.map(|&[x, y, z]| {
			drop.contains(&[x + 1, y, z]) as usize
				+ drop.contains(&[x - 1, y, z]) as usize
				+ drop.contains(&[x, y + 1, z]) as usize
				+ drop.contains(&[x, y - 1, z]) as usize
				+ drop.contains(&[x, y, z + 1]) as usize
				+ drop.contains(&[x, y, z - 1]) as usize
		})
		.sum()
}
