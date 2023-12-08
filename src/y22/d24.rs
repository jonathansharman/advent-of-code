use std::collections::{HashSet, VecDeque};

use num::integer::lcm;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 334);
crate::test::test_part!(test2, part2, ?);

enum Tile {
	Wall,
	Up,
	Down,
	Left,
	Right,
	Floor,
}

struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
	fn load() -> Tiles {
		Tiles(
			read_lines("input/2022/24.txt")
				.map(|line| {
					line.chars()
						.map(|c| match c {
							'^' => Tile::Up,
							'v' => Tile::Down,
							'<' => Tile::Left,
							'>' => Tile::Right,
							'#' => Tile::Wall,
							_ => Tile::Floor,
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
		let n = self.height() - 2;
		let m = self.width() - 2;
		(0..self.period())
			.map(|time| {
				let mut t = vec![vec![true; self.width()]; self.height()];
				for (i, row) in self.0.iter().enumerate() {
					for (j, tile) in row.iter().enumerate() {
						match tile {
							Tile::Wall => t[i][j] = false,
							Tile::Up => {
								t[(i + n - 1 - (time % n)) % n + 1][j] = false
							}
							Tile::Down => t[(i + time - 1) % n + 1][j] = false,
							Tile::Left => {
								t[i][(j + m - 1 - (time % m)) % m + 1] = false
							}
							Tile::Right => t[i][(j + time - 1) % m + 1] = false,
							Tile::Floor => {}
						}
					}
				}
				t
			})
			.collect::<Vec<_>>()
	}
}

pub fn part1() -> usize {
	let tiles = Tiles::load();
	let open = tiles.get_open();
	let mut visited = HashSet::new();

	let mut queue = VecDeque::new();
	queue.push_back((0, 0, 1));
	while let Some((t, i, j)) = queue.pop_front() {
		if visited.contains(&(t, i, j)) {
			continue;
		}
		// Check if we're at the exit.
		if i == tiles.height() - 1 && j == tiles.width() - 2 {
			return t;
		}
		// Mark these spatio-temporal coordinates as visited.
		visited.insert((t, i, j));
		// Get the open spaces at the next minute.
		let next_open = &open[(t + 1) % tiles.period()];
		// Wait.
		if next_open[i][j] {
			queue.push_back((t + 1, i, j));
		}
		// Go up.
		if i > 0 && next_open[i - 1][j] {
			queue.push_back((t + 1, i - 1, j));
		}
		// Go down.
		if i < tiles.height() - 1 && next_open[i + 1][j] {
			queue.push_back((t + 1, i + 1, j));
		}
		// Go left.
		if j > 0 && next_open[i][j - 1] {
			queue.push_back((t + 1, i, j - 1));
		}
		// Go right.
		if j < tiles.width() - 1 && next_open[i][j + 1] {
			queue.push_back((t + 1, i, j + 1));
		}
	}
	panic!("end unreachable");
}

pub fn part2() -> usize {
	0
}
