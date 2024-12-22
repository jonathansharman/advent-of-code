use aoc::input::{input, ParseLines};

aoc::test::test_part!(test1, part1, 14180628689);
aoc::test::test_part!(test2, part2, ?);

const MODULUS: usize = 16777216;

pub fn part1() -> usize {
	input!()
		.parse_lines()
		.map(|mut secret: usize| {
			for _ in 0..2000 {
				secret ^= 64 * secret;
				secret %= MODULUS;
				secret ^= secret / 32;
				secret %= MODULUS;
				secret ^= 2048 * secret;
				secret %= MODULUS;
			}
			secret
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
