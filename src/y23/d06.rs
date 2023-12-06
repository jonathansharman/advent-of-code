use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut lines = read_lines("input/2023/06.txt");
	let ts = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|n| n.parse().unwrap())
		.collect::<Vec<usize>>();
	let ds = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|n| n.parse().unwrap())
		.collect::<Vec<usize>>();
	ts.into_iter()
		.zip(ds.into_iter())
		.map(|(t, d)| (0..=t).filter(|hold| hold * (t - hold) > d).count())
		.product()
}

pub fn part2() -> usize {
	let lines = read_lines("input/2023/06.txt")
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
