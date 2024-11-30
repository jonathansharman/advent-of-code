use std::{
	cmp::Ordering,
	collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

use crate::{io::read_lines, neighbors};

aoc::test::test_part!(test1, part1, 3733);
aoc::test::test_part!(test2, part2, 617729401414635);

fn read_grid() -> Vec<Vec<bool>> {
	read_lines("input/21.txt")
		.map(|line| line.chars().map(|c| c == '.' || c == 'S').collect())
		.collect()
}

const TOTAL_STEPS_1: usize = 64;
const PARITY_1: usize = TOTAL_STEPS_1 % 2;

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
		if d % 2 == PARITY_1 {
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
const PARITY_2: usize = TOTAL_STEPS_2 % 2;

pub fn part2() -> usize {
	let grid = read_grid();
	let diameter = grid.len();
	let radius = diameter / 2;
	let critical_points = (0..=2)
		.cartesian_product(0..=2)
		.map(|(i, j)| {
			let coords = (i * radius, j * radius);
			let degrees_of_freedom = i.abs_diff(1) + j.abs_diff(1);
			let zone_distance = degrees_of_freedom * (radius + 1);
			let distances = get_distances(&grid, coords);
			CriticalPoint {
				zone_distance,
				degrees_of_freedom,
				distances,
			}
		})
		.collect::<Vec<_>>();
	let mut count = 0;
	for p in critical_points {
		for d in p.distances.into_values() {
			let remaining = match TOTAL_STEPS_2.cmp(&(d + p.zone_distance)) {
				Ordering::Less => continue,
				Ordering::Equal => 0,
				Ordering::Greater => TOTAL_STEPS_2 - d - p.zone_distance,
			};
			// Compute the tile distance from the first tile in this critical
			// point's zone to farthest reachable tile.
			let n = remaining / diameter;
			// Check whether this point has correct parity in the first tile.
			let correct_parity = (p.zone_distance + d) % 2 == PARITY_2;
			match p.degrees_of_freedom {
				0 => {
					if correct_parity {
						count += 1;
					}
				}
				1 => {
					// Assuming the tile diameter is odd, parity alternates per
					// tile, and approximately half the reachable tiles will
					// have the correct parity. If the total distance to the
					// point in the first zone has correct parity, round up
					// instead of down if the number of reachable tiles is odd.
					count += (n + 1 + correct_parity as usize) / 2;
				}
				2 => {
					// Parity alternates per diagonal row of tiles, starting
					// with the correct parity if the point in the first zone
					// does. These summations work because of mathematics. 🧠
					if correct_parity {
						count += (n / 2 + 1).pow(2);
					} else {
						count += ((n + 1) / 2) * ((n + 1) / 2 + 1);
					}
				}
				_ => unreachable!(),
			}
		}
	}
	count
}

/// Info about one of the nine "important" points on the grid: the center and
/// the center +/- the radius on each axis.
struct CriticalPoint {
	/// Number of directions in this point's zone.
	degrees_of_freedom: usize,
	/// Distance from the start to this point's zone.
	zone_distance: usize,
	/// Distances from the point to every other point within the primary tile.
	distances: HashMap<(usize, usize), usize>,
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
