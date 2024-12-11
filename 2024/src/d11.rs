use std::collections::HashMap;

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 194782);
aoc::test::test_part!(test2, part2, 233007586663131);

fn read_stones() -> HashMap<usize, usize> {
	read_lines("input/11.txt")
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
			let string = stone.to_string();
			let stones = match string.as_str() {
				"0" => vec![1],
				string if string.len() % 2 == 0 => {
					let (left, right) = string.split_at(string.len() / 2);
					vec![left.parse().unwrap(), right.parse().unwrap()]
				}
				_ => vec![2024 * stone],
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
