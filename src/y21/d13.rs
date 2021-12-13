use crate::io::read_lines;

use itertools::Itertools;

use std::collections::HashSet;

crate::test::test_part!(test1, part1, 743);
crate::test::test_part!(
	test2,
	part2,
	concat!(
		"###   ##  ###  #     ##  #  # #  # #   \n",
		"#  # #  # #  # #    #  # # #  #  # #   \n",
		"#  # #    #  # #    #  # ##   #### #   \n",
		"###  #    ###  #    #### # #  #  # #   \n",
		"# #  #  # #    #    #  # # #  #  # #   \n",
		"#  #  ##  #    #### #  # #  # #  # ####\n",
	)
);

pub fn part1() -> usize {
	let mut lines = read_lines("input/2021/13.txt");
	let points = read_points(&mut lines);
	apply_fold(&points, lines.next().unwrap()).len()
}

pub fn part2() -> String {
	let mut lines = read_lines("input/2021/13.txt");
	let mut points = read_points(&mut lines);
	for fold in lines {
		points = apply_fold(&points, fold);
	}

	let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
		(max_x.max(*x), max_y.max(*y))
	});

	let mut output = String::new();
	for y in 0..=max_y {
		for x in 0..=max_x {
			output.push(if points.contains(&(x, y)) { '#' } else { ' ' });
		}
		output.push('\n');
	}
	output
}

fn read_points(lines: &mut impl Iterator<Item = String>) -> HashSet<(i32, i32)> {
	let mut points = HashSet::new();
	for line in lines {
		if line.is_empty() {
			return points;
		}
		points.insert(
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap(),
		);
	}
	points
}

fn apply_fold(points: &HashSet<(i32, i32)>, fold: String) -> HashSet<(i32, i32)> {
	let (axis, fold_coord) = fold
		.trim_start_matches("fold along ")
		.split('=')
		.collect_tuple()
		.unwrap();
	let fold_coord: i32 = fold_coord.parse().unwrap();
	points
		.iter()
		.map(|(x, y)| {
			if axis == "y" && *y > fold_coord {
				(*x, 2 * fold_coord - y)
			} else if axis == "x" && *x > fold_coord {
				(2 * fold_coord - x, *y)
			} else {
				(*x, *y)
			}
		})
		.collect()
}
