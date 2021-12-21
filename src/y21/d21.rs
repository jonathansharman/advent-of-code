use crate::io::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

crate::test::test_part!(test1, part1, 684495);
crate::test::test_part!(test2, part2, 152587196649184);

pub fn part1() -> u32 {
	let mut spaces = read_starting_spaces();
	let mut scores = [0, 0];
	let mut player = 0;
	let mut die = DeterministicDie { value: 0, rolls: 0 };
	loop {
		spaces[player] += die.roll() + die.roll() + die.roll();
		spaces[player] = (spaces[player] - 1) % 10 + 1;
		scores[player] += spaces[player];
		if scores[player] >= 1000 {
			return scores[1 - player] * die.rolls;
		}
		player = 1 - player;
	}
}

pub fn part2() -> u64 {
	let start_state = State {
		spaces: read_starting_spaces(),
		scores: [0, 0],
	};
	*start_state
		.wins(&mut HashMap::new(), 0, 3)
		.iter()
		.max()
		.unwrap()
}

fn read_starting_spaces() -> [u32; 2] {
	read_lines("input/2021/21.txt")
		.map(|line| line.chars().last().unwrap().to_digit(10).unwrap())
		.collect_vec()
		.try_into()
		.unwrap()
}

struct DeterministicDie {
	value: u32,
	rolls: u32,
}

impl DeterministicDie {
	fn roll(&mut self) -> u32 {
		self.value += 1;
		self.rolls += 1;
		if self.value > 100 {
			self.value = 1;
		}
		self.value
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
	spaces: [u32; 2],
	scores: [u32; 2],
}

impl State {
	fn roll(&self, player: usize, roll: u32) -> State {
		let mut next = self.clone();
		next.spaces[player] = (next.spaces[player] + roll - 1) % 10 + 1;
		next
	}

	fn wins(
		&self,
		cache: &mut HashMap<(State, usize), [u64; 2]>,
		player: usize,
		rolls: u32,
	) -> [u64; 2] {
		let mut wins = [0, 0];
		if rolls == 0 {
			if let Some(wins) = cache.get(&(self.clone(), player)) {
				return *wins;
			}
			let mut next = self.clone();
			next.scores[player] += next.spaces[player];
			if next.scores[player] >= 21 {
				wins[player] += 1;
			} else {
				let next_wins = next.wins(cache, 1 - player, 3);
				wins[0] += next_wins[0];
				wins[1] += next_wins[1];
			}
			cache.insert((self.clone(), player), wins);
		} else {
			for roll in 1..=3 {
				let next_wins = self.roll(player, roll).wins(cache, player, rolls - 1);
				wins[0] += next_wins[0];
				wins[1] += next_wins[1];
			}
		}
		wins
	}
}
