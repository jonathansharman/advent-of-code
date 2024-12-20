use std::collections::{HashMap, HashSet};

use itertools::Itertools;

aoc::test::test_part!(test1, part1, 269);
aoc::test::test_part!(test2, part2, 1380);

const INPUT: &str = include_str!("input/17.txt");

type Point3 = [i32; 3];

pub fn part1() -> usize {
	let mut state: HashSet<Point3> = INPUT
		.lines()
		.enumerate()
		.flat_map(|(i, line)| {
			line.chars()
				.enumerate()
				.filter_map(|(j, c)| {
					(c == '#').then_some([i as i32, j as i32, 0])
				})
				.collect::<Vec<_>>()
		})
		.collect();
	for _ in 0..6 {
		let neighbors = state.iter().fold(
			HashMap::new(),
			|mut acc: HashMap<Point3, usize>, [i, j, k]| {
				(-1..=1)
					.cartesian_product(-1..=1)
					.cartesian_product(-1..=1)
					.for_each(|((di, dj), dk)| {
						// Ensure that the point and all its neighboring points
						// exist in the neighbors map, but only increment the
						// neighboring points.
						let coords = [i + di, j + dj, k + dk];
						let diff = (di != 0 || dj != 0 || dk != 0) as usize;
						*acc.entry(coords).or_default() += diff;
					});
				acc
			},
		);
		state = neighbors
			.into_iter()
			.filter_map(|(coords, count)| {
				let alive = state.contains(&coords);
				((alive && count == 2) || count == 3).then_some(coords)
			})
			.collect();
	}
	state.len()
}

type Point4 = [i32; 4];

pub fn part2() -> usize {
	let mut state: HashSet<Point4> = INPUT
		.lines()
		.enumerate()
		.flat_map(|(i, line)| {
			line.chars()
				.enumerate()
				.filter_map(|(j, c)| {
					(c == '#').then_some([i as i32, j as i32, 0, 0])
				})
				.collect::<Vec<_>>()
		})
		.collect();
	for _ in 0..6 {
		let neighbors = state.iter().fold(
			HashMap::new(),
			|mut acc: HashMap<Point4, usize>, [i, j, k, l]| {
				(-1..=1)
					.cartesian_product(-1..=1)
					.cartesian_product(-1..=1)
					.cartesian_product(-1..=1)
					.for_each(|(((di, dj), dk), dl)| {
						// Ensure that the point and all its neighboring points
						// exist in the neighbors map, but only increment the
						// neighboring points.
						let coords = [i + di, j + dj, k + dk, l + dl];
						let diff =
							(di != 0 || dj != 0 || dk != 0 || dl != 0) as usize;
						*acc.entry(coords).or_default() += diff;
					});
				acc
			},
		);
		state = neighbors
			.into_iter()
			.filter_map(|(coords, count)| {
				let alive = state.contains(&coords);
				((alive && count == 2) || count == 3).then_some(coords)
			})
			.collect();
	}
	state.len()
}
