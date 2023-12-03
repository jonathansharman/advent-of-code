use std::collections::{HashSet, VecDeque};

use num::integer::lcm;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 334);
crate::test::test_part!(test2, part2, ?);

enum Blizzard {
	None,
	Up,
	Down,
	Left,
	Right,
}

struct Blizzards(Vec<Vec<Blizzard>>);

impl Blizzards {
	fn load() -> Blizzards {
		let lines = read_lines("input/2022/24.txt").collect::<Vec<_>>();
		Blizzards(
			lines[1..lines.len() - 1]
				.iter()
				.map(|line| {
					line.chars()
						.filter_map(|c| match c {
							'^' => Some(Blizzard::Up),
							'v' => Some(Blizzard::Down),
							'<' => Some(Blizzard::Left),
							'>' => Some(Blizzard::Right),
							'#' => None,
							_ => Some(Blizzard::None),
						})
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>(),
		)
	}

	fn height(&self) -> usize {
		self.0.len()
	}

	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn period(&self) -> usize {
		lcm(self.width(), self.height())
	}

	// Minute -> row -> column -> "is this space open at that time"
	fn get_open(&self) -> Vec<Vec<Vec<bool>>> {
		let mut result = Vec::new();
		for time in 0..self.period() {
			let mut t = vec![vec![true; self.width()]; self.height()];
			for (i, row) in self.0.iter().enumerate() {
				for (j, bliz) in row.iter().enumerate() {
					match bliz {
						Blizzard::None => {}
						Blizzard::Up => {
							t[(i + self.height() - (time % self.height()))
								% self.height()][j] = false
						}
						Blizzard::Down => {
							t[(i + time) % self.height()][j] = false
						}
						Blizzard::Left => {
							t[i][(j + self.width() - (time % self.width()))
								% self.width()] = false
						}
						Blizzard::Right => {
							t[i][(j + time) % self.width()] = false
						}
					}
				}
			}
			result.push(t);
		}
		result
	}
}

pub fn part1() -> usize {
	let blizzards = Blizzards::load();
	let open = blizzards.get_open();
	let mut visited = HashSet::new();

	let mut queue = VecDeque::new();
	// TODO: This assumes the solution requires moving into the blizzards at
	// minute 1, but it could potentially require moving later.
	queue.push_back((1, 0, 0));
	while let Some((t, i, j)) = queue.pop_front() {
		if visited.contains(&(t, i, j)) {
			continue;
		}
		// Check if we're one away from the exit.
		if i == blizzards.height() - 1 && j == blizzards.width() - 1 {
			return t + 1;
		}
		// Mark these spatio-temporal coordinates as visited.
		visited.insert((t, i, j));
		// Get the open spaces at the next minute.
		let next_open = &open[(t + 1) % blizzards.period()];
		// Wait.
		if next_open[i][j] {
			queue.push_back((t + 1, i, j));
		}
		// Go up.
		if i > 0 && next_open[i - 1][j] {
			queue.push_back((t + 1, i - 1, j));
		}
		// Go down.
		if i < blizzards.height() - 1 && next_open[i + 1][j] {
			queue.push_back((t + 1, i + 1, j));
		}
		// Go left.
		if j > 0 && next_open[i][j - 1] {
			queue.push_back((t + 1, i, j - 1));
		}
		// Go right.
		if j < blizzards.width() - 1 && next_open[i][j + 1] {
			queue.push_back((t + 1, i, j + 1));
		}
	}
	panic!("end unreachable");
}

pub fn part2() -> usize {
	0
}
