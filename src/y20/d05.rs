use crate::io::read_lines;

crate::test::test_part!(test1, part1, 904);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	read_lines("input/2020/05.txt")
		.map(|line| Pass::from(line.as_bytes()).seat_id())
		.max()
		.unwrap_or_default()
}

pub fn part2() -> usize {
	read_lines("input/2020/05.txt").count()
}

struct Pass {
	row: u32,
	col: u32,
}

impl Pass {
	fn seat_id(&self) -> u32 {
		8 * self.row + self.col
	}
}

impl From<&[u8]> for Pass {
	fn from(bytes: &[u8]) -> Self {
		let row = bytes[0..7]
			.iter()
			.fold(0, |acc, &byte| 2 * acc + ((byte == b'B') as u32));
		let col = bytes[7..]
			.iter()
			.fold(0, |acc, &byte| 2 * acc + ((byte == b'R') as u32));
		Pass { row, col }
	}
}
