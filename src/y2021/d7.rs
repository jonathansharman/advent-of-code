use crate::io::read_lines;

pub fn part1() -> i64 {
	let crabs = std::fs::read_to_string("input/2021/7.txt")
		.expect("error reading input")
		.split(',')
		.map(|crab| crab.parse::<i64>().expect("error parsing number"))
		.collect::<Vec<i64>>();
	crabs
		.iter()
		.map(|crab| {
			crabs
				.iter()
				.fold(0, |acc, crab2| acc + (crab - crab2).abs())
		})
		.min()
		.unwrap()
}

pub fn part2() -> i64 {
	let lines = read_lines("input/2021/7.txt");
	0
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(0, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(0, super::part2());
	}
}
