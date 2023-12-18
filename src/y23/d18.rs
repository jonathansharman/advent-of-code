use std::collections::HashSet;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 36679);
crate::test::test_part!(test2, part2, 88007104020978);

#[derive(Clone, Copy)]
struct Instruction {
	dir: (i64, i64),
	len: i64,
}

impl Instruction {
	fn parse1(s: String) -> Instruction {
		let parts = s.split_whitespace().collect::<Vec<_>>();
		let dir = match parts[0] {
			"U" => (0, 1),
			"D" => (0, -1),
			"R" => (1, 0),
			"L" => (-1, 0),
			_ => panic!("invalid direction"),
		};
		let len = parts[1].parse().unwrap();
		Instruction { dir, len }
	}

	fn parse2(s: String) -> Instruction {
		let start = s.find('#').unwrap() + 1;
		let len = i64::from_str_radix(
			&s.chars().skip(start).take(5).collect::<String>(),
			16,
		)
		.unwrap();
		let dir = match s.chars().nth(start + 5).unwrap() {
			'0' => (1, 0),
			'1' => (0, -1),
			'2' => (-1, 0),
			'3' => (0, 1),
			_ => panic!("invalid direction"),
		};
		Instruction { dir, len }
	}
}

pub fn part1() -> usize {
	let instructions = read_lines("input/2023/18.txt")
		.map(Instruction::parse1)
		.collect::<Vec<_>>();
	// Compute 4 times the winding number (+/-1 since the close of the loop is
	// not included) to tell which side interior points are on.
	let winding: i64 = instructions
		.iter()
		.tuple_windows()
		.map(|(a, b)| a.dir.0 * b.dir.1 - a.dir.1 * b.dir.0)
		.sum();
	let mut coords = (0, 0);
	let mut holes = HashSet::new();
	let mut queue = Vec::new();
	// Form the outline and enqueue adjacent interior points.
	for i in instructions {
		// Offset = segment direction rotated in the direction of the loop.
		let offset = if winding > 0 {
			(-i.dir.1, i.dir.0)
		} else {
			(i.dir.1, -i.dir.0)
		};
		for _ in 0..i.len {
			coords.0 += i.dir.0;
			coords.1 += i.dir.1;
			holes.insert(coords);
			queue.push((coords.0 + offset.0, coords.1 + offset.1));
		}
	}
	// Fill the interior.
	while let Some((x, y)) = queue.pop() {
		if !holes.insert((x, y)) {
			continue;
		}
		queue.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
	}
	holes.len()
}

pub fn part2() -> i64 {
	let instructions = read_lines("input/2023/18.txt")
		.map(Instruction::parse2)
		.collect::<Vec<_>>();
	// Use the winding number to tell whether the loop is counter-clockwise.
	let winding_ccw = instructions
		.iter()
		.tuple_windows()
		.map(|(a, b)| a.dir.0 * b.dir.1 - a.dir.1 * b.dir.0)
		.sum::<i64>()
		> 0;
	let mut coords = (0, 0);
	let mut vertices = Vec::new();
	// Find the vertices of the exterior.
	for (i, a) in instructions.iter().copied().enumerate() {
		let b = instructions[(i + 1) % instructions.len()];
		let turn_ccw = (a.dir.0 * b.dir.1 - a.dir.1 * b.dir.0) > 0;
		coords.0 += a.len * a.dir.0;
		coords.1 += a.len * a.dir.1;
		let (x, y) = coords;
		// TODO: Find an expression to unify all these cases.
		let v = match (a.dir, winding_ccw, turn_ccw) {
			// Right, CW loop, right turn
			((1, 0), false, false) => (x + 1, y + 1),
			// Right, CW loop, left turn
			((1, 0), false, true) => (x, y + 1),
			// Right, CCW loop, right turn
			((1, 0), true, false) => (x, y),
			// Right, CCW loop, left turn
			((1, 0), true, true) => (x + 1, y),

			// Left, CW loop, right turn
			((-1, 0), false, false) => (x, y),
			// Left, CW loop, left turn
			((-1, 0), false, true) => (x + 1, y),
			// Left, CCW loop, right turn
			((-1, 0), true, false) => (x + 1, y + 1),
			// Left, CCW loop, left turn
			((-1, 0), true, true) => (x, y + 1),

			// Up, CW loop, right turn
			((0, 1), false, false) => (x, y + 1),
			// Up, CW loop, left turn
			((0, 1), false, true) => (x, y),
			// Up, CCW loop, right turn
			((0, 1), true, false) => (x + 1, y),
			// Up, CCW loop, left turn
			((0, 1), true, true) => (x + 1, y + 1),

			// Down, CW loop, right turn
			((0, -1), false, false) => (x + 1, y),
			// Down, CW loop, left turn
			((0, -1), false, true) => (x + 1, y + 1),
			// Down, CCW loop, right turn
			((0, -1), true, false) => (x, y + 1),
			// Down, CCW loop, left turn
			((0, -1), true, true) => (x, y),

			_ => unreachable!(),
		};
		vertices.push(v);
	}
	vertices.push(vertices[0]);
	// Use the shoelace formula to find the area.
	vertices
		.into_iter()
		.tuple_windows()
		.map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
		.sum::<i64>()
		.abs() / 2
}
