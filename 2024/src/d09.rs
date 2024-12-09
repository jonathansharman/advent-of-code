use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	// Load.
	let mut disk = Vec::new();
	let mut is_file = true;
	for (id, length) in read_lines("input/09.txt")
		.next()
		.unwrap()
		.bytes()
		.map(|b| (b - b'0') as usize)
		.enumerate()
	{
		disk.extend(vec![is_file.then_some(id / 2); length]);
		is_file = !is_file;
	}
	// Compact.
	let mut start = 0;
	let mut end = disk.len() - 1;
	'outer: loop {
		while disk[start].is_some() {
			start += 1;
			if start == end {
				break 'outer;
			}
		}
		while disk[end].is_none() {
			end -= 1;
			if start == end {
				break 'outer;
			}
		}
		disk.swap(start, end);
	}
	// Checksum.
	disk.into_iter()
		.enumerate()
		.filter_map(|(i, id)| id.map(|id| i * id))
		.sum()
}

pub fn part2() -> usize {
	0
}
