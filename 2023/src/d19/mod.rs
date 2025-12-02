use std::{collections::HashMap, ops::RangeInclusive};

use aoc::input;

aoc::test::test_part!(test1, part1, 532551);
aoc::test::test_part!(test2, part2, 134343280273968);

pub fn part1() -> usize {
	let mut lines = input!().lines();
	let rules = read_rules(lines.by_ref());
	let parts = read_parts(lines);
	let mut result = 0;
	for part in parts {
		let mut state = "in".to_string();
		loop {
			match state.as_str() {
				"A" => {
					result += part.0.iter().sum::<usize>();
					break;
				}
				"R" => break,
				_ => {}
			}
			for rule in &rules[&state] {
				if let Some(cond) = &rule.cond {
					if cond.range.contains(&part.0[cond.index]) {
						state = rule.state.clone();
						break;
					}
				} else {
					state = rule.state.clone();
					break;
				}
			}
		}
	}
	result
}

pub fn part2() -> usize {
	let rules = read_rules(input!().lines().by_ref());
	combinations("in", &rules, Parts::new())
}

fn combinations(
	state: &str,
	rules: &HashMap<String, Vec<Rule>>,
	mut parts: Parts,
) -> usize {
	match state {
		"A" => parts.0.into_iter().map(|range| range.count()).product(),
		"R" => 0,
		_ => {
			let mut result = 0;
			for rule in &rules[state] {
				if let Some(cond) = &rule.cond {
					let (matched, unmatched) =
						parts.split(cond.index, &cond.range);
					if let Some(matched) = matched {
						result += combinations(&rule.state, rules, matched);
					}
					match unmatched {
						Some(unmatched) => parts = unmatched,
						None => {
							return result;
						}
					}
				} else {
					return result + combinations(&rule.state, rules, parts);
				}
			}
			panic!("no matches");
		}
	}
}

fn read_rules<'a>(
	lines: &mut impl Iterator<Item = &'a str>,
) -> HashMap<String, Vec<Rule>> {
	lines
		.take_while(|s| !s.is_empty())
		.map(|line| {
			let line = &line[..line.len() - 1];
			let (source, rest) = line.split_once('{').unwrap();
			let rest = rest.trim_end_matches('}');
			let workflows = rest
				.split(',')
				.map(|s| {
					if let Some((cond, target)) = s.split_once(':') {
						let index = match cond.chars().next().unwrap() {
							'x' => 0,
							'm' => 1,
							'a' => 2,
							's' => 3,
							_ => panic!("invalid category"),
						};
						let rhs = cond[2..].parse::<usize>().unwrap();
						let range = if cond.chars().nth(1).unwrap() == '<' {
							1..=rhs - 1
						} else {
							rhs + 1..=4000
						};
						Rule {
							cond: Some(Cond { index, range }),
							state: target.to_owned(),
						}
					} else {
						Rule {
							cond: None,
							state: s.to_owned(),
						}
					}
				})
				.collect();
			(source.to_owned(), workflows)
		})
		.collect()
}

fn read_parts<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Part> {
	lines
		.map(|line| {
			let line = line.trim_matches(&['{', '}'] as &[_]);
			let ratings = line
				.split(',')
				.map(|s| s[2..].parse().unwrap())
				.collect::<Vec<usize>>();
			Part(ratings.try_into().unwrap())
		})
		.collect()
}

#[derive(Debug)]
struct Rule {
	cond: Option<Cond>,
	state: String,
}

#[derive(Debug)]
struct Cond {
	index: usize,
	range: RangeInclusive<usize>,
}

struct Part([usize; 4]);

#[derive(Clone)]
struct Parts([RangeInclusive<usize>; 4]);

impl Parts {
	fn new() -> Parts {
		Parts(std::array::from_fn(|_| 1..=4000))
	}

	fn replace(mut self, index: usize, range: RangeInclusive<usize>) -> Parts {
		self.0[index] = range;
		self
	}

	fn split(
		&self,
		index: usize,
		splitter: &RangeInclusive<usize>,
	) -> (Option<Parts>, Option<Parts>) {
		let range = &self.0[index];
		let matched = intersection(range, splitter)
			.map(|range| self.clone().replace(index, range));
		let unmatched = intersection(range, &complement(splitter))
			.map(|range| self.clone().replace(index, range));
		(matched, unmatched)
	}
}

fn complement(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
	if *range.start() == 1 {
		range.end() + 1..=4000
	} else {
		1..=range.start() - 1
	}
}

fn intersection(
	range1: &RangeInclusive<usize>,
	range2: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
	let start = *range1.start().max(range2.start());
	let end = *range1.end().min(range2.end());
	if start <= end {
		Some(start..=end)
	} else {
		None
	}
}
