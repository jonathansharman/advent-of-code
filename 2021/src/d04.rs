use aoc::io::read_lines;

use itertools::Itertools;

aoc::test::test_part!(test1, part1, 38594);
aoc::test::test_part!(test2, part2, 21184);

const SIDE_LENGTH: usize = 5;

struct Board {
	spaces: [Option<i64>; SIDE_LENGTH * SIDE_LENGTH],
	score: Option<i64>,
}

impl Board {
	fn row(&self, idx: usize) -> impl Iterator<Item = &Option<i64>> {
		self.spaces.iter().skip(idx * SIDE_LENGTH).take(SIDE_LENGTH)
	}

	fn col(&self, idx: usize) -> impl Iterator<Item = &Option<i64>> {
		self.spaces.iter().skip(idx).step_by(SIDE_LENGTH)
	}

	fn tick(&mut self, n: i64) {
		for idx in 0..self.spaces.len() {
			if self.spaces[idx] == Some(n) {
				self.spaces[idx] = None;
				let (row_idx, col_idx) = (idx / SIDE_LENGTH, idx % SIDE_LENGTH);
				if self.row(row_idx).all(Option::is_none)
					|| self.col(col_idx).all(Option::is_none)
				{
					self.score = Some(
						n * self
							.spaces
							.iter()
							.fold(0, |acc, space| acc + space.unwrap_or(0)),
					);
				}
				return;
			}
		}
	}
}

fn read_numbers(lines: &mut impl Iterator<Item = String>) -> Vec<i64> {
	lines
		.next()
		.expect("missing first line")
		.split(',')
		.map(|s| s.parse::<i64>().expect("failed to parse number"))
		.collect()
}

fn read_boards(lines: &mut impl Iterator<Item = String>) -> Vec<Board> {
	lines
		.chunks(SIDE_LENGTH + 1)
		.into_iter()
		.map(|chunk| Board {
			spaces: chunk
				.skip(1)
				.flat_map(|line| {
					line.split_whitespace()
						.map(|s| {
							Some(
								s.parse::<i64>()
									.expect("failed to parse board space"),
							)
						})
						.collect::<Vec<Option<i64>>>()
				})
				.collect::<Vec<Option<i64>>>()
				.try_into()
				.expect("wrong number of board spaces"),
			score: None,
		})
		.collect()
}

pub fn part1() -> i64 {
	let mut lines = read_lines("input/04.txt");
	let numbers = read_numbers(&mut lines);
	let mut boards = read_boards(&mut lines);

	for n in numbers {
		for board in boards.iter_mut() {
			board.tick(n);
			if let Some(score) = board.score {
				return score;
			}
		}
	}
	0
}

pub fn part2() -> i64 {
	let mut lines = read_lines("input/04.txt");
	let numbers = read_numbers(&mut lines);
	let mut boards = read_boards(&mut lines);

	let mut last_score = 0;
	for n in numbers {
		for board in boards.iter_mut() {
			board.tick(n);
			if let Some(score) = board.score {
				last_score = score;
			}
		}
		boards.retain(|board| board.score.is_none());
		if boards.is_empty() {
			break;
		}
	}
	last_score
}
