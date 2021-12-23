use crate::io::read_lines;
use std::collections::HashMap;

crate::test::test_part!(test1, part1, 11608);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	read_state().min_energy_to_solve(&mut HashMap::new())
}

pub fn part2() -> u32 {
	read_state().min_energy_to_solve(&mut HashMap::new())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Amphipod {
	A,
	B,
	C,
	D,
}

impl Amphipod {
	fn energy_per_step(self) -> u32 {
		match self {
			Amphipod::A => 1,
			Amphipod::B => 10,
			Amphipod::C => 100,
			Amphipod::D => 1000,
		}
	}
}

impl TryFrom<u8> for Amphipod {
	type Error = ();

	fn try_from(c: u8) -> Result<Self, ()> {
		match c {
			b'A' => Ok(Amphipod::A),
			b'B' => Ok(Amphipod::B),
			b'C' => Ok(Amphipod::C),
			b'D' => Ok(Amphipod::D),
			_ => Err(()),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
	hall: [Option<Amphipod>; 11],
	outer: [Option<Amphipod>; 4],
	inner: [Option<Amphipod>; 4],
}

impl State {
	fn min_energy_to_solve(self, cache: &mut HashMap<State, u32>) -> u32 {
		if self.solved() {
			0
		} else {
			if let Some(&energy) = cache.get(&self) {
				return energy;
			}
			let energy = self
				.possible_next_state_energies()
				.into_iter()
				.map(|(state, energy)| energy.saturating_add(state.min_energy_to_solve(cache)))
				.fold(u32::MAX, |acc, energy| acc.min(energy));
			cache.insert(self, energy);
			energy
		}
	}

	fn swap_outer(&self, hall_idx: usize, room_idx: usize, amphipod: Amphipod) -> (State, u32) {
		let mut state = self.clone();
		let energy = steps_between(hall_idx, room_idx) * amphipod.energy_per_step();
		std::mem::swap(&mut state.outer[room_idx], &mut state.hall[hall_idx]);
		(state, energy)
	}

	fn swap_inner(&self, hall_idx: usize, room_idx: usize, amphipod: Amphipod) -> (State, u32) {
		let mut state = self.clone();
		let energy = (1 + steps_between(hall_idx, room_idx)) * amphipod.energy_per_step();
		std::mem::swap(&mut state.inner[room_idx], &mut state.hall[hall_idx]);
		(state, energy)
	}

	fn possible_next_state_energies(&self) -> Vec<(State, u32)> {
		let mut state_energies = Vec::new();
		// Skip room openings.
		for h in [0, 1, 3, 5, 7, 9, 10] {
			for r in 0..4 {
				if !self.hall_is_clear_between(h, r) {
					continue;
				}
				// Outer
				match (self.outer[r], self.inner[r], self.hall[h]) {
					// Outgoing
					(Some(amphipod), Some(neighbor), None) => {
						if amphipod != neighbor || !belongs(amphipod, r) {
							state_energies.push(self.swap_outer(h, r, amphipod));
						}
					}
					// Incoming
					(None, Some(neighbor), Some(amphipod)) => {
						if neighbor == amphipod && belongs(amphipod, r) {
							state_energies.push(self.swap_outer(h, r, amphipod));
						}
					}
					_ => (),
				}
				// Inner
				match (self.outer[r], self.inner[r], self.hall[h]) {
					// Outgoing
					(None, Some(amphipod), None) => {
						if !belongs(amphipod, r) {
							state_energies.push(self.swap_inner(h, r, amphipod));
						}
					}
					// Incoming
					(None, None, Some(amphipod)) => {
						if belongs(amphipod, r) {
							state_energies.push(self.swap_inner(h, r, amphipod));
						}
					}
					_ => (),
				}
			}
		}
		state_energies
	}

	fn hall_is_clear_between(&self, hall_idx: usize, room_idx: usize) -> bool {
		let room_pos = match room_idx {
			0 => 2,
			1 => 4,
			2 => 6,
			3 => 8,
			_ => panic!("room_idx out of bounds"),
		};
		let min = hall_idx.min(room_pos) + 1;
		let max = hall_idx.max(room_pos);
		self.hall[min..max].iter().all(Option::is_none)
	}

	fn solved(&self) -> bool {
		let sorted = [
			Some(Amphipod::A),
			Some(Amphipod::B),
			Some(Amphipod::C),
			Some(Amphipod::D),
		];
		self.outer == sorted && self.inner == sorted
	}
}

fn belongs(amphipod: Amphipod, room_idx: usize) -> bool {
	matches!(
		(amphipod, room_idx),
		(Amphipod::A, 0) | (Amphipod::B, 1) | (Amphipod::C, 2) | (Amphipod::D, 3)
	)
}

fn steps_between(hall_idx: usize, room_idx: usize) -> u32 {
	let room_pos = match room_idx {
		0 => 2,
		1 => 4,
		2 => 6,
		3 => 8,
		_ => panic!("room_idx out of bounds"),
	};
	(hall_idx.max(room_pos) - hall_idx.min(room_pos) + 1) as u32
}

fn read_state() -> State {
	let mut lines = read_lines("input/2021/23.txt")
		.map(String::into_bytes)
		.skip(2);
	let outer = lines.next().unwrap();
	let inner = lines.next().unwrap();
	State {
		hall: [None; 11],
		outer: [outer[3], outer[5], outer[7], outer[9]]
			.map(TryInto::try_into)
			.map(Result::ok),
		inner: [inner[3], inner[5], inner[7], inner[9]]
			.map(TryInto::try_into)
			.map(Result::ok),
	}
}
