use std::{collections::HashSet, ops::RangeInclusive};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 21071);
aoc::test::test_part!(test2, part2, 3429967441937);

struct Rule {
	field: String,
	ranges: Vec<RangeInclusive<usize>>,
}

fn parse_ticket(s: &str) -> Vec<usize> {
	s.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/16.txt");
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
					field: field.to_owned(),
					ranges,
				}
			})
		})
		.collect::<Vec<_>>();
	let nearby_tickets = lines
		.skip(4)
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
	let mut lines = read_lines("input/16.txt");
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
					field: field.to_owned(),
					ranges,
				}
			})
		})
		.collect::<Vec<_>>();
	let ticket = parse_ticket(&lines.by_ref().nth(1).unwrap());
	// Map each field index to its set of possible field labels.
	let all_fields = rules.iter().map(|rule| rule.field.clone()).collect();
	let mut possible_fields: Vec<HashSet<String>> =
		std::iter::repeat(all_fields).take(ticket.len()).collect();
	// Eliminate field possibilities using valid nearby tickets.
	for line in lines.skip(2) {
		let ticket = parse_ticket(&line);
		if !ticket.iter().all(|n| {
			rules
				.iter()
				.any(|rule| rule.ranges.iter().any(|range| range.contains(n)))
		}) {
			continue;
		}
		for (i, n) in ticket.into_iter().enumerate() {
			for rule in rules.iter() {
				if rule.ranges.iter().all(|range| !range.contains(&n)) {
					possible_fields[i].remove(&rule.field);
				}
			}
		}
	}
	// Iteratively remove singleton fields from all other sets until only
	// singleton possibility sets remain.
	let mut queue: Vec<String> = possible_fields
		.iter()
		.filter(|&fields| (fields.len() == 1))
		.map(|fields| fields.iter().next().unwrap().clone())
		.collect();
	while let Some(singleton) = queue.pop() {
		for fields in possible_fields.iter_mut() {
			if fields.len() > 1 {
				fields.remove(&singleton);
				if fields.len() == 1 {
					queue.push(fields.iter().next().unwrap().clone());
				}
			}
		}
	}
	// There's now only one possible field per position.
	let fields: Vec<String> = possible_fields
		.into_iter()
		.map(|set| set.into_iter().next().unwrap())
		.collect();
	ticket
		.into_iter()
		.enumerate()
		.filter_map(|(i, value)| {
			if fields[i].starts_with("departure") {
				Some(value)
			} else {
				None
			}
		})
		.take(6)
		.product()
}
