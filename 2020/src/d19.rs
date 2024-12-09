use std::collections::{HashMap, HashSet};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 220);
aoc::test::test_part!(test2, part2, 439);

#[derive(Clone)]
enum Rule {
	Literal(char),
	Concat(Vec<u8>),
	Alt((Vec<u8>, Vec<u8>)),
}

struct RuleSet {
	rules: HashMap<u8, Rule>,
	strings: HashMap<u8, HashSet<String>>,
}

impl RuleSet {
	fn get(&self, rule: &u8) -> &HashSet<String> {
		&self.strings[rule]
	}

	fn eval_concat(&mut self, rules: &[u8]) -> HashSet<String> {
		rules
			.iter()
			.map(|rule| {
				self.eval(rule);
				self.get(rule).clone()
			})
			.reduce(|left, right| {
				left.into_iter()
					.flat_map(|s1| {
						right
							.clone()
							.into_iter()
							.map(move |s2| format!("{s1}{s2}"))
					})
					.collect()
			})
			.unwrap()
	}

	fn eval(&mut self, rule: &u8) {
		let strings = match self.rules[rule].clone() {
			Rule::Literal(c) => HashSet::from([c.to_string()]),
			Rule::Concat(rules) => self.eval_concat(&rules),
			Rule::Alt((left, right)) => {
				let mut left = self.eval_concat(&left);
				let right = self.eval_concat(&right);
				left.extend(right);
				left
			}
		};
		self.strings.insert(*rule, strings);
	}
}

fn parse_list(s: &str) -> Vec<u8> {
	s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_rule_set(lines: &mut impl Iterator<Item = String>) -> RuleSet {
	RuleSet {
		rules: lines
			.map(|line| {
				let (lhs, rhs) = line.split_once(": ").unwrap();
				let rule = if rhs == "\"a\"" {
					Rule::Literal('a')
				} else if rhs == "\"b\"" {
					Rule::Literal('b')
				} else if let Some((left, right)) = rhs.split_once(" | ") {
					Rule::Alt((parse_list(left), parse_list(right)))
				} else {
					Rule::Concat(parse_list(rhs))
				};
				(lhs.parse().unwrap(), rule)
			})
			.collect(),
		strings: HashMap::new(),
	}
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/19.txt");
	let mut rule_set =
		parse_rule_set(&mut lines.by_ref().take_while(|line| !line.is_empty()));
	rule_set.eval(&0);
	let rule0 = rule_set.get(&0);
	lines.filter(|line| rule0.contains(line)).count()
}

pub fn part2() -> usize {
	let mut lines = read_lines("input/19.txt");
	let mut rule_set =
		parse_rule_set(&mut lines.by_ref().take_while(|line| !line.is_empty()));
	// Replacing rules 8 and 11, rule 0 is equivalent to: 0 -> 42+ 42{n} 31{n}.
	// Therefore, there's no need to evaluate 0, 8, or 11. Just evaluate 42 and
	// 31, and check for a valid pattern of 42s followed by 31s.
	rule_set.eval(&42);
	rule_set.eval(&31);
	let rule42 = rule_set.get(&42);
	let rule31 = rule_set.get(&31);
	lines
		.filter(|line| {
			let mut l = line.as_str();
			// Count the number of leading 42s.
			let mut n = 0;
			loop {
				let mut found = false;
				for pattern in rule42 {
					if let Some(rest) = l.strip_prefix(pattern) {
						l = rest;
						n += 1;
						found = true;
					}
				}
				if !found {
					if n == 0 {
						return false;
					} else {
						break;
					}
				}
			}
			// Now look for 31s: at least one but no more than n - 1.
			let mut m = 0;
			loop {
				let mut found = false;
				for pattern in rule31 {
					if let Some(rest) = l.strip_prefix(pattern) {
						l = rest;
						m += 1;
						found = true;
					}
				}
				if !found {
					return l.is_empty() && m > 0 && m < n;
				}
			}
		})
		.count()
}
