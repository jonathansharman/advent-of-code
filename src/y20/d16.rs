use std::ops::RangeInclusive;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 21071);
crate::test::test_part!(test2, part2, ?);

struct Rule {
	_field: String,
	ranges: Vec<RangeInclusive<usize>>,
}

fn parse_ticket(s: &str) -> Vec<usize> {
	s.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/2020/16.txt");
	let rules = lines
		.by_ref()
		.map_while(|line| {
			(!line.is_empty()).then(|| {
				let (field, rest) = line.split_once(": ").unwrap();
				let ranges = rest
					.split(" or ")
					.map(|range| {
						let (start, end) = range.split_once('-').unwrap();
						start.parse().unwrap()..=end.parse().unwrap()
					})
					.collect();
				Rule {
					_field: field.to_owned(),
					ranges,
				}
			})
		})
		.collect::<Vec<_>>();
	let _ticket = parse_ticket(&lines.by_ref().nth(1).unwrap());
	let nearby_tickets = lines
		.skip(2)
		.map(|line| parse_ticket(&line))
		.collect::<Vec<_>>();
	nearby_tickets
		.into_iter()
		.map(|nearby_ticket| {
			nearby_ticket
				.into_iter()
				.map(|n| {
					if rules.iter().any(|rule| {
						rule.ranges.iter().any(|range| range.contains(&n))
					}) {
						0
					} else {
						n
					}
				})
				.sum::<usize>()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
