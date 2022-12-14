use std::collections::HashSet;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 592);
crate::test::test_part!(test2, part2, 30367);

fn merge(
	acc: HashSet<(i32, i32)>,
	next: HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
	acc.union(&next).cloned().collect()
}

pub fn part1() -> usize {
	let mut walls = read_lines("input/2022/14.txt")
		.map(|line| {
			line.split(" -> ")
				.map(|coords| {
					coords
						.split(',')
						.map(|s| s.parse::<i32>().unwrap())
						.collect_tuple()
						.unwrap()
				})
				.tuple_windows()
				.map(|((x1, y1), (x2, y2))| {
					let mut walls = HashSet::new();
					if x1 == x2 {
						for y in y1.min(y2)..=y1.max(y2) {
							walls.insert((x1, y));
						}
					} else {
						for x in x1.min(x2)..=x1.max(x2) {
							walls.insert((x, y1));
						}
					}
					walls
				})
				.reduce(merge)
				.unwrap()
		})
		.reduce(merge)
		.unwrap();
	let walls_len_start = walls.len();
	let ymax = walls.iter().max_by_key(|(_, y)| y).unwrap().1;
	'outer: loop {
		let (mut x, mut y) = (500, 0);
		loop {
			if walls.get(&(x, y + 1)).is_none() {
				y += 1;
			} else if walls.get(&(x - 1, y + 1)).is_none() {
				y += 1;
				x -= 1;
			} else if walls.get(&(x + 1, y + 1)).is_none() {
				y += 1;
				x += 1;
			} else {
				walls.insert((x, y));
				break;
			}
			if y > ymax {
				break 'outer;
			}
		}
	}
	walls.len() - walls_len_start
}

pub fn part2() -> usize {
	let mut walls = read_lines("input/2022/14.txt")
		.map(|line| {
			line.split(" -> ")
				.map(|coords| {
					coords
						.split(',')
						.map(|s| s.parse::<i32>().unwrap())
						.collect_tuple()
						.unwrap()
				})
				.tuple_windows()
				.map(|((x1, y1), (x2, y2))| {
					let mut walls = HashSet::new();
					if x1 == x2 {
						for y in y1.min(y2)..=y1.max(y2) {
							walls.insert((x1, y));
						}
					} else {
						for x in x1.min(x2)..=x1.max(x2) {
							walls.insert((x, y1));
						}
					}
					walls
				})
				.reduce(merge)
				.unwrap()
		})
		.reduce(merge)
		.unwrap();
	let walls_len_start = walls.len();
	let ymax = walls.iter().max_by_key(|(_, y)| y).unwrap().1;
	'outer: loop {
		let (mut x, mut y) = (500, 0);
		loop {
			if y == ymax + 1 {
				walls.insert((x, y));
				break;
			} else if walls.get(&(x, y + 1)).is_none() {
				y += 1;
			} else if walls.get(&(x - 1, y + 1)).is_none() {
				y += 1;
				x -= 1;
			} else if walls.get(&(x + 1, y + 1)).is_none() {
				y += 1;
				x += 1;
			} else {
				walls.insert((x, y));
				if (x, y) == (500, 0) {
					break 'outer;
				}
				break;
			}
		}
	}
	walls.len() - walls_len_start
}
