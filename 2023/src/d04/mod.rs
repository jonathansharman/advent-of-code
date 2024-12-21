use std::collections::{BTreeMap, HashSet};

aoc::test::test_part!(test1, part1, 21959);
aoc::test::test_part!(test2, part2, 5132675);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	INPUT
		.lines()
		.map(|line| {
			let line = &line[line.find(':').unwrap() + 1..];
			let (winners, have) = line.split_once('|').unwrap();
			let winners: HashSet<usize> = HashSet::from_iter(
				winners
					.split_whitespace()
					.map(|s| s.parse::<usize>().unwrap()),
			);
			let n_wins = have
				.split_whitespace()
				.filter(|s| winners.contains(&s.parse().unwrap()))
				.count();
			if n_wins == 0 {
				0
			} else {
				2u32.pow((n_wins - 1) as u32) as usize
			}
		})
		.sum()
}

struct Card {
	winners: HashSet<usize>,
	have: Vec<usize>,
}

pub fn part2() -> usize {
	let cards = INPUT
		.lines()
		.map(|line| {
			let line = &line[line.find(':').unwrap() + 1..];
			let (winners, have) = line.split_once('|').unwrap();
			Card {
				winners: HashSet::from_iter(
					winners
						.split_whitespace()
						.map(|s| s.parse::<usize>().unwrap()),
				),
				have: have
					.split_whitespace()
					.map(|s| s.parse().unwrap())
					.collect(),
			}
		})
		.collect::<Vec<_>>();
	let mut queue: BTreeMap<usize, usize> =
		BTreeMap::from_iter((0..cards.len()).map(|idx| (idx, 1)));
	let mut n_cards = 0;
	while let Some((idx, count)) = queue.pop_first() {
		n_cards += count;
		let n_wins = cards[idx]
			.have
			.iter()
			.filter(|n| cards[idx].winners.contains(n))
			.count();
		for idx in idx + 1..idx + 1 + n_wins {
			*queue.entry(idx).or_default() += count;
		}
	}
	n_cards
}
