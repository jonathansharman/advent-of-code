use std::collections::HashMap;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

struct Blizzards {
	blizzards: Vec<[bool; 4]>,
	width: i32,
	height: i32,
}

impl Blizzards {
	fn load() -> Blizzards {
		let lines = read_lines("input/2022/24.txt").collect::<Vec<_>>();
		let blizzards_vecs = lines[1..lines.len() - 1]
			.iter()
			.map(|line| {
				line.chars()
					.filter_map(|c| match c {
						'^' => Some([true, false, false, false]),
						'v' => Some([false, true, false, false]),
						'<' => Some([false, false, true, false]),
						'>' => Some([false, false, false, true]),
						'#' => None,
						_ => Some([false; 4]),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let width = blizzards_vecs[0].len() as i32;
		let height = blizzards_vecs.len() as i32;
		let blizzards = Vec::from_iter(
			blizzards_vecs.into_iter().flat_map(|v| v.into_iter()),
		);
		Blizzards {
			blizzards,
			width,
			height,
		}
	}

	fn open(&self, coords: [i32; 2], time: i32) -> bool {
		if coords == [-1, 0] || coords == [self.height, self.width - 1] {
			return true;
		}
		if coords[0] < 0
			|| coords[0] >= self.height
			|| coords[1] < 0
			|| coords[1] >= self.width
		{
			return false;
		}
		let idx = ((coords[0] + time) % self.height) * self.width + coords[1];
		let southerly = self.blizzards[idx as usize][0];
		let idx = ((((coords[0] - time) % self.height) + self.height)
			% self.height)
			* self.width + coords[1];
		let northerly = self.blizzards[idx as usize][1];
		let idx = coords[0] * self.width + (coords[1] + time) % self.width;
		let easterly = self.blizzards[idx as usize][2];
		let idx = coords[0] * self.width
			+ ((coords[1] - time) % self.width + self.width) % self.width;
		let westerly = self.blizzards[idx as usize][3];
		!southerly && !northerly && !easterly && !westerly
	}

	fn time_to_exit(
		&self,
		cache: &mut HashMap<([i32; 2], i32), Option<usize>>,
		coords: [i32; 2],
		time: i32,
		time_limit: i32,
	) -> Option<usize> {
		if coords == [self.height, self.width - 1] {
			let result = Some(0);
			cache.insert((coords, time), result);
			result
		} else if !self.open(coords, time) || time == time_limit {
			cache.insert((coords, time), None);
			None
		} else {
			let wait = self
				.time_to_exit(cache, coords, time + 1, time_limit)
				.map(|v| v + 1);
			let north = self
				.time_to_exit(
					cache,
					[coords[0] - 1, coords[1]],
					time + 1,
					time_limit,
				)
				.map(|v| v + 1);
			let south = self
				.time_to_exit(
					cache,
					[coords[0] + 1, coords[1]],
					time + 1,
					time_limit,
				)
				.map(|v| v + 1);
			let west = self
				.time_to_exit(
					cache,
					[coords[0], coords[1] - 1],
					time + 1,
					time_limit,
				)
				.map(|v| v + 1);
			let east = self
				.time_to_exit(
					cache,
					[coords[0], coords[1] + 1],
					time + 1,
					time_limit,
				)
				.map(|v| v + 1);

			let result = wait
				.unwrap_or(usize::MAX)
				.min(north.unwrap_or(usize::MAX))
				.min(south.unwrap_or(usize::MAX))
				.min(west.unwrap_or(usize::MAX))
				.min(east.unwrap_or(usize::MAX));
			let result = if result == usize::MAX {
				None
			} else {
				Some(result)
			};
			cache.insert((coords, time), result);
			result
		}
	}
}

pub fn part1() -> usize {
	let blizzards = Blizzards::load();
	let mut cache = HashMap::new();
	for time_limit in blizzards.width + blizzards.height.. {
		println!("Time limit: {time_limit}");
		if let Some(time) =
			blizzards.time_to_exit(&mut cache, [-1, 0], 0, time_limit)
		{
			return time;
		}
	}
	panic!("No answer found");
}

pub fn part2() -> usize {
	0
}
