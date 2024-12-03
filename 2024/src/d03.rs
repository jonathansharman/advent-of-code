use aoc::io::read_lines;
use regex::Regex;

aoc::test::test_part!(test1, part1, 170068701);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let regex = Regex::new(r"mul\(([1-9]\d*),([1-9]\d*)\)").unwrap();
	read_lines("input/03.txt")
		.map(|line| {
			let mut sum = 0;
			for (_, [a, b]) in regex.captures_iter(&line).map(|c| c.extract()) {
				let a: usize = a.parse().unwrap();
				let b: usize = b.parse().unwrap();
				sum += a * b;
			}
			sum
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
