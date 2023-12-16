use crate::io::read_lines;

crate::test::test_part!(test1, part1, 55971);
crate::test::test_part!(test2, part2, 54719);

pub fn part1() -> usize {
	read_lines("input/2023/01.txt")
		.map(|line| {
			let digits = line
				.bytes()
				.filter_map(|c| {
					c.is_ascii_digit().then_some((c - b'0') as usize)
				})
				.collect::<Vec<_>>();
			10 * digits[0] + digits.last().unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	read_lines("input/2023/01.txt")
		.map(|line| 10 * first(line.as_bytes()) + last(line.as_bytes()))
		.sum()
}

fn first(mut s: &[u8]) -> usize {
	loop {
		if (b'1'..=b'9').contains(&s[0]) {
			return (s[0] - b'0') as usize;
		}
		if s.starts_with("one".as_bytes()) {
			return 1;
		}
		if s.starts_with("two".as_bytes()) {
			return 2;
		}
		if s.starts_with("three".as_bytes()) {
			return 3;
		}
		if s.starts_with("four".as_bytes()) {
			return 4;
		}
		if s.starts_with("five".as_bytes()) {
			return 5;
		}
		if s.starts_with("six".as_bytes()) {
			return 6;
		}
		if s.starts_with("seven".as_bytes()) {
			return 7;
		}
		if s.starts_with("eight".as_bytes()) {
			return 8;
		}
		if s.starts_with("nine".as_bytes()) {
			return 9;
		}
		s = &s[1..];
	}
}

fn last(mut s: &[u8]) -> usize {
	loop {
		let last_b = s.last().unwrap();
		if (b'1'..=b'9').contains(last_b) {
			return (last_b - b'0') as usize;
		}
		if s.ends_with("one".as_bytes()) {
			return 1;
		}
		if s.ends_with("two".as_bytes()) {
			return 2;
		}
		if s.ends_with("three".as_bytes()) {
			return 3;
		}
		if s.ends_with("four".as_bytes()) {
			return 4;
		}
		if s.ends_with("five".as_bytes()) {
			return 5;
		}
		if s.ends_with("six".as_bytes()) {
			return 6;
		}
		if s.ends_with("seven".as_bytes()) {
			return 7;
		}
		if s.ends_with("eight".as_bytes()) {
			return 8;
		}
		if s.ends_with("nine".as_bytes()) {
			return 9;
		}
		s = &s[..s.len() - 1];
	}
}
