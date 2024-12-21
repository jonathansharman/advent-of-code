use itertools::Itertools;
use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 684495);
aoc::test::test_part!(test2, part2, 152587196649184);

const INPUT: &str = include_str!("input.txt");

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
	INPUT
		.lines()
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
		if rolls > 0 {
			return (1..=3)
				.map(|roll| {
					self.roll(player, roll).wins(cache, player, rolls - 1)
				})
				.fold([0, 0], |acc, wins| {
					[acc[0] + wins[0], acc[1] + wins[1]]
				});
		}

		if let Some(wins) = cache.get(&(self.clone(), player)) {
			return *wins;
		}

		let mut next = self.clone();
		next.scores[player] += next.spaces[player];
		let wins = if next.scores[player] >= 21 {
			if player == 0 {
				[1, 0]
			} else {
				[0, 1]
			}
		} else {
			next.wins(cache, 1 - player, 3)
		};

		cache.insert((self.clone(), player), wins);
		wins
	}
}
