use itertools::Itertools;

use crate::io::read_lines;

use std::collections::HashSet;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let lines = read_lines("input/2021/13.txt").collect_vec();
	let points: HashSet<(i32, i32)> = lines
		.iter()
		.take_while(|line| !line.is_empty())
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect();
	if let Some(line) = lines.into_iter().skip_while(|line| !line.is_empty()).nth(1) {
		let (axis, fold_coord) = line
			.trim_start_matches("fold along ")
			.split('=')
			.collect_tuple()
			.unwrap();
		let fold_coord: i32 = fold_coord.parse().unwrap();
		points
			.iter()
			.map(|(x, y)| {
				if axis == "y" {
					if *y > fold_coord {
						if 2 * fold_coord - y < 0 {
							panic!();
						}
						(*x, 2 * fold_coord - y)
					} else {
						(*x, *y)
					}
				} else if *x > fold_coord {
					if 2 * fold_coord - x < 0 {
						panic!();
					}
					(2 * fold_coord - x, *y)
				} else {
					(*x, *y)
				}
			})
			.collect::<HashSet<(i32, i32)>>()
			.len()
	} else {
		0
	}
}

pub fn part2() {
	let lines = read_lines("input/2021/13.txt").collect_vec();
	let mut points: HashSet<(i32, i32)> = lines
		.iter()
		.take_while(|line| !line.is_empty())
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect();
	for line in lines
		.into_iter()
		.skip_while(|line| !line.is_empty())
		.skip(1)
	{
		let (axis, fold_coord) = line
			.trim_start_matches("fold along ")
			.split('=')
			.collect_tuple()
			.unwrap();
		let fold_coord: i32 = fold_coord.parse().unwrap();
		points = points
			.iter()
			.map(|(x, y)| {
				if axis == "y" {
					if *y > fold_coord {
						if 2 * fold_coord - y < 0 {
							panic!();
						}
						(*x, 2 * fold_coord - y)
					} else {
						(*x, *y)
					}
				} else if *x > fold_coord {
					if 2 * fold_coord - x < 0 {
						panic!();
					}
					(2 * fold_coord - x, *y)
				} else {
					(*x, *y)
				}
			})
			.collect::<HashSet<(i32, i32)>>();
	}

	let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
		(max_x.max(*x), max_y.max(*y))
	});

	for x in 0..=max_x {
		for y in 0..=max_y {
			print!("{}", if points.contains(&(x, y)) { '#' } else { '.' });
		}
		println!();
	}
}
