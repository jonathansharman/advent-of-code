use itertools::Itertools;
use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 5359);
aoc::test::test_part!(test2, part2, 12333);

const INPUT: &str = include_str!("input/20.txt");

pub fn part1() -> usize {
	enhance(2)
}

pub fn part2() -> usize {
	enhance(50)
}

fn enhance(iter_count: u32) -> usize {
	let mut lines = INPUT.lines();
	let algo = read_algo(&mut lines);
	lines.next();
	let mut image = read_image(&mut lines);
	for iter in 0..iter_count {
		// If the image enhancement algorithm maps 0 to on, then every other
		// image has an infinite number of light pixels. These must be
		// represented implicitly, based on the current iteration. If the
		// algorithm maps 0 to off, then unmapped pixels are simply off.
		let unmapped_is_on = algo[0] && iter % 2 == 1;

		let mut next_image = HashMap::new();
		for ((i, j), _) in image.iter() {
			for i in i - 1..=i + 1 {
				for j in j - 1..=j + 1 {
					let mut idx = 0;
					for k in i - 1..=i + 1 {
						for l in j - 1..=j + 1 {
							idx <<= 1;
							let on = match image.get(&(k, l)) {
								Some(on) => *on,
								None => unmapped_is_on,
							};
							if on {
								idx += 1;
							}
						}
					}
					next_image.insert((i, j), algo[idx]);
				}
			}
		}
		image = next_image;
	}
	image.into_values().filter(|&x| x).count()
}

fn read_algo(lines: &mut impl Iterator<Item = &'static str>) -> [bool; 512] {
	lines
		.next()
		.unwrap()
		.chars()
		.map(|c| c == '#')
		.collect_vec()
		.try_into()
		.unwrap()
}

fn read_image(
	lines: &mut impl Iterator<Item = &'static str>,
) -> HashMap<(i32, i32), bool> {
	let mut image = HashMap::new();
	for (i, line) in lines.enumerate() {
		for (j, c) in line.chars().enumerate() {
			image.insert((i as i32, j as i32), c == '#');
		}
	}
	image
}
