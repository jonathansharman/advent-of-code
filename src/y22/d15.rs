use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 4748135);
crate::test::test_part!(test2, part2, ?);

const Y: i32 = 2_000_000;

#[derive(Clone, Copy, Debug)]
struct Range {
	start: i32,
	end: i32,
}

impl Range {
	fn len(&self) -> i32 {
		self.end - self.start + 1
	}

	fn contains(&self, value: i32) -> bool {
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
}

pub fn part1() -> usize {
	let (ranges, points) = read_lines("input/2022/15.txt")
		.map(|line| {
			let (sensor, beacon) = line.split_once(':').unwrap();
			let (sx, sy) = sensor
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.into_iter()
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i32>().unwrap())
					}
				})
				.into_iter()
				.collect_tuple()
				.unwrap();
			let (bx, by) = beacon
				.split(|c: char| c != '-' && !c.is_ascii_digit())
				.into_iter()
				.filter_map(|s| {
					if s.is_empty() {
						None
					} else {
						Some(s.parse::<i32>().unwrap())
					}
				})
				.into_iter()
				.collect_tuple()
				.unwrap();
			let radius = (sx - bx).abs() + (sy - by).abs();
			let reduced_radius = radius - (Y - sy).abs();
			if reduced_radius < 0 {
				(None, (bx, by))
			} else {
				(
					Some(Range {
						start: sx - reduced_radius,
						end: sx + reduced_radius,
					}),
					(bx, by),
				)
			}
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
		);
	let covered = ranges.iter().map(Range::len).sum::<i32>() as usize;
	let uncovered = points
		.iter()
		.unique()
		.filter(|p| p.1 == Y && ranges.iter().any(|range| range.contains(p.0)))
		.count();
	covered - uncovered
}

pub fn part2() -> usize {
	0
}
