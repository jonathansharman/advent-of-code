use aoc::{input, input::ParseGrid};

aoc::test::test_part!(test1, part1, 1569);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let forklifts = input!().parse_grid(|c| c == '@');
	forklifts
		.iter()
		.filter(|&(p, &f)| {
			f && forklifts.eight_neighbors(p).filter(|&(_, &n)| n).count() < 4
		})
		.count()
}

pub fn part2() -> usize {
	0
}
