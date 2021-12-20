use crate::io::read_lines;
use itertools::Itertools;
use std::{collections::HashSet, hash::Hash, ops::Index};

crate::test::test_part!(test1, part1, 357);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut scanners = read_scanners();
	'merge_loop: loop {
		let n = scanners.len();
		for i in 0..n - 1 {
			for j in i + 1..n {
				if let Some(merged_scanner) = scanners[i].merge(&scanners[j]) {
					scanners[i] = merged_scanner;
					scanners.swap_remove(j);
					continue 'merge_loop;
				}
			}
		}
		break;
	}
	scanners.iter().map(|scanner| scanner.readings.len()).sum()
}

pub fn part2() -> usize {
	0
}

fn read_scanners() -> Vec<Scanner> {
	let mut lines = read_lines("input/2021/19.txt");
	let mut scanners = Vec::new();
	while lines.next().is_some() {
		scanners.push(read_scanner(&mut lines));
	}
	scanners
}

fn read_scanner(lines: &mut impl Iterator<Item = String>) -> Scanner {
	let mut readings = HashSet::new();
	loop {
		match lines.next() {
			Some(line) => {
				if line.is_empty() {
					return Scanner { readings };
				}
				let reading: [i32; 3] = line
					.split(',')
					.map(|n| n.parse().unwrap())
					.collect_vec()
					.try_into()
					.unwrap();
				readings.insert(Point(reading));
			}
			None => return Scanner { readings },
		}
	}
}

struct Scanner {
	readings: HashSet<Point>,
}

impl Scanner {
	/// Projects the readings to a single `axis` and then maps them using `f`.
	fn project_map_readings<T, F>(&self, axis: usize, f: F) -> HashSet<T>
	where
		T: Hash + Eq,
		F: Fn(i32) -> T,
	{
		self.readings.iter().map(|p| f(p[axis])).collect()
	}

	fn merge(&self, other: &Scanner) -> Option<Scanner> {
		use Heading::*;
		for heading in [PlusX, MinusX, PlusY, MinusY, PlusZ, MinusZ] {
			use Rotation::*;
			for rotation in [Zero, Quarter, Half, ThreeQuarters] {
				let other = other.reorient(Orientation { rotation, heading });

				let translation_sets = [0, 1, 2].map(|axis| {
					let mut coords = HashSet::new();
					for self_reading in self.readings.iter() {
						for other_reading in other.readings.iter() {
							coords.insert(self_reading[axis] - other_reading[axis]);
						}
					}
					coords
				});

				let axis_alignment = |axis: usize| {
					let self_coords = self.project_map_readings(axis, |x| x);
					for translation in translation_sets[axis].iter() {
						let other_coords = other.project_map_readings(axis, |x| x + translation);

						let total_coords = self_coords.len() + other_coords.len();
						let common_coords = self_coords.union(&other_coords).count();
						if total_coords - common_coords >= 12 {
							return Some(*translation);
						}
					}
					None
				};
				if let Some(x_translation) = axis_alignment(0) {
					if let Some(y_translation) = axis_alignment(1) {
						if let Some(z_translation) = axis_alignment(2) {
							let translation = [x_translation, y_translation, z_translation];
							let other = other.translate(translation);
							return Some(Scanner {
								readings: self.readings.union(&other.readings).cloned().collect(),
							});
						}
					}
				}
			}
		}
		None
	}

	fn reorient(&self, orientation: Orientation) -> Scanner {
		Scanner {
			readings: self
				.readings
				.iter()
				.map(|p| p.reorient(orientation))
				.collect(),
		}
	}

	fn translate(&self, translation: [i32; 3]) -> Scanner {
		Scanner {
			readings: self
				.readings
				.iter()
				.map(|p| {
					Point([
						p[0] + translation[0],
						p[1] + translation[1],
						p[2] + translation[2],
					])
				})
				.collect(),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point([i32; 3]);

impl Point {
	fn reorient(self, orientation: Orientation) -> Point {
		// Rotate around x-axis.
		let rotated = match orientation.rotation {
			Rotation::Zero => self,
			Rotation::Quarter => Point([self[0], -self[2], self[1]]),
			Rotation::Half => Point([self[0], -self[1], -self[2]]),
			Rotation::ThreeQuarters => Point([self[0], self[2], -self[1]]),
		};
		match orientation.heading {
			// Rotate around y-axis.
			Heading::PlusX => rotated,
			Heading::MinusX => Point([-rotated[0], rotated[1], -rotated[2]]),
			// Rotate around z-axis.
			Heading::PlusY => Point([-rotated[1], rotated[0], rotated[2]]),
			Heading::MinusY => Point([rotated[1], -rotated[0], rotated[2]]),
			// Rotate around y-axis.
			Heading::PlusZ => Point([-rotated[2], rotated[1], rotated[0]]),
			Heading::MinusZ => Point([rotated[2], rotated[1], -rotated[0]]),
		}
	}
}

impl Index<usize> for Point {
	type Output = i32;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

#[derive(Clone, Copy)]
enum Rotation {
	Zero,
	Quarter,
	Half,
	ThreeQuarters,
}

#[derive(Clone, Copy)]
enum Heading {
	PlusX,
	MinusX,
	PlusY,
	MinusY,
	PlusZ,
	MinusZ,
}

#[derive(Clone, Copy)]
struct Orientation {
	rotation: Rotation,
	heading: Heading,
}
