use regex::Regex;

aoc::test::test_part!(test1, part1, 170068701);
aoc::test::test_part!(test2, part2, 78683433);

const INPUT: &str = include_str!("input/03.txt");

pub fn part1() -> usize {
	let regex = Regex::new(r"mul\(([1-9]\d*),([1-9]\d*)\)").unwrap();
	INPUT
		.lines()
		.map(|line| {
			let mut sum = 0;
			for (_, [a, b]) in regex.captures_iter(line).map(|c| c.extract()) {
				let a: usize = a.parse().unwrap();
				let b: usize = b.parse().unwrap();
				sum += a * b;
			}
			sum
		})
		.sum()
}

pub fn part2() -> usize {
	let regex =
		Regex::new(r"do\(\)|don't\(\)|mul\(([1-9]\d*),([1-9]\d*)\)").unwrap();
	let mut enabled = true;
	INPUT
		.lines()
		.map(|line| {
			let mut sum = 0;
			for m in regex.captures_iter(line) {
				match &m[0] {
					"do()" => enabled = true,
					"don't()" => enabled = false,
					_ => {
						if enabled {
							let a: usize = m[1].parse().unwrap();
							let b: usize = m[2].parse().unwrap();
							sum += a * b;
						}
					}
				}
			}
			sum
		})
		.sum()
}
