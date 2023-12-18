use std::collections::HashSet;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 36679);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone)]
struct Instruction {
	dir: (i32, i32),
	len: i32,
	_color: String,
}

impl Instruction {
	fn parse(s: String) -> Instruction {
		let parts = s.split_whitespace().collect::<Vec<_>>();
		let dir = match parts[0] {
			"U" => (0, 1),
			"D" => (0, -1),
			"R" => (1, 0),
			"L" => (-1, 0),
			_ => panic!("invalid direction"),
		};
		let len = parts[1].parse().unwrap();
		let _color = parts[2][2..parts[2].len() - 1].to_owned();
		Instruction { dir, len, _color }
	}
}

#[allow(unused)]
fn print_holes(holes: &HashSet<(i32, i32)>) {
	let min_x = *holes.iter().map(|(x, _)| x).min().unwrap();
	let max_x = *holes.iter().map(|(x, _)| x).max().unwrap();
	let min_y = *holes.iter().map(|(_, y)| y).min().unwrap();
	let max_y = *holes.iter().map(|(_, y)| y).max().unwrap();
	let width = max_x - min_x + 1;
	let height = max_y - min_y + 1;
	for y in (min_y..=max_y).rev() {
		for x in min_x..=max_x {
			if holes.contains(&(x, y)) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!();
	}
}

pub fn part1() -> usize {
	let instructions = read_lines("input/2023/18.txt")
		.map(Instruction::parse)
		.collect::<Vec<_>>();
	// Compute 4 times the winding number (+/-1 since the close of the loop is
	// not included) to tell which side interior points are on.
	let winding: i32 = instructions
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

pub fn part2() -> usize {
	0
}
