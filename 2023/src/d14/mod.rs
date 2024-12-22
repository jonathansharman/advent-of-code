use std::{
	collections::{
		hash_map::{DefaultHasher, Entry},
		HashMap,
	},
	hash::{Hash, Hasher},
};

aoc::test::test_part!(test1, part1, 112048);
aoc::test::test_part!(test2, part2, 105606);


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
	Floor,
	Cube,
	RoundedRock,
}

#[derive(PartialEq, Eq, Hash)]
struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
	fn read() -> Tiles {
		Tiles(
			input!()
				.lines()
				.map(|line| {
					line.chars()
						.map(|c| match c {
							'O' => Tile::RoundedRock,
							'#' => Tile::Cube,
							_ => Tile::Floor,
						})
						.collect()
				})
				.collect(),
		)
	}

	fn get(&self, i: usize, j: usize) -> Tile {
		self.0[i][j]
	}

	fn get_mut(&mut self, i: usize, j: usize) -> &mut Tile {
		&mut self.0[i][j]
	}

	fn height(&self) -> usize {
		self.0.len()
	}

	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn get_load(&self) -> usize {
		self.0
			.iter()
			.enumerate()
			.map(|(i, row)| {
				let count = row
					.iter()
					.filter(|tile| matches!(tile, Tile::RoundedRock))
					.count();
				(self.height() - i) * count
			})
			.sum()
	}

	fn get_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish()
	}

	fn cycle(&mut self) {
		self.roll_north();
		self.roll_west();
		self.roll_south();
		self.roll_east();
	}

	fn roll_north(&mut self) {
		for iter in 0..self.height() - 1 {
			let mut unchanged = true;
			for i in 0..self.height() - 1 - iter {
				for j in 0..self.width() {
					let from = self.get(i + 1, j);
					let to = self.get_mut(i, j);
					if from == Tile::RoundedRock && *to == Tile::Floor {
						*to = Tile::RoundedRock;
						*self.get_mut(i + 1, j) = Tile::Floor;
						unchanged = false;
					}
				}
			}
			if unchanged {
				break;
			}
		}
	}

	fn roll_west(&mut self) {
		for iter in 0..self.width() - 1 {
			let mut unchanged = true;
			for i in 0..self.height() {
				for j in 0..self.width() - 1 - iter {
					let from = self.get(i, j + 1);
					let to = self.get_mut(i, j);
					if from == Tile::RoundedRock && *to == Tile::Floor {
						*to = Tile::RoundedRock;
						*self.get_mut(i, j + 1) = Tile::Floor;
						unchanged = false;
					}
				}
			}
			if unchanged {
				break;
			}
		}
	}

	fn roll_south(&mut self) {
		for iter in 0..self.height() - 1 {
			let mut unchanged = true;
			for i in (1 + iter..self.height()).rev() {
				for j in 0..self.width() {
					let from = self.get(i - 1, j);
					let to = self.get_mut(i, j);
					if from == Tile::RoundedRock && *to == Tile::Floor {
						*to = Tile::RoundedRock;
						*self.get_mut(i - 1, j) = Tile::Floor;
						unchanged = false;
					}
				}
			}
			if unchanged {
				break;
			}
		}
	}

	fn roll_east(&mut self) {
		for iter in 0..self.width() - 1 {
			let mut unchanged = true;
			for i in 0..self.height() {
				for j in (1 + iter..self.width()).rev() {
					let from = self.get(i, j - 1);
					let to = self.get_mut(i, j);
					if from == Tile::RoundedRock && *to == Tile::Floor {
						*to = Tile::RoundedRock;
						*self.get_mut(i, j - 1) = Tile::Floor;
						unchanged = false;
					}
				}
			}
			if unchanged {
				break;
			}
		}
	}

	#[allow(unused)]
	fn print(&self) {
		for line in self.0.iter() {
			for tile in line {
				match tile {
					Tile::Floor => print!("."),
					Tile::Cube => print!("#"),
					Tile::RoundedRock => print!("O"),
				}
			}
			println!();
		}
		println!();
	}
}

pub fn part1() -> usize {
	let mut tiles = Tiles::read();
	tiles.roll_north();
	tiles.get_load()
}

const TARGET_CYCLES: usize = 1_000_000_000;

pub fn part2() -> usize {
	let mut tiles = Tiles::read();
	let mut first_seen = HashMap::new();
	let mut cycles: usize = 0;
	loop {
		// Hash collisions will result in false positive loops, but it seems
		// worth the risk to avoid storing entire tile maps in the hash map.
		let hash = tiles.get_hash();
		match first_seen.entry(hash) {
			Entry::Occupied(loop_start) => {
				let loop_length = cycles - loop_start.get();
				let remaining = TARGET_CYCLES - cycles;
				cycles += remaining - (remaining % loop_length);
			}
			Entry::Vacant(entry) => {
				entry.insert(cycles);
			}
		}
		if cycles == TARGET_CYCLES {
			return tiles.get_load();
		}
		tiles.cycle();
		cycles += 1;
	}
}
