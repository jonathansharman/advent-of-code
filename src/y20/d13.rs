use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut lines = read_lines("input/2020/13.txt");
	let earliest: usize = lines.next().unwrap().parse().unwrap();
	let (wait, id) = lines
		.next()
		.unwrap()
		.split(',')
		.filter_map(|s| {
			s.parse::<usize>().ok().map(|id| (id - earliest % id, id))
		})
		.min_by(|a, b| a.0.cmp(&b.0))
		.unwrap();
	wait * id
}

pub fn part2() -> usize {
	0
}
