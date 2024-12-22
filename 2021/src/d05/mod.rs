use aoc::input;

aoc::test::test_part!(test1, part1, 6225);
aoc::test::test_part!(test2, part2, 22116);

pub fn part1() -> usize {
	intersections(true)
}

pub fn part2() -> usize {
	intersections(false)
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

struct Line {
	start: Point,
	end: Point,
}

fn intersections(ignore_diagonals: bool) -> usize {
	let lines = input!()
		.lines()
		.map(|line_str| {
			let points = line_str
				.splitn(2, " -> ")
				.map(|point_str| {
					let coordinates =
						point_str.splitn(2, ',').collect::<Vec<&str>>();
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
		if ignore_diagonals
			&& line.start.x != line.end.x
			&& line.start.y != line.end.y
		{
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
		.count()
}
