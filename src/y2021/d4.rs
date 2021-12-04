use crate::io::read_lines;

use itertools::Itertools;

pub fn part1() -> i64 {
	let mut lines = read_lines("input/2021/4-1.txt");
	let numbers = lines
		.next()
		.expect("missing first line")
		.split(',')
		.map(|s| s.parse::<i64>().expect("failed to parse number"))
		.collect::<Vec<i64>>();

	let mut boards = Vec::<Board>::new();
	for chunk in lines.chunks(6).into_iter() {
		let mut board = Board { v: Vec::new() };
		for line in chunk.skip(1) {
			board.v.push(
				line.split_whitespace()
					.map(|s| Some(s.parse::<i64>().expect("failed to parse board space")))
					.collect(),
			);
		}
		boards.push(board);
	}

	for n in numbers {
		for board in boards.iter_mut() {
			board.check(n);
			if board.bingo() {
				return board.score() * n;
			}
		}
	}
	0
}

struct Board {
	v: Vec<Vec<Option<i64>>>,
}

impl Board {
	fn check(&mut self, n: i64) {
		for i in 0..5 {
			for j in 0..5 {
				if self.v[i][j] == Some(n) {
					self.v[i][j] = None;
				}
			}
		}
	}

	fn bingo(&self) -> bool {
		'iloop: for i in 0..5 {
			for j in 0..5 {
				if self.v[i][j].is_some() {
					continue 'iloop;
				}
			}
			return true;
		}
		'jloop: for j in 0..5 {
			for i in 0..5 {
				if self.v[i][j].is_some() {
					continue 'jloop;
				}
			}
			return true;
		}
		false
	}

	fn score(&self) -> i64 {
		self.v.iter().fold(0, |acc, row| {
			acc + row.iter().fold(0, |acc, col| acc + col.unwrap_or(0))
		})
	}
}

pub fn part2() -> i64 {
	0
}
