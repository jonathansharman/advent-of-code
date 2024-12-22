use std::collections::HashMap;

use aoc::input;

aoc::test::test_part!(test1, part1, 194782);
aoc::test::test_part!(test2, part2, 233007586663131);

fn read_stones() -> HashMap<usize, usize> {
	input!()
		.lines()
		.next()
		.unwrap()
		.split_whitespace()
		.map(|n| n.parse().unwrap())
		.fold(HashMap::new(), |mut acc, n| {
			*acc.entry(n).or_default() += 1;
			acc
		})
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
	stones
		.into_iter()
		.flat_map(|(stone, count)| {
			let stones = if stone == 0 {
				vec![1]
			} else {
				let digit_count = stone.ilog10() + 1;
				if digit_count % 2 == 0 {
					let divisor = 10usize.pow(digit_count / 2);
					vec![stone / divisor, stone % divisor]
				} else {
					vec![2024 * stone]
				}
			};
			stones.into_iter().map(move |stone| (stone, count))
		})
		.fold(HashMap::new(), |mut acc, (stone, count)| {
			*acc.entry(stone).or_default() += count;
			acc
		})
}

fn solve(n: usize) -> usize {
	let mut stones = read_stones();
	for _ in 0..n {
		stones = blink(stones);
	}
	stones.into_values().sum()
}

pub fn part1() -> usize {
	solve(25)
}

pub fn part2() -> usize {
	solve(75)
}
