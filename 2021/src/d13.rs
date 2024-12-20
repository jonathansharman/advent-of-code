use itertools::Itertools;

use std::collections::HashSet;

aoc::test::test_part!(test1, part1, 743);
aoc::test::test_part!(
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

const INPUT: &str = include_str!("input/13.txt");

pub fn part1() -> usize {
	let mut lines = INPUT.lines();
	let points = read_points(&mut lines);
	apply_fold(&points, lines.next().unwrap()).len()
}

pub fn part2() -> String {
	let mut lines = INPUT.lines();
	let mut points = read_points(&mut lines);
	for fold in lines {
		points = apply_fold(&points, fold);
	}

	let (max_x, max_y) =
		points.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
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

fn read_points(
	lines: &mut impl Iterator<Item = &'static str>,
) -> HashSet<(i32, i32)> {
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

fn apply_fold(points: &HashSet<(i32, i32)>, fold: &str) -> HashSet<(i32, i32)> {
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
