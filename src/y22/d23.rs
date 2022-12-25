use std::collections::{HashMap, HashSet};

use itertools::{Itertools, MinMaxResult};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 4068);
crate::test::test_part!(test2, part2, 968);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
	row: i64,
	col: i64,
}

fn get_elves() -> HashSet<Coords> {
	let mut elves = HashSet::new();
	for (row, line) in read_lines("input/2022/23.txt").enumerate() {
		for (col, c) in line.chars().enumerate() {
			if c == '#' {
				elves.insert(Coords {
					row: row as i64,
					col: col as i64,
				});
			}
		}
	}
	elves
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
	North,
	South,
	West,
	East,
}

fn neighbor_to(coords: Coords, direction: Direction) -> Coords {
	match direction {
		Direction::North => Coords {
			row: coords.row - 1,
			col: coords.col,
		},
		Direction::South => Coords {
			row: coords.row + 1,
			col: coords.col,
		},
		Direction::West => Coords {
			row: coords.row,
			col: coords.col - 1,
		},
		Direction::East => Coords {
			row: coords.row,
			col: coords.col + 1,
		},
	}
}

fn all_open(elves: &HashSet<Coords>, coords: Coords) -> bool {
	for row in coords.row - 1..=coords.row + 1 {
		for col in coords.col - 1..=coords.col + 1 {
			let neighbor = Coords { row, col };
			if neighbor != coords && elves.contains(&neighbor) {
				return false;
			}
		}
	}
	true
}

fn is_open_to(
	elves: &HashSet<Coords>,
	coords: Coords,
	direction: Direction,
) -> bool {
	match direction {
		Direction::North => {
			!elves.contains(&Coords {
				row: coords.row - 1,
				col: coords.col,
			}) && !elves.contains(&Coords {
				row: coords.row - 1,
				col: coords.col + 1,
			}) && !elves.contains(&Coords {
				row: coords.row - 1,
				col: coords.col - 1,
			})
		}
		Direction::South => {
			!elves.contains(&Coords {
				row: coords.row + 1,
				col: coords.col,
			}) && !elves.contains(&Coords {
				row: coords.row + 1,
				col: coords.col + 1,
			}) && !elves.contains(&Coords {
				row: coords.row + 1,
				col: coords.col - 1,
			})
		}
		Direction::East => {
			!elves.contains(&Coords {
				row: coords.row - 1,
				col: coords.col + 1,
			}) && !elves.contains(&Coords {
				row: coords.row,
				col: coords.col + 1,
			}) && !elves.contains(&Coords {
				row: coords.row + 1,
				col: coords.col + 1,
			})
		}
		Direction::West => {
			!elves.contains(&Coords {
				row: coords.row - 1,
				col: coords.col - 1,
			}) && !elves.contains(&Coords {
				row: coords.row,
				col: coords.col - 1,
			}) && !elves.contains(&Coords {
				row: coords.row + 1,
				col: coords.col - 1,
			})
		}
	}
}

// Returns the 0-based final round simulated.
fn simulate(elves: &mut HashSet<Coords>, rounds: Option<usize>) -> usize {
	let directions = [
		Direction::North,
		Direction::South,
		Direction::West,
		Direction::East,
	];
	let mut round = 0;
	loop {
		let mut proposals = HashMap::new();
		let mut stable = true;
		for &coords in elves.iter() {
			if all_open(elves, coords) {
				continue;
			}
			stable = false;
			for i in 0..directions.len() {
				let to = directions[(round + i) % directions.len()];
				if is_open_to(elves, coords, to) {
					let target = neighbor_to(coords, to);
					proposals
						.entry(target)
						// If there's already a proposal to move here, no one
						// gets to move there.
						.and_modify(|from| *from = None)
						// If the spot's open, propose filling it from this
						// direction.
						.or_insert(Some(coords));
					break;
				}
			}
		}
		for (target, from) in proposals {
			if let Some(from) = from {
				elves.insert(target);
				elves.remove(&from);
			}
		}
		get_empty_count(elves);
		round += 1;
		if stable {
			return round;
		}
		if let Some(rounds) = rounds {
			if round == rounds {
				return round;
			}
		}
	}
}

fn get_empty_count(elves: &HashSet<Coords>) -> usize {
	let (row0, row1) = match elves.iter().map(|coords| coords.row).minmax() {
		MinMaxResult::NoElements => panic!("No elves"),
		MinMaxResult::OneElement(row) => (row, row),
		MinMaxResult::MinMax(row0, row1) => (row0, row1),
	};
	let (col0, col1) = match elves.iter().map(|coords| coords.col).minmax() {
		MinMaxResult::NoElements => panic!("No elves"),
		MinMaxResult::OneElement(col) => (col, col),
		MinMaxResult::MinMax(col0, col1) => (col0, col1),
	};
	let mut empty_count = 0;
	for row in row0..=row1 {
		for col in col0..=col1 {
			if !elves.contains(&Coords { row, col }) {
				empty_count += 1;
			}
		}
	}
	empty_count
}

pub fn part1() -> usize {
	let mut elves = get_elves();
	simulate(&mut elves, Some(10));
	get_empty_count(&elves)
}

pub fn part2() -> usize {
	let mut elves = get_elves();
	simulate(&mut elves, None)
}
