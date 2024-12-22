use aoc::input;

aoc::test::test_part!(test1, part1, 14720);
aoc::test::test_part!(
	test2,
	part2,
	concat!(
		"\n####.####.###..###..###..####.####.####.",
		"\n#.......#.#..#.#..#.#..#.#.......#.#....",
		"\n###....#..###..#..#.###..###....#..###..",
		"\n#.....#...#..#.###..#..#.#.....#...#....",
		"\n#....#....#..#.#....#..#.#....#....#....",
		"\n#....####.###..#....###..#....####.#...."
	)
);

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
	for line in input!().lines() {
		tick(&mut cycle, &x);
		if let Some(v) = line.strip_prefix("addx ") {
			tick(&mut cycle, &x);
			x += v.parse::<i32>().unwrap();
		}
	}
	signal
}

pub fn part2() -> String {
	let mut display = String::new();
	let mut cycle = 1;
	let mut x = 1;
	let mut tick = |cycle: &mut i32, x: i32| {
		let col = (*cycle - 1) % 40;
		if col == 0 {
			display += "\n";
		}
		display += if col.abs_diff(x) <= 1 { "#" } else { "." };
		*cycle += 1;
	};
	for line in input!().lines() {
		tick(&mut cycle, x);
		if let Some(v) = line.strip_prefix("addx ") {
			tick(&mut cycle, x);
			x += v.parse::<i32>().unwrap();
		}
	}
	display
}
