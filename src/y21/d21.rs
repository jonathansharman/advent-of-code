use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 684495);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	let (mut space1, mut space2) = read_starting_spaces();
	let (mut score1, mut score2) = (0, 0);
	let mut die = DeterministicDie { value: 0, rolls: 0 };
	loop {
		space1 += die.roll() + die.roll() + die.roll();
		space1 = (space1 - 1) % 10 + 1;
		score1 += space1;
		if score1 >= 1000 {
			return score2 * die.rolls;
		}
		space2 += die.roll() + die.roll() + die.roll();
		space2 = (space2 - 1) % 10 + 1;
		score2 += space2;
		if score2 >= 1000 {
			return score1 * die.rolls;
		}
	}
}

pub fn part2() -> u64 {
	let (space1, space2) = read_starting_spaces();
	let start_state = State {
		spaces: [space1, space2],
		scores: [0, 0],
	};
	*start_state.wins(0).iter().max().unwrap()
}

fn read_starting_spaces() -> (u32, u32) {
	read_lines("input/2021/21.txt")
		.map(|line| line.chars().last().unwrap().to_digit(10).unwrap())
		.collect_tuple()
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

#[derive(Clone)]
struct State {
	spaces: [u32; 2],
	scores: [u32; 2],
}

impl State {
	fn roll(&self, player: usize, roll: u32) -> State {
		let mut next = self.clone();
		next.spaces[player] = (next.spaces[player] + roll - 1) % 10 + 1;
		next.scores[player] += next.spaces[player];
		next
	}

	fn wins(&self, player: usize) -> [u64; 2] {
		let next1 = self.roll(player, 1);
		let next2 = self.roll(player, 2);
		let next3 = self.roll(player, 3);

		let mut wins = [0, 0];
		if next1.scores[player] >= 1000 {
			wins[player] += 1;
		} else {
			let next_wins = next1.wins(1 - player);
			wins[0] += next_wins[0];
			wins[1] += next_wins[1];
		}
		if next2.scores[player] >= 1000 {
			wins[player] += 1;
		} else {
			let next_wins = next2.wins(1 - player);
			wins[0] += next_wins[0];
			wins[1] += next_wins[1];
		}
		if next3.scores[player] >= 1000 {
			wins[player] += 1;
		} else {
			let next_wins = next3.wins(1 - player);
			wins[0] += next_wins[0];
			wins[1] += next_wins[1];
		}
		wins
	}
}
