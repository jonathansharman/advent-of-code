use std::collections::HashMap;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

enum Yell {
	Number(i64),
	Sum(String, String),
	Difference(String, String),
	Product(String, String),
	Quotient(String, String),
}

fn get_yells() -> HashMap<String, Yell> {
	read_lines("input/2022/21.txt")
		.map(|line| {
			let (monkey, yell) = line.split_once(": ").unwrap();
			let monkey = monkey.to_owned();
			if let Ok(n) = yell.parse::<i64>() {
				(monkey, Yell::Number(n))
			} else {
				let (l, op, r) = yell.split(' ').collect_tuple().unwrap();
				let (l, r) = (l.to_owned(), r.to_owned());
				match op {
					"+" => (monkey, Yell::Sum(l, r)),
					"-" => (monkey, Yell::Difference(l, r)),
					"*" => (monkey, Yell::Product(l, r)),
					"/" => (monkey, Yell::Quotient(l, r)),
					_ => panic!("Invalid operator"),
				}
			}
		})
		.collect()
}

fn evaluate(
	cache: &mut HashMap<String, i64>,
	yells: &HashMap<String, Yell>,
	monkey: String,
) -> i64 {
	if let Some(n) = cache.get(&monkey) {
		return *n;
	}
	let n = match yells.get(&monkey).unwrap() {
		Yell::Number(n) => *n,
		Yell::Sum(l, r) => {
			evaluate(cache, yells, l.clone())
				+ evaluate(cache, yells, r.clone())
		}
		Yell::Difference(l, r) => {
			evaluate(cache, yells, l.clone())
				- evaluate(cache, yells, r.clone())
		}
		Yell::Product(l, r) => {
			evaluate(cache, yells, l.clone())
				* evaluate(cache, yells, r.clone())
		}
		Yell::Quotient(l, r) => {
			evaluate(cache, yells, l.clone())
				/ evaluate(cache, yells, r.clone())
		}
	};
	cache.insert(monkey, n);
	n
}

pub fn part1() -> i64 {
	let yells = get_yells();
	let mut cache = HashMap::new();
	evaluate(&mut cache, &yells, "root".to_string())
}

pub fn part2() -> usize {
	0
}
