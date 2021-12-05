use crate::io::read_lines;

type Point = (usize, usize);
type Line = (Point, Point);

pub fn part1() -> i64 {
	let lines = read_lines("input/2021/5-1.txt");
	let lines = lines
		.map(|line| {
			let start_end = line.split(" -> ").collect::<Vec<&str>>();
			let (start, end) = (start_end[0], start_end[1]);
			let x1_y1 = start.split(',').collect::<Vec<&str>>();
			let x2_y2 = end.split(',').collect::<Vec<&str>>();
			let (x1, y1) = (x1_y1[0], x1_y1[1]);
			let (x2, y2) = (x2_y2[0], x2_y2[1]);
			(
				(x1.parse().unwrap(), y1.parse().unwrap()),
				(x2.parse().unwrap(), y2.parse().unwrap()),
			)
		})
		.collect::<Vec<Line>>();
	let (width, height) = lines.iter().fold((0, 0), |acc, line| {
		(
			acc.0.max(line.0 .0).max(line.1 .0),
			acc.1.max(line.0 .1).max(line.1 .1),
		)
	});
	let width = width + 1;
	let height = height + 1;
	let mut field: Vec<Vec<i64>> = Vec::new();
	for i in 0..height {
		field.push(Vec::new());
		for j in 0..width {
			field[i].push(0);
		}
	}
	for line in lines {
		if line.0 .0 != line.1 .0 && line.0 .1 != line.1 .1 {
			continue;
		}
		if line.0 .0 == line.1 .0 {
			// Vertical
			let y1;
			let y2;
			if line.0 .1 < line.1 .1 {
				y1 = line.0 .1;
				y2 = line.1 .1;
			} else {
				y1 = line.1 .1;
				y2 = line.0 .1;
			}
			for y in y1..=y2 {
				field[y][line.0 .0] += 1;
			}
		} else {
			// Horizontal
			let x1;
			let x2;
			if line.0 .0 < line.1 .0 {
				x1 = line.0 .0;
				x2 = line.1 .0;
			} else {
				x1 = line.1 .0;
				x2 = line.0 .0;
			}
			for x in x1..=x2 {
				field[line.0 .1][x] += 1;
			}
		}
	}

	let mut overlaps = 0;
	for row in field.into_iter() {
		for point in row.into_iter() {
			if point > 1 {
				overlaps += 1;
			}
		}
	}
	overlaps
}

pub fn part2() -> i64 {
	let lines = read_lines("input/2021/5-2.txt");
	0
}
