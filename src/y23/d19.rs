use std::{cmp::Ordering, collections::HashMap};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 532551);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let (rules, parts) = read_rules_and_parts();
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
					if part.0[cond.index].cmp(&cond.rhs) == cond.ord {
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
	0
}

fn read_rules_and_parts() -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
	let mut lines = read_lines("input/2023/d19.txt");
	let mut rules = HashMap::new();
	for line in lines.by_ref().take_while(|s| !s.is_empty()) {
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
					let ord = if cond.chars().nth(1).unwrap() == '<' {
						Ordering::Less
					} else {
						Ordering::Greater
					};
					let rhs = cond[2..].parse::<usize>().unwrap();
					Rule {
						cond: Some(Cond { index, ord, rhs }),
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
		rules.insert(source.to_owned(), workflows);
	}
	let parts = lines
		.map(|line| {
			let line = line.trim_matches(&['{', '}'] as &[_]);
			let ratings = line
				.split(',')
				.map(|s| s[2..].parse().unwrap())
				.collect::<Vec<usize>>();
			Part(ratings.try_into().unwrap())
		})
		.collect();
	(rules, parts)
}

#[derive(Debug)]
struct Rule {
	cond: Option<Cond>,
	state: String,
}

#[derive(Debug)]
struct Cond {
	index: usize,
	ord: Ordering,
	rhs: usize,
}

struct Part([usize; 4]);
