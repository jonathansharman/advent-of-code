use std::collections::HashMap;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 250474325);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Rank {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

fn get_type(hand: &[Rank]) -> usize {
	let hand = hand
		.iter()
		.fold(HashMap::<Rank, usize>::new(), |mut acc, r| {
			*acc.entry(*r).or_default() += 1;
			acc
		});
	let counts = hand.values().copied().sorted().collect_vec();
	if counts == vec![5] {
		6
	} else if counts == vec![1, 4] {
		5
	} else if counts == vec![2, 3] {
		4
	} else if counts == vec![1, 1, 3] {
		3
	} else if counts == vec![1, 2, 2] {
		2
	} else if counts == vec![1, 1, 1, 2] {
		1
	} else {
		// High card
		0
	}
}

pub fn part1() -> usize {
	let mut hands_and_bids = read_lines("input/2023/07.txt")
		.map(|line| {
			let (hand, bid) = line.split_once(' ').unwrap();
			let hand = hand
				.chars()
				.map(|c| match c {
					'2' => Rank::Two,
					'3' => Rank::Three,
					'4' => Rank::Four,
					'5' => Rank::Five,
					'6' => Rank::Six,
					'7' => Rank::Seven,
					'8' => Rank::Eight,
					'9' => Rank::Nine,
					'T' => Rank::Ten,
					'J' => Rank::Jack,
					'Q' => Rank::Queen,
					'K' => Rank::King,
					'A' => Rank::Ace,
					_ => unreachable!(),
				})
				.collect_vec();
			let bid: usize = bid.parse().unwrap();
			(hand, bid)
		})
		.collect::<Vec<_>>();
	hands_and_bids
		.sort_by(|(a, _), (b, _)| get_type(a).cmp(&get_type(b)).then(a.cmp(b)));
	hands_and_bids
		.into_iter()
		.enumerate()
		.map(|(i, (_, bid))| (i + 1) * bid)
		.sum()
}

pub fn part2() -> usize {
	0
}
