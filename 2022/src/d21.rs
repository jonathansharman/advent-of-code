use std::collections::HashMap;

use itertools::Itertools;

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 93813115694560);
aoc::test::test_part!(test2, part2, 3910938071092);

enum Yell {
	Number(f64),
	Sum(String, String),
	Difference(String, String),
	Product(String, String),
	Quotient(String, String),
}

fn get_yells() -> HashMap<String, Yell> {
	read_lines("input/21.txt")
		.map(|line| {
			let (monkey, yell) = line.split_once(": ").unwrap();
			let monkey = monkey.to_owned();
			if let Ok(n) = yell.parse::<f64>() {
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
	cache: &mut HashMap<String, f64>,
	yells: &HashMap<String, Yell>,
	monkey: String,
) -> f64 {
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
	evaluate(&mut cache, &yells, "root".to_string()) as i64
}

pub fn part2() -> i64 {
	let mut yells = get_yells();
	let root_name = "root".to_string();
	let current_root_yell = yells.remove(&root_name).unwrap();
	yells.insert(
		root_name,
		match current_root_yell {
			Yell::Number(_) => panic!("Expected binary expression"),
			Yell::Sum(l, r) => Yell::Difference(l, r),
			difference @ Yell::Difference(_, _) => difference,
			Yell::Product(l, r) => Yell::Difference(l, r),
			Yell::Quotient(l, r) => Yell::Difference(l, r),
		},
	);
	// These bounds as well as the slope of the search space are hard-coded for
	// my input (which has a negative slope). A general solution would require
	// first inferring the upper and lower bounds and the slope.
	let mut min: i64 = 0;
	let mut max: i64 = 2 << 42;
	while min <= max {
		let humn = ((min + max) / 2) as f64;
		yells.insert("humn".to_string(), Yell::Number(humn));
		let mut cache = HashMap::new();
		let root = evaluate(&mut cache, &yells, "root".to_string());
		println!("{min}-{max}: {root}");
		match root.partial_cmp(&0.0).unwrap() {
			std::cmp::Ordering::Less => max = ((min + max) / 2) - 1,
			std::cmp::Ordering::Equal => return humn as i64,
			std::cmp::Ordering::Greater => min = ((min + max) / 2) + 1,
		}
	}
	panic!("Answer not found");
}
