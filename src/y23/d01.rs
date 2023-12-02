use crate::io::read_lines;

crate::test::test_part!(test1, part1, 55971);
crate::test::test_part!(test2, part2, 54719);

pub fn part1() -> usize {
	read_lines("input/2023/01.txt")
		.map(|line| {
			let digits = line
				.bytes()
				.filter_map(|c| {
					c.is_ascii_digit().then_some((c - b'0') as usize)
				})
				.collect::<Vec<_>>();
			10 * digits[0] + digits.last().unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	read_lines("input/2023/01.txt")
		.map(|line| {
			let regex = regex::Regex::new(
				"[1-9]|one|two|three|four|five|six|seven|eight|nine|zero",
			)
			.unwrap();
			let first = value(regex.find(&line).unwrap().as_str());

			let rev_regex = regex::Regex::new(
				"[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez",
			)
			.unwrap();
			let rev_line = line.chars().rev().collect::<String>();
			let rev_last_match = rev_regex.find(&rev_line).unwrap().as_str();
			let last_match = rev_last_match.chars().rev().collect::<String>();
			let last = value(&last_match);

			10 * first + last
		})
		.sum()
}

fn value(s: &str) -> usize {
	match s {
		"1" | "one" => 1,
		"2" | "two" => 2,
		"3" | "three" => 3,
		"4" | "four" => 4,
		"5" | "five" => 5,
		"6" | "six" => 6,
		"7" | "seven" => 7,
		"8" | "eight" => 8,
		"9" | "nine" => 9,
		_ => panic!(),
	}
}
