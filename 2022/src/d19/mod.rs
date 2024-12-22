use aoc::input;

aoc::test::test_part!(test1, part1, 1395);
aoc::test::test_part!(test2, part2, 2700);

const TIME_LIMIT_1: usize = 24;
const TIME_LIMIT_2: usize = 32;

#[derive(Clone)]
struct Factory<'b> {
	blueprint: &'b Blueprint,
	time_left: usize,
	/// Ore, clay, obsidian, geodes.
	inv: [usize; 4],
	/// Produce ore, clay, obsidian, geodes.
	bots: [usize; 4],
}

impl Factory<'_> {
	fn max_geodes(mut self, build: usize) -> usize {
		// Determine the amount of time required to acquire resources to build
		// the target bot.
		let required_resources = self.blueprint[build];
		let mut wait = 0;
		for (i, req) in required_resources.into_iter().enumerate() {
			if req > self.inv[i] {
				if self.bots[i] == 0 {
					// No bots available to build a required resource.
					// Impossible to build this bot next.
					return self.inv[3];
				}
				let deficit = req - self.inv[i];
				wait = wait.max(deficit.div_ceil(self.bots[i]));
			}
		}
		if self.time_left < wait {
			return self.inv[3];
		}
		// Produce enough resources to build the bot.
		self.time_left -= wait;
		for (i, resource) in self.inv.iter_mut().enumerate() {
			*resource += self.bots[i] * wait;
		}
		// Build the bot on the next turn, producing resources first.
		if self.time_left == 0 {
			return self.inv[3];
		}
		self.time_left -= 1;
		for (i, resource) in self.inv.iter_mut().enumerate() {
			*resource += self.bots[i];
		}
		for (i, c) in self.inv.iter_mut().zip(self.blueprint[build].iter()) {
			*i -= c;
		}
		self.bots[build] += 1;
		// Try building all possible bots next and choose the best outcome.
		(0..4)
			.map(|build| self.clone().max_geodes(build))
			.max()
			.unwrap()
	}
}

/// Ore/clay/obsidian/geode costs for ore/clay/obsidian/geode bots.
type Blueprint = [[usize; 4]; 4];

fn max_geodes(time_limit: usize, blueprint: &Blueprint) -> usize {
	let factory = Factory {
		blueprint,
		time_left: time_limit,
		inv: [0; 4],
		bots: [1, 0, 0, 0],
	};
	(0..4)
		.map(|build| factory.clone().max_geodes(build))
		.max()
		.unwrap()
}

pub fn part1() -> usize {
	let overall_start = std::time::Instant::now();
	input!()
		.lines()
		.enumerate()
		.map(|(i, line)| {
			let start = std::time::Instant::now();
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
			let blueprint = [
				[numbers[1], 0, 0, 0],
				[numbers[2], 0, 0, 0],
				[numbers[3], numbers[4], 0, 0],
				[numbers[5], 0, numbers[6], 0],
			];
			let quality = (i + 1) * max_geodes(TIME_LIMIT_1, &blueprint);
			let now = std::time::Instant::now();
			let elapsed = (now - start).as_secs_f32();
			let total_elapsed = (now - overall_start).as_secs_f32();
			println!("Quality {i}: {quality} ({elapsed}s / {total_elapsed}s)");
			quality
		})
		.sum()
}

pub fn part2() -> usize {
	let overall_start = std::time::Instant::now();
	input!()
		.lines()
		.take(3)
		.map(|line| {
			let start = std::time::Instant::now();
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
			let blueprint = [
				[numbers[1], 0, 0, 0],
				[numbers[2], 0, 0, 0],
				[numbers[3], numbers[4], 0, 0],
				[numbers[5], 0, numbers[6], 0],
			];
			let m = max_geodes(TIME_LIMIT_2, &blueprint);
			let now = std::time::Instant::now();
			let elapsed = (now - start).as_secs_f32();
			let total_elapsed = (now - overall_start).as_secs_f32();
			println!("Max geodes: {m} ({elapsed}s / {total_elapsed}s)");
			m
		})
		.product()
}
