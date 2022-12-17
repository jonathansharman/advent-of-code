use std::collections::{BTreeSet, HashSet};

crate::test::test_part!(test1, part1, 3179);
crate::test::test_part!(test2, part2, ?);

fn get_wind() -> Vec<i64> {
	std::fs::read_to_string("input/2022/17.txt")
		.unwrap()
		.chars()
		.map(|c| if c == '>' { 1 } else { -1 })
		.collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coords {
	x: i64,
	y: i64,
}

impl Ord for Coords {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.y.cmp(&other.y).reverse().then(self.x.cmp(&other.x))
	}
}

impl PartialOrd for Coords {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl From<(i64, i64)> for Coords {
	fn from(coords: (i64, i64)) -> Self {
		Coords {
			x: coords.0,
			y: coords.1,
		}
	}
}

fn get_shapes() -> Vec<HashSet<Coords>> {
	vec![
		// ####
		HashSet::from([
			(0, 0).into(),
			(1, 0).into(),
			(2, 0).into(),
			(3, 0).into(),
		]),
		//  #
		// ###
		//  #
		HashSet::from([
			(0, 1).into(),
			(1, 0).into(),
			(1, 1).into(),
			(1, 2).into(),
			(2, 1).into(),
		]),
		//   #
		//   #
		// ###
		HashSet::from([
			(0, 0).into(),
			(1, 0).into(),
			(2, 0).into(),
			(2, 1).into(),
			(2, 2).into(),
		]),
		// #
		// #
		// #
		// #
		HashSet::from([
			(0, 0).into(),
			(0, 1).into(),
			(0, 2).into(),
			(0, 3).into(),
		]),
		// ##
		// ##
		HashSet::from([
			(0, 0).into(),
			(1, 0).into(),
			(0, 1).into(),
			(1, 1).into(),
		]),
	]
}

fn get_rock(shape: &HashSet<Coords>, coords: Coords) -> HashSet<Coords> {
	shape
		.iter()
		.map(|offset| Coords {
			x: coords.x + offset.x,
			y: coords.y + offset.y,
		})
		.collect()
}

fn collides(blocks: &HashSet<Coords>, rock: &HashSet<Coords>) -> bool {
	rock.iter().any(|block| block.x < 0 || block.x > 6)
		|| blocks.intersection(rock).next().is_some()
}

#[allow(unused)]
fn print_state(
	blocks: &HashSet<Coords>,
	ordered_blocks: &BTreeSet<Coords>,
	shape: &HashSet<Coords>,
	rock_coords: Coords,
) {
	let rock = get_rock(shape, rock_coords);
	let max_y = ordered_blocks
		.iter()
		.next()
		.map_or(0, |block| block.y)
		.max(rock.iter().map(|block| block.y).max().unwrap_or_default());
	for y in (1..=max_y).rev() {
		print!("|");
		for x in 0..7 {
			let coords = Coords { x, y };
			if rock.contains(&coords) {
				print!("@");
			} else if blocks.contains(&coords) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("|");
	}
	println!("+-------+");
}

pub fn part1() -> i64 {
	let (winds, mut wind_idx) = (get_wind(), 0);
	let (shapes, mut shape_idx) = (get_shapes(), 0);
	let mut landed_count = 0;
	let mut blocks: HashSet<Coords> = HashSet::new();
	let mut ordered_blocks: BTreeSet<Coords> = BTreeSet::new();
	let mut rock_coords: Coords;
	'spawn: loop {
		let y = 4 + ordered_blocks.iter().next().map_or(0, |block| block.y);
		rock_coords = Coords { x: 2, y };
		let shape = &shapes[shape_idx];
		shape_idx = (shape_idx + 1) % shapes.len();
		loop {
			// Blow.
			let wind = &winds[wind_idx];
			wind_idx = (wind_idx + 1) % winds.len();
			let blown = Coords {
				x: rock_coords.x + wind,
				y: rock_coords.y,
			};
			if !collides(&blocks, &get_rock(shape, blown)) {
				rock_coords = blown;
			}
			// Fall.
			let fallen = Coords {
				x: rock_coords.x,
				y: rock_coords.y - 1,
			};
			if fallen.y > 0 && !collides(&blocks, &get_rock(shape, fallen)) {
				rock_coords = fallen;
			} else {
				for block in get_rock(shape, rock_coords) {
					blocks.insert(block);
					ordered_blocks.insert(block);
				}
				landed_count += 1;
				if landed_count == 2022 {
					return ordered_blocks.iter().next().unwrap().y;
				}
				continue 'spawn;
			}
		}
	}
}

pub fn part2() -> i64 {
	let (winds, mut wind_idx) = (get_wind(), 0);
	let (shapes, mut shape_idx) = (get_shapes(), 0);
	let mut landed_count: usize = 0;
	let mut blocks: HashSet<Coords> = HashSet::new();
	let mut ordered_blocks: BTreeSet<Coords> = BTreeSet::new();
	let mut rock_coords: Coords;
	'spawn: loop {
		let y = 4 + ordered_blocks.iter().next().map_or(0, |block| block.y);
		rock_coords = Coords { x: 2, y };
		let shape = &shapes[shape_idx];
		shape_idx = (shape_idx + 1) % shapes.len();
		loop {
			// Blow.
			let wind = &winds[wind_idx];
			wind_idx = (wind_idx + 1) % winds.len();
			let blown = Coords {
				x: rock_coords.x + wind,
				y: rock_coords.y,
			};
			if !collides(&blocks, &get_rock(shape, blown)) {
				rock_coords = blown;
			}
			// Fall.
			let fallen = Coords {
				x: rock_coords.x,
				y: rock_coords.y - 1,
			};
			if fallen.y > 0 && !collides(&blocks, &get_rock(shape, fallen)) {
				rock_coords = fallen;
			} else {
				for block in get_rock(shape, rock_coords) {
					blocks.insert(block);
					ordered_blocks.insert(block);
				}
				landed_count += 1;
				if landed_count == 1_000_000_000_000 {
					return ordered_blocks.iter().next().unwrap().y;
				}
				continue 'spawn;
			}
		}
	}
}
