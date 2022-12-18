use std::collections::HashSet;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 3396);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let cubes: HashSet<[i32; 3]> = read_lines("input/2022/18.txt")
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_vec()
				.try_into()
				.unwrap()
		})
		.collect();
	cubes
		.iter()
		.map(|&[x, y, z]| {
			!cubes.contains(&[x + 1, y, z]) as usize
				+ !cubes.contains(&[x - 1, y, z]) as usize
				+ !cubes.contains(&[x, y + 1, z]) as usize
				+ !cubes.contains(&[x, y - 1, z]) as usize
				+ !cubes.contains(&[x, y, z + 1]) as usize
				+ !cubes.contains(&[x, y, z - 1]) as usize
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
