aoc::test::test_part!(test1, part1, 449550);
aoc::test::test_part!(test2, part2, 28360140);


pub fn part1() -> usize {
	let lines = input!()
		.lines()
		.map(|line| {
			line.split_whitespace()
				.skip(1)
				.map(|n| n.parse().unwrap())
				.collect::<Vec<usize>>()
		})
		.collect::<Vec<_>>();
	let ts = &lines[0];
	let ds = &lines[1];
	ts.iter()
		.zip(ds.iter())
		.map(|(t, d)| (0..=*t).filter(|hold| hold * (t - hold) > *d).count())
		.product()
}

pub fn part2() -> usize {
	let lines = input!()
		.lines()
		.map(|line| {
			line.chars()
				.filter(char::is_ascii_digit)
				.collect::<String>()
				.parse()
				.unwrap()
		})
		.collect::<Vec<usize>>();
	let t = lines[0];
	let d = lines[1];
	(0..=t).filter(|hold| hold * (t - hold) > d).count()
}
