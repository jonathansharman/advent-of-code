use crate::io::read_lines;

crate::test::test_part!(test1, part1, 1395);
crate::test::test_part!(test2, part2, ?);

const TIME_LIMIT: usize = 24;

#[derive(Clone)]
struct Factory<'b> {
	blueprint: &'b Blueprint,
	time_left: usize,
	/// Ore, clay, obsidian, geodes.
	inv: [usize; 4],
	/// Produce ore, clay, obsidian, geodes.
	bots: [usize; 4],
}

impl<'b> Factory<'b> {
	fn max_geodes(mut self, build: Option<usize>) -> usize {
		if self.time_left == 0 {
			return self.inv[3];
		}
		self.time_left -= 1;
		// Produce.
		for (i, resource) in self.inv.iter_mut().enumerate() {
			*resource += self.bots[i];
		}
		// Build.
		if let Some(build) = build {
			for (i, c) in self
				.inv
				.iter_mut()
				.zip(self.blueprint.bot_ore_costs[build].iter())
			{
				*i -= c;
			}
			self.bots[build] += 1;
		}
		// Execute all possible moves (including doing nothing).
		let mut best = 0;
		for build in 0..4 {
			if self
				.inv
				.iter()
				.zip(self.blueprint.bot_ore_costs[build].iter())
				.all(|(i, c)| i >= c)
			{
				best = best.max(self.clone().max_geodes(Some(build)))
			}
		}
		best.max(self.max_geodes(None))
	}
}

struct Blueprint {
	id: usize,
	/// Ore/clay/obsidian/geode costs for ore/clay/obsidian/geode bots.
	bot_ore_costs: [[usize; 4]; 4],
}

impl Blueprint {
	fn quality(&self) -> usize {
		let factory = Factory {
			blueprint: self,
			time_left: TIME_LIMIT,
			inv: [0; 4],
			bots: [1, 0, 0, 0],
		};
		factory.max_geodes(None) * self.id
	}
}

pub fn part1() -> usize {
	let start = std::time::Instant::now();
	read_lines("input/2022/19.txt")
		.enumerate()
		.map(|(i, line)| {
			let numbers: Vec<usize> = line
				.split(|c: char| !c.is_ascii_digit())
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse().unwrap())
					}
				})
				.collect();
			let blueprint = Blueprint {
				id: i + 1,
				bot_ore_costs: [
					[numbers[1], 0, 0, 0],
					[numbers[2], 0, 0, 0],
					[numbers[3], numbers[4], 0, 0],
					[numbers[5], 0, numbers[6], 0],
				],
			};
			let quality = blueprint.quality();
			let elapsed = (std::time::Instant::now() - start).as_secs_f32();
			println!("Quality {i}: {quality} ({elapsed}s)");
			quality
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
