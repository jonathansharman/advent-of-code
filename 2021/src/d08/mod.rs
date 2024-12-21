use std::collections::{HashMap, HashSet};

aoc::test::test_part!(test1, part1, 367);
aoc::test::test_part!(test2, part2, 974512);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> i64 {
	INPUT
		.lines()
		.map(|line| -> i64 {
			line.split(" | ")
				.nth(1)
				.unwrap()
				.split_whitespace()
				.map(|word| match word.len() {
					2 | 3 | 4 | 7 => 1,
					_ => 0,
				})
				.sum()
		})
		.sum()
}

pub fn part2() -> i64 {
	INPUT
		.lines()
		.map(|line| -> i64 {
			let split = line
				.splitn(2, '|')
				.map(|part| {
					part.split_whitespace()
						.map(|word| word.chars().collect())
						.collect()
				})
				.collect::<Vec<Vec<HashSet<char>>>>();
			let patterns = &split[0];
			let outputs = &split[1];

			let mut signal_frequencies: HashMap<char, u8> = HashMap::new();
			for pattern in patterns {
				for &c in pattern {
					*signal_frequencies.entry(c).or_insert(0) += 1;
				}
			}

			let mut signal_map: HashMap<char, char> = HashMap::new();

			let mut digit_map: HashMap<i64, HashSet<char>> = HashMap::new();
			// Determine digits with unique length.
			digit_map.insert(
				1,
				patterns
					.iter()
					.find(|signals| signals.len() == 2)
					.unwrap()
					.clone(),
			);
			digit_map.insert(
				4,
				patterns
					.iter()
					.find(|signals| signals.len() == 4)
					.unwrap()
					.clone(),
			);
			digit_map.insert(
				7,
				patterns
					.iter()
					.find(|signals| signals.len() == 3)
					.unwrap()
					.clone(),
			);
			digit_map.insert(
				8,
				patterns
					.iter()
					.find(|signals| signals.len() == 7)
					.unwrap()
					.clone(),
			);
			// Determine signals with unique frequency.
			signal_map.insert(
				'e',
				*signal_frequencies.iter().find(|(_, &v)| v == 4).unwrap().0,
			);
			signal_map.insert(
				'f',
				*signal_frequencies.iter().find(|(_, &v)| v == 9).unwrap().0,
			);
			// '7' \ '1' = 'a'.
			signal_map.insert(
				'a',
				*digit_map[&7].difference(&digit_map[&1]).next().unwrap(),
			);
			// Only 'a' and 'c' occur 8 times.
			signal_map.insert(
				'c',
				*signal_frequencies
					.iter()
					.find(|(&k, &v)| k != signal_map[&'a'] && v == 8)
					.unwrap()
					.0,
			);
			// '8' \ 'e' = '9'.
			digit_map.insert(9, {
				let mut s = digit_map[&8].clone();
				s.remove(&signal_map[&'e']);
				s
			});
			// '8' \ 'c' = '6'.
			digit_map.insert(6, {
				let mut s = digit_map[&8].clone();
				s.remove(&signal_map[&'c']);
				s
			});
			// '6' \ 'e' = '5'.
			digit_map.insert(5, {
				let mut s = digit_map[&6].clone();
				s.remove(&signal_map[&'e']);
				s
			});
			// '0' contains both 'e' and 'f'.
			digit_map.insert(
				0,
				patterns
					.iter()
					.find(|signals| {
						!digit_map.iter().any(|(_, v)| &v == signals)
							&& signals.contains(&signal_map[&'e'])
							&& signals.contains(&signal_map[&'f'])
					})
					.unwrap()
					.clone(),
			);
			// '2' contains 'e' but not 'f'.
			digit_map.insert(
				2,
				patterns
					.iter()
					.find(|signals| {
						!digit_map.iter().any(|(_, v)| &v == signals)
							&& signals.contains(&signal_map[&'e'])
							&& !signals.contains(&signal_map[&'f'])
					})
					.unwrap()
					.clone(),
			);
			// '3' contains 'f' but not 'e'.
			digit_map.insert(
				3,
				patterns
					.iter()
					.find(|signals| {
						!digit_map.iter().any(|(_, v)| &v == signals)
							&& !signals.contains(&signal_map[&'e'])
							&& signals.contains(&signal_map[&'f'])
					})
					.unwrap()
					.clone(),
			);

			let get_digit = |signals: &HashSet<char>| {
				*digit_map.iter().find(|(_, v)| v == &signals).unwrap().0
			};
			outputs
				.iter()
				.fold(0, |acc, signals| 10 * acc + get_digit(signals))
		})
		.sum()
}
