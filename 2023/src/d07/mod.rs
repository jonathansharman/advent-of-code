use std::collections::HashMap;

use itertools::Itertools;

aoc::test::test_part!(test1, part1, 250474325);
aoc::test::test_part!(test2, part2, 248909434);


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Type {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

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

fn get_type(hand: &[Rank]) -> Type {
	let hand = hand
		.iter()
		.fold(HashMap::<Rank, usize>::new(), |mut acc, r| {
			*acc.entry(*r).or_default() += 1;
			acc
		});
	let counts = hand.values().copied().sorted().collect_vec();
	if counts == vec![5] {
		Type::FiveOfAKind
	} else if counts == vec![1, 4] {
		Type::FourOfAKind
	} else if counts == vec![2, 3] {
		Type::FullHouse
	} else if counts == vec![1, 1, 3] {
		Type::ThreeOfAKind
	} else if counts == vec![1, 2, 2] {
		Type::TwoPair
	} else if counts == vec![1, 1, 1, 2] {
		Type::OnePair
	} else {
		Type::HighCard
	}
}

pub fn part1() -> usize {
	let mut hands_and_bids = input!()
		.lines()
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

fn get_type2(hand: &[Rank2]) -> Type {
	let hand =
		hand.iter()
			.fold(HashMap::<Rank2, usize>::new(), |mut acc, r| {
				*acc.entry(*r).or_default() += 1;
				acc
			});
	let n_jokers = *hand.get(&Rank2::Joker).unwrap_or(&0);
	let counts = hand.values().copied().sorted().collect_vec();
	if counts == vec![5] {
		Type::FiveOfAKind
	} else if counts == vec![1, 4] {
		match n_jokers {
			0 => Type::FourOfAKind,
			_ => Type::FiveOfAKind,
		}
	} else if counts == vec![2, 3] {
		match n_jokers {
			0 => Type::FullHouse,
			_ => Type::FiveOfAKind,
		}
	} else if counts == vec![1, 1, 3] {
		match n_jokers {
			0 => Type::ThreeOfAKind,
			_ => Type::FourOfAKind,
		}
	} else if counts == vec![1, 2, 2] {
		match n_jokers {
			0 => Type::TwoPair,
			1 => Type::FullHouse,
			_ => Type::FourOfAKind,
		}
	} else if counts == vec![1, 1, 1, 2] {
		match n_jokers {
			0 => Type::OnePair,
			_ => Type::ThreeOfAKind,
		}
	} else {
		match n_jokers {
			0 => Type::HighCard,
			_ => Type::OnePair,
		}
	}
}

pub fn part2() -> usize {
	let mut hands_and_bids = input!()
		.lines()
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
