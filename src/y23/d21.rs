use std::{
	cmp::Ordering,
	collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, 3733);
crate::test::test_part!(test2, part2, ?); // 617687757554990 is too low

fn read_grid() -> Vec<Vec<bool>> {
	read_lines("input/2023/21.txt")
		.map(|line| line.chars().map(|c| c == '.').collect())
		.collect()
}

const TOTAL_STEPS_1: usize = 64;

pub fn part1() -> usize {
	let grid = read_grid();
	// All available inputs are square with S in the center. ¯\_(ツ)_/¯
	let radius = grid.len() / 2;
	let start = (radius, radius);
	let mut count = 0;
	let mut queue = VecDeque::from([(start, 0)]);
	let mut visited = HashSet::new();
	while let Some(((i, j), d)) = queue.pop_front() {
		if !visited.insert((i, j)) {
			continue;
		}
		if d % 2 == 0 {
			count += 1;
		}
		for n in neighbors::four(grid.len(), grid[0].len(), i, j) {
			if grid[n.0][n.1] && d < TOTAL_STEPS_1 {
				queue.push_back((n, d + 1));
			}
		}
	}
	count
}

const TOTAL_STEPS_2: usize = 26_501_365;

pub fn part2() -> usize {
	let grid = read_grid();
	let diameter = grid.len();
	let radius = diameter / 2;
	let reachable_counts: HashMap<(usize, usize), HashMap<usize, usize>> =
		[0, radius, 2 * radius]
			.into_iter()
			.cartesian_product([0, radius, 2 * radius])
			.map(|start| {
				let distance_counts: HashMap<usize, usize> = get_distances(
					&grid, start,
				)
				.into_values()
				.fold(HashMap::new(), |mut acc, d| {
					*acc.entry(d).or_default() += 1;
					acc
				});
				(start, distance_counts)
			})
			.collect();
	// TODO: It should be possible to just iterate over the most-distant
	// reachable tiles and compute the contributions of the interior tiles
	// analytically. (There would be two cases for interior tiles, depending on
	// tile coordinate parity.)
	let max_tile_idx = TOTAL_STEPS_2 as i64 / diameter as i64;
	let mut count = 0;
	let reachable_even = reachable_counts[&(0, 0)]
		.iter()
		.filter_map(|(&d, count)| if d % 2 == 1 { Some(count) } else { None })
		.sum::<usize>();
	let reachable_odd = reachable_counts[&(0, 0)]
		.iter()
		.filter_map(|(&d, count)| if d % 2 == 0 { Some(count) } else { None })
		.sum::<usize>();
	for i in -max_tile_idx..=max_tile_idx {
		println!("{i}");
		for j in -max_tile_idx..=max_tile_idx {
			let tile_distance = i.abs() + j.abs();
			match tile_distance.cmp(&max_tile_idx) {
				Ordering::Less => {
					count += if tile_distance % 2 == 0 {
						reachable_even
					} else {
						reachable_odd
					};
					continue;
				}
				Ordering::Greater => continue,
				Ordering::Equal => {}
			}
			let i_distance_to_start = if i == 0 {
				0
			} else {
				diameter * (i.abs() - 1) as usize + radius
			};
			let j_distance_to_start = if j == 0 {
				0
			} else {
				diameter * (j.abs() - 1) as usize + radius
			};
			let distance_to_start = i_distance_to_start + j_distance_to_start;
			let remaining_distance = TOTAL_STEPS_2 - distance_to_start;
			let parity = remaining_distance % 2;
			let i_start = (i.signum() + 1) as usize * radius;
			let j_start = (j.signum() + 1) as usize * radius;
			count += reachable_counts[&(i_start, j_start)]
				.iter()
				.filter_map(|(&d, count)| {
					if d <= remaining_distance && d % 2 == parity {
						Some(count)
					} else {
						None
					}
				})
				.sum::<usize>();
		}
	}
	count
}

fn get_distances(
	grid: &[Vec<bool>],
	start: (usize, usize),
) -> HashMap<(usize, usize), usize> {
	let mut queue = VecDeque::from([(start, 0)]);
	let mut distance_counts = HashMap::new();
	while let Some(((i, j), d)) = queue.pop_front() {
		if let Entry::Vacant(entry) = distance_counts.entry((i, j)) {
			entry.insert(d);
		} else {
			continue;
		}
		for n in neighbors::four(grid.len(), grid[0].len(), i, j) {
			if grid[n.0][n.1] {
				queue.push_back((n, d + 1));
			}
		}
	}
	distance_counts
}
