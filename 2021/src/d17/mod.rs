use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 5253);
aoc::test::test_part!(test2, part2, 1770);

pub fn part1() -> i64 {
	let target = read_target();
	let vy = target
		.intersecting_trajectories()
		.into_iter()
		.map(|trajectory| trajectory.vy)
		.max()
		.unwrap();
	vy * (vy + 1) / 2
}

pub fn part2() -> usize {
	read_target().intersecting_trajectories().len()
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

	fn intersecting_trajectories(&self) -> Vec<Trajectory> {
		let critical_vxs = [
			self.x_min,
			self.x_max,
			vx_to_reach(self.x_min),
			vx_to_reach(self.x_max),
		];
		let vx_min = critical_vxs.into_iter().min().unwrap();
		let vx_max = critical_vxs.into_iter().max().unwrap();
		// Any faster downward than this, and the trajectory immediately
		// overshoots downward.
		let vy_min = self.y_min;
		// Not sure if this is correct in general. The idea is to avoid
		// overshooting either y limit of the target.
		let vy_max = self.y_min.abs().max(self.y_max);

		let mut trajectories = Vec::new();
		for vx in vx_min..=vx_max {
			for vy in vy_min..=vy_max {
				let trajectory = Trajectory { vx, vy };
				if trajectory.hits(self) {
					trajectories.push(trajectory);
				}
			}
		}
		trajectories
	}
}

/// The smallest intitial `vx` needed for a trajectory to reach `x`.
///
/// This is derived from the triangular number formula and the quadratic
/// formula, to determine a `vx` that accumulates to at least `x`.
fn vx_to_reach(x: i64) -> i64 {
	let sign = if x.is_negative() { -1 } else { 1 };
	sign * ((1 + 8 * x.abs()) as f64).sqrt() as i64 / 2
}

struct Trajectory {
	vx: i64,
	vy: i64,
}

impl Trajectory {
	fn hits(&self, target: &Target) -> bool {
		let (mut vx, mut vy) = (self.vx, self.vy);
		let (mut x, mut y) = (0, 0);
		while vy >= 0 || y >= target.y_min {
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
			if target.contains(x, y) {
				return true;
			}
		}
		false
	}
}

fn read_target() -> Target {
	let input = input!().lines().next().unwrap();
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
