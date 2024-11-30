use aoc::io::read_lines;
use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 11608);
aoc::test::test_part!(test2, part2, 46754);

pub fn part1() -> u32 {
	read_state().min_energy_to_solve(&mut HashMap::new())
}

pub fn part2() -> u32 {
	let a = Some(Amphipod { home: 0 });
	let b = Some(Amphipod { home: 1 });
	let c = Some(Amphipod { home: 2 });
	let d = Some(Amphipod { home: 3 });

	let mut state = read_state();
	state.rooms[0].splice(1..1, [d, d]);
	state.rooms[1].splice(1..1, [c, b]);
	state.rooms[2].splice(1..1, [b, a]);
	state.rooms[3].splice(1..1, [a, c]);
	state.min_energy_to_solve(&mut HashMap::new())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Amphipod {
	home: usize,
}

impl Amphipod {
	fn energy_per_step(self) -> u32 {
		u32::pow(10, self.home as u32)
	}
}

impl TryFrom<u8> for Amphipod {
	type Error = ();

	fn try_from(c: u8) -> Result<Self, ()> {
		match c {
			b'A' => Ok(Amphipod { home: 0 }),
			b'B' => Ok(Amphipod { home: 1 }),
			b'C' => Ok(Amphipod { home: 2 }),
			b'D' => Ok(Amphipod { home: 3 }),
			_ => Err(()),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
	hall: [Option<Amphipod>; 11],
	/// Room spaces indexed by room then by increasing distance from the hall
	rooms: [Vec<Option<Amphipod>>; 4],
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
				.map(|(state, energy)| {
					energy.saturating_add(state.min_energy_to_solve(cache))
				})
				.fold(u32::MAX, |acc, energy| acc.min(energy));
			cache.insert(self, energy);
			energy
		}
	}

	fn swap(
		&self,
		hall_idx: usize,
		room_idx: usize,
		distance: usize,
		amphipod: Amphipod,
	) -> (State, u32) {
		let mut state = self.clone();
		std::mem::swap(
			&mut state.rooms[room_idx][distance],
			&mut state.hall[hall_idx],
		);
		let steps = steps_between(hall_idx, room_idx) + distance as u32;
		let energy = steps * amphipod.energy_per_step();
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
				for d in 0..self.rooms[r].len() {
					match (self.hall[h], self.rooms[r][d]) {
						// Incoming
						(Some(hall_amphipod), None) => {
							let mut can_enter = hall_amphipod.home == r;
							for d2 in d + 1..self.rooms[r].len() {
								match self.rooms[r][d2] {
									Some(Amphipod { home }) => {
										if home != r {
											can_enter = false;
											break;
										}
									}
									None => {
										can_enter = false;
										break;
									}
								}
							}
							if can_enter {
								state_energies.push(self.swap(
									h,
									r,
									d,
									hall_amphipod,
								));
								break;
							}
						}
						// Outgoing
						(None, Some(room_amphipod)) => {
							let mut can_leave = room_amphipod.home != r;
							for d2 in d + 1..self.rooms[r].len() {
								if self.rooms[r][d2].unwrap().home != r {
									can_leave = true;
									break;
								}
							}
							if can_leave {
								state_energies.push(self.swap(
									h,
									r,
									d,
									room_amphipod,
								));
								break;
							}
						}
						_ => (),
					}
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
		self.rooms.iter().enumerate().all(|(room_idx, room)| {
			room.iter()
				.all(|amphipod| amphipod.map(|a| a.home) == Some(room_idx))
		})
	}
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
	let mut lines = read_lines("input/23.txt")
		.map(String::into_bytes)
		.skip(2);
	let depth0 = lines.next().unwrap();
	let depth1 = lines.next().unwrap();
	State {
		hall: [None; 11],
		rooms: [
			vec![depth0[3].try_into().ok(), depth1[3].try_into().ok()],
			vec![depth0[5].try_into().ok(), depth1[5].try_into().ok()],
			vec![depth0[7].try_into().ok(), depth1[7].try_into().ok()],
			vec![depth0[9].try_into().ok(), depth1[9].try_into().ok()],
		],
	}
}
