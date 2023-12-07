use std::collections::HashMap;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 250474325);
crate::test::test_part!(test2, part2, 248909434);

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
		// Five of a kind
		6
	} else if counts == vec![1, 4] {
		// Four of a kind
		5
	} else if counts == vec![2, 3] {
		// Full house
		4
	} else if counts == vec![1, 1, 3] {
		// Three of a kind
		3
	} else if counts == vec![1, 2, 2] {
		// Two pair
		2
	} else if counts == vec![1, 1, 1, 2] {
		// One pair
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Rank2 {
	Joker,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Queen,
	King,
	Ace,
}

fn get_type2(hand: &[Rank2]) -> usize {
	let hand =
		hand.iter()
			.fold(HashMap::<Rank2, usize>::new(), |mut acc, r| {
				*acc.entry(*r).or_default() += 1;
				acc
			});
	let n_jokers = *hand.get(&Rank2::Joker).unwrap_or(&0);
	let counts = hand.values().copied().sorted().collect_vec();
	if counts == vec![5] {
		// Five of a kind
		6
	} else if counts == vec![1, 4] {
		match n_jokers {
			// Four of a kind
			0 => 5,
			// Five of a kind
			_ => 6,
		}
	} else if counts == vec![2, 3] {
		match n_jokers {
			// Full house
			0 => 4,
			// Five of a kind
			_ => 6,
		}
	} else if counts == vec![1, 1, 3] {
		match n_jokers {
			// Three of a kind
			0 => 3,
			// Four of a kind
			_ => 5,
		}
	} else if counts == vec![1, 2, 2] {
		match n_jokers {
			// Two pair
			0 => 2,
			// Full house
			1 => 4,
			// Four of a kind
			_ => 5,
		}
	} else if counts == vec![1, 1, 1, 2] {
		match n_jokers {
			// One pair
			0 => 1,
			// Three of a kind
			_ => 3,
		}
	} else {
		match n_jokers {
			// High card
			0 => 0,
			// One pair
			_ => 1,
		}
	}
}

pub fn part2() -> usize {
	let mut hands_and_bids = read_lines("input/2023/07.txt")
		.map(|line| {
			let (hand, bid) = line.split_once(' ').unwrap();
			let hand = hand
				.chars()
				.map(|c| match c {
					'J' => Rank2::Joker,
					'2' => Rank2::Two,
					'3' => Rank2::Three,
					'4' => Rank2::Four,
					'5' => Rank2::Five,
					'6' => Rank2::Six,
					'7' => Rank2::Seven,
					'8' => Rank2::Eight,
					'9' => Rank2::Nine,
					'T' => Rank2::Ten,
					'Q' => Rank2::Queen,
					'K' => Rank2::King,
					'A' => Rank2::Ace,
					_ => unreachable!(),
				})
				.collect_vec();
			let bid: usize = bid.parse().unwrap();
			(hand, bid)
		})
		.collect::<Vec<_>>();
	hands_and_bids.sort_by(|(a, _), (b, _)| {
		get_type2(a).cmp(&get_type2(b)).then(a.cmp(b))
	});
	hands_and_bids
		.into_iter()
		.enumerate()
		.map(|(i, (_, bid))| (i + 1) * bid)
		.sum()
}
