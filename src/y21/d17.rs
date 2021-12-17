use crate::io::read_lines;
use itertools::Itertools;

crate::test::test_part!(test1, part1, 5253);
crate::test::test_part!(test2, part2, 1770);

pub fn part1() -> i64 {
	let target = read_target();
	let mut max_vy_hit: Option<i64> = None;
	for vy in -1_000..=1_000 {
		for vx in 1..=1_000 {
			if target.hit_time(vx, vy).is_some() {
				max_vy_hit = Some(match max_vy_hit {
					Some(current) => current.max(vy),
					None => vy,
				})
			}
		}
	}
	let vy = max_vy_hit.unwrap();
	vy * (vy + 1) / 2
}

pub fn part2() -> usize {
	let target = read_target();
	let mut hits = 0;
	for vy in -1_000..=1_000 {
		for vx in 1..=1_000 {
			if target.hit_time(vx, vy).is_some() {
				hits += 1;
			}
		}
	}
	hits
}

struct Target {
	x_min: i64,
	x_max: i64,
	y_min: i64,
	y_max: i64,
}

impl Target {
	fn contains(&self, x: i64, y: i64) -> bool {
		self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
	}

	fn hit_time(&self, mut vx: i64, mut vy: i64) -> Option<u32> {
		let (mut x, mut y) = (0, 0);
		let mut t = 0;
		while vy >= 0 || y >= self.y_min {
			t += 1;
			// Apply velocity.
			x += vx;
			y += vy;
			// Apply drag and gravity.
			if vx.is_negative() {
				vx += 1;
			} else if vx.is_positive() {
				vx -= 1;
			}
			vy -= 1;
			// Check containment.
			if self.contains(x, y) {
				return Some(t);
			}
		}
		None
	}
}

fn read_target() -> Target {
	let input = read_lines("input/2021/17.txt").next().unwrap();
	let x_start = input.find("x=").unwrap() + 2;
	let x_end = input.find(", y=").unwrap();
	let y_start = x_end + 4;
	let (x_min, x_max) = parse_range(&input[x_start..x_end]);
	let (y_min, y_max) = parse_range(&input[y_start..]);
	Target {
		x_min,
		x_max,
		y_min,
		y_max,
	}
}

fn parse_range(range: &str) -> (i64, i64) {
	let ends = range.split("..").map(|s| s.parse().unwrap()).collect_vec();
	(ends[0], ends[1])
}
