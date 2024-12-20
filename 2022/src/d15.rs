use itertools::Itertools;

aoc::test::test_part!(test1, part1, 4748135);
aoc::test::test_part!(test2, part2, 13743542639657);

const INPUT: &str = include_str!("input/15.txt");

struct Area {
	x: i64,
	y: i64,
	radius: i64,
}

impl Area {
	fn range(&self, y: i64) -> Option<Range> {
		let reduced_radius = self.radius - (y - self.y).abs();
		if reduced_radius < 0 {
			None
		} else {
			Some(Range {
				start: self.x - reduced_radius,
				end: self.x + reduced_radius,
			})
		}
	}
}

#[derive(Clone, Copy, Debug)]
struct Range {
	start: i64,
	end: i64,
}

impl Range {
	fn len(&self) -> i64 {
		self.end - self.start + 1
	}

	fn contains(&self, value: i64) -> bool {
		self.start <= value && value <= self.end
	}

	fn overlaps(&self, other: &Range) -> bool {
		self.contains(other.start)
			|| self.contains(other.end)
			|| other.contains(self.start)
			|| other.contains(self.end)
	}

	fn union(&self, other: &Range) -> Vec<Range> {
		if other.start < self.start {
			other.union(self)
		} else if other.end <= self.end {
			vec![*self]
		} else if other.start > self.end {
			vec![*self, *other]
		} else {
			vec![Range {
				start: self.start,
				end: other.end,
			}]
		}
	}

	fn intersect(&self, other: &Range) -> Option<Range> {
		if !self.overlaps(other) {
			None
		} else {
			Some(Range {
				start: self.start.max(other.start),
				end: self.end.min(other.end),
			})
		}
	}
}

fn ranges_points(target_y: i64) -> (Vec<Range>, Vec<(i64, i64)>) {
	INPUT
		.lines()
		.map(|line| {
			let (sensor, beacon) = line.split_once(':').unwrap();
			let (sx, sy) = sensor
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i64>().unwrap())
					}
				})
				.collect_tuple()
				.unwrap();
			let (bx, by) = beacon
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i64>().unwrap())
					}
				})
				.collect_tuple()
				.unwrap();
			let radius = (sx - bx).abs() + (sy - by).abs();
			let area = Area {
				x: sx,
				y: sy,
				radius,
			};
			(area.range(target_y), (bx, by))
		})
		.fold(
			(Vec::new(), Vec::new()),
			|(mut ranges, mut points), (range, point)| {
				if let Some(range) = range {
					let mut queue = ranges;
					let mut new_ranges = vec![range];
					'work: while let Some(next) = queue.pop() {
						for idx in 0..new_ranges.len() {
							if new_ranges[idx].overlaps(&next) {
								let removed = new_ranges.remove(idx);
								queue.append(&mut removed.union(&next));
								continue 'work;
							}
						}
						new_ranges.push(next);
					}
					ranges = new_ranges;
				}
				points.push(point);
				(ranges, points)
			},
		)
}

pub fn part1() -> usize {
	const Y: i64 = 2_000_000;
	let (ranges, points) = ranges_points(Y);
	let covered = ranges.iter().map(Range::len).sum::<i64>() as usize;
	let uncovered = points
		.iter()
		.unique()
		.filter(|p| p.1 == Y && ranges.iter().any(|range| range.contains(p.0)))
		.count();
	covered - uncovered
}

fn get_areas() -> Vec<Area> {
	INPUT
		.lines()
		.map(|line| {
			let (sensor, beacon) = line.split_once(':').unwrap();
			let (sx, sy) = sensor
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i64>().unwrap())
					}
				})
				.collect_tuple()
				.unwrap();
			let (bx, by) = beacon
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i64>().unwrap())
					}
				})
				.collect_tuple()
				.unwrap();
			let radius = (sx - bx).abs() + (sy - by).abs();
			Area {
				x: sx,
				y: sy,
				radius,
			}
		})
		.collect()
}

pub fn part2() -> i64 {
	let areas = get_areas();

	const MAX_COORD: i64 = 4_000_000;
	let window = Range {
		start: 0,
		end: MAX_COORD,
	};
	for y in 0..=MAX_COORD {
		let ranges = areas
			.iter()
			.filter_map(|area| {
				area.range(y).and_then(|range| range.intersect(&window))
			})
			.fold(Vec::new(), |ranges, range| {
				let mut queue = ranges;
				let mut new_ranges = vec![range];
				'work: while let Some(next) = queue.pop() {
					for idx in 0..new_ranges.len() {
						if new_ranges[idx].overlaps(&next) {
							let removed = new_ranges.remove(idx);
							queue.append(&mut removed.union(&next));
							continue 'work;
						}
					}
					new_ranges.push(next);
				}
				new_ranges
			});
		if ranges[0].len() < MAX_COORD {
			let x = if ranges.len() == 1 {
				if ranges[0].start == 0 {
					MAX_COORD
				} else {
					0
				}
			} else if ranges[0].start == 0 {
				ranges[0].end + 1
			} else {
				ranges[1].end + 1
			};
			return MAX_COORD * x + y;
		}
	}
	0
}
