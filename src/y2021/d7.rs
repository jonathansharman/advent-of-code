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
	let crabs = std::fs::read_to_string("input/2021/7.txt")
		.expect("error reading input")
		.split(',')
		.map(|crab| crab.parse::<i64>().expect("error parsing number"))
		.collect::<Vec<i64>>();
	let min = *crabs.iter().min().unwrap();
	let max = *crabs.iter().max().unwrap();
	(min..=max)
		.map(|position| {
			crabs.iter().fold(0, |acc, crab| {
				let distance = (position - crab).abs();
				acc + distance * (distance + 1) / 2
			})
		})
		.min()
		.unwrap()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(343605, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(96744904, super::part2());
	}
}
