use crate::io::read_lines;

crate::test::test_part!(test1, part1, 14720);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> i32 {
	let mut cycle = 1;
	let mut x = 1;
	let mut signal = 0;
	let mut tick = |cycle: &mut i32, x: &i32| {
		if [20, 60, 100, 140, 180, 220].contains(cycle) {
			signal += *cycle * x;
		}
		*cycle += 1;
	};
	for line in read_lines("input/2022/10.txt") {
		tick(&mut cycle, &x);
		if let Some(v) = line.strip_prefix("addx ") {
			tick(&mut cycle, &x);
			x += v.parse::<i32>().unwrap();
		}
	}
	signal
}

pub fn part2() -> usize {
	0
}
