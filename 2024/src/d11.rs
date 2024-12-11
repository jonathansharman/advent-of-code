use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 194782);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut stones: Vec<usize> = read_lines("input/11.txt")
		.next()
		.unwrap()
		.split_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();
	for _ in 0..25 {
		stones = stones
			.into_iter()
			.flat_map(|stone| {
				let string = stone.to_string();
				match string.as_str() {
					"0" => vec![1],
					string if string.len() % 2 == 0 => {
						let (left, right) = string.split_at(string.len() / 2);
						vec![left.parse().unwrap(), right.parse().unwrap()]
					}
					_ => vec![2024 * stone],
				}
			})
			.collect();
	}
	stones.len()
}

pub fn part2() -> usize {
	0
}
