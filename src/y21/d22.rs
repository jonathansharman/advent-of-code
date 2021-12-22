use crate::io::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

crate::test::test_part!(test1, part1, 570915);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut cubes = HashMap::new();
	for step in read_reboot_steps() {
		for x in step.min[0].max(-50)..=step.max[0].min(50) {
			for y in step.min[1].max(-50)..=step.max[1].min(50) {
				for z in step.min[2].max(-50)..=step.max[2].min(50) {
					cubes.insert([x, y, z], step.on);
				}
			}
		}
	}
	cubes.into_values().filter(|&cube| cube).count()
}

pub fn part2() -> usize {
	read_lines("input/2021/22.txt").count()
}

struct RebootStep {
	on: bool,
	min: [i64; 3],
	max: [i64; 3],
}

fn read_reboot_steps() -> Vec<RebootStep> {
	read_lines("input/2021/22.txt")
		.map(|line| {
			let on = line.as_bytes()[1] == b'n';
			let x_start = line.find("x=").unwrap() + 2;
			let x_end = line.find(",y").unwrap();
			let y_start = line.find("y=").unwrap() + 2;
			let y_end = line.find(",z").unwrap();
			let z_start = line.find("z=").unwrap() + 2;
			let (x_min, x_max) = parse_range(&line[x_start..x_end]);
			let (y_min, y_max) = parse_range(&line[y_start..y_end]);
			let (z_min, z_max) = parse_range(&line[z_start..]);
			RebootStep {
				on,
				min: [x_min, y_min, z_min],
				max: [x_max, y_max, z_max],
			}
		})
		.collect()
}

fn parse_range(range: &str) -> (i64, i64) {
	range
		.split("..")
		.map(|n| n.parse().unwrap())
		.collect_tuple()
		.unwrap()
}
