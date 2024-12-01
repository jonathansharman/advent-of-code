use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	let pairs = read_lines("input/01.txt")
		.map(|line| {
			let parts = line.split_whitespace().collect::<Vec<_>>();
			(
				parts[0].parse::<i32>().unwrap(),
				parts[1].parse::<i32>().unwrap(),
			)
		})
		.collect::<Vec<_>>();
	let mut first = pairs.iter().map(|(a, _)| a).copied().collect::<Vec<_>>();
	first.sort();
	let mut second = pairs.iter().map(|(_, b)| b).copied().collect::<Vec<_>>();
	second.sort();
	first
		.into_iter()
		.zip(second)
		.map(|(a, b)| a.abs_diff(b))
		.sum()
}

pub fn part2() -> usize {
	read_lines("input/01.txt").count()
}
