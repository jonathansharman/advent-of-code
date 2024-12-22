aoc::test::test_part!(test1, part1, 1921197370);
aoc::test::test_part!(test2, part2, 1124);


fn prediction(history: Vec<i64>) -> i64 {
	let mut rows = vec![history];
	loop {
		let last = rows.last_mut().unwrap();
		if last.iter().all(|n| *n == 0) {
			last.push(0);
			break;
		}
		let mut next = Vec::new();
		for i in 0..last.len() - 1 {
			next.push(last[i + 1] - last[i]);
		}
		rows.push(next);
	}
	let n = rows.len();
	for i in (0..n - 1).rev() {
		let i_last = *rows[i].last().unwrap();
		let i_plus_1_last = *rows[i + 1].last().unwrap();
		rows[i].push(i_last + i_plus_1_last);
	}
	*rows[0].last().unwrap()
}

pub fn part1() -> i64 {
	input!()
		.lines()
		.map(|line| {
			let history = line
				.split_whitespace()
				.map(|n| n.parse().unwrap())
				.collect::<Vec<_>>();
			prediction(history)
		})
		.sum()
}

fn prediction2(history: Vec<i64>) -> i64 {
	let mut rows = vec![history];
	loop {
		let last = rows.last_mut().unwrap();
		if last.iter().all(|n| *n == 0) {
			last.insert(0, 0);
			break;
		}
		let mut next = Vec::new();
		for i in 0..last.len() - 1 {
			next.push(last[i + 1] - last[i]);
		}
		rows.push(next);
	}
	let n = rows.len();
	for i in (0..n - 1).rev() {
		let i_first = *rows[i].first().unwrap();
		let i_plus_1_first = *rows[i + 1].first().unwrap();
		rows[i].insert(0, i_first - i_plus_1_first);
	}
	*rows[0].first().unwrap()
}

pub fn part2() -> i64 {
	input!()
		.lines()
		.map(|line| {
			let history = line
				.split_whitespace()
				.map(|n| n.parse().unwrap())
				.collect::<Vec<_>>();
			prediction2(history)
		})
		.sum()
}
