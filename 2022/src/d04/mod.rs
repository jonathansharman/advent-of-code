use aoc::input;

aoc::test::test_part!(test1, part1, 540);
aoc::test::test_part!(test2, part2, 872);

pub fn part1() -> usize {
	input!()
		.lines()
		.filter(|line| {
			let bounds: Vec<u32> =
				line.split(['-', ',']).map(|n| n.parse().unwrap()).collect();
			let (a, b) = (bounds[0]..=bounds[1], bounds[2]..=bounds[3]);
			a.contains(b.start()) && a.contains(b.end())
				|| b.contains(a.start()) && b.contains(a.end())
		})
		.count()
}

pub fn part2() -> usize {
	input!()
		.lines()
		.filter(|line| {
			let bounds: Vec<u32> =
				line.split(['-', ',']).map(|n| n.parse().unwrap()).collect();
			let (a, b) = (bounds[0]..=bounds[1], bounds[2]..=bounds[3]);
			a.contains(b.start())
				|| a.contains(b.end())
				|| b.contains(a.start())
				|| b.contains(a.end())
		})
		.count()
}
