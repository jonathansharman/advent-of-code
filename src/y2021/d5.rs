use crate::io::read_lines;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

struct Line {
	start: Point,
	end: Point,
}

fn intersections(ignore_diagonals: bool) -> i64 {
	let lines = read_lines("input/2021/5.txt")
		.map(|line_str| {
			let points = line_str
				.splitn(2, " -> ")
				.map(|point_str| {
					let coordinates = point_str.splitn(2, ',').collect::<Vec<&str>>();
					Point {
						x: coordinates[0].parse().unwrap(),
						y: coordinates[1].parse().unwrap(),
					}
				})
				.collect::<Vec<Point>>();
			Line {
				start: points[0],
				end: points[1],
			}
		})
		.collect::<Vec<Line>>();

	let (max_x, max_y) = lines.iter().fold((0, 0), |acc, line| {
		(
			acc.0.max(line.start.x).max(line.end.x),
			acc.1.max(line.start.y).max(line.end.y),
		)
	});
	let mut field = vec![vec![0; max_x + 1]; max_y + 1];

	for line in lines {
		if ignore_diagonals && line.start.x != line.end.x && line.start.y != line.end.y {
			continue;
		}
		let mut p = line.start;
		while p != line.end {
			field[p.y][p.x] += 1;
			use std::cmp::Ordering::*;
			match line.start.x.cmp(&line.end.x) {
				Less => p.x += 1,
				Equal => (),
				Greater => p.x -= 1,
			}
			match line.start.y.cmp(&line.end.y) {
				Less => p.y += 1,
				Equal => (),
				Greater => p.y -= 1,
			}
		}
		field[line.end.y][line.end.x] += 1;
	}

	field
		.iter()
		.flat_map(|row| row.iter().filter(|&&p| p > 1))
		.count() as i64
}

pub fn part1() -> i64 {
	intersections(true)
}

pub fn part2() -> i64 {
	intersections(false)
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(6225, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(22116, super::part2());
	}
}
