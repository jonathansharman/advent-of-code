use aoc::input;

aoc::test::test_part!(test1, part1, 1055);
aoc::test::test_part!(test2, part2, ?);

fn rotations() -> impl Iterator<Item = i32> {
	input!().lines().map(|s| {
		let bytes = s.as_bytes();
		let value =
			str::from_utf8(&bytes[1..]).unwrap().parse::<i32>().unwrap();
		if bytes[0] == b'L' { -value } else { value }
	})
}

const MOD: i32 = 100;

pub fn part1() -> usize {
	let mut dial = 50;
	let mut zeros = 0;
	for rotation in rotations() {
		dial += rotation;
		dial = (dial % MOD + MOD) % MOD;
		if dial == 0 {
			zeros += 1;
		}
	}
	zeros
}

pub fn part2() -> usize {
	0
}
