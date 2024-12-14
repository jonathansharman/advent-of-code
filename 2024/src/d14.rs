use aoc::{define_point_and_vector, io::read_lines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 230435667);
aoc::test::test_part!(test2, part2, ?);

define_point_and_vector!(Point, Vector, x, y, i64);

fn parse_line(line: &str) -> (Point, Vector) {
	let (p, v) = line.split_once(' ').unwrap();
	(
		p[2..]
			.split(',')
			.map(|n| n.parse().unwrap())
			.collect_tuple::<(i64, i64)>()
			.unwrap()
			.into(),
		v[2..]
			.split(',')
			.map(|n| n.parse().unwrap())
			.collect_tuple::<(i64, i64)>()
			.unwrap()
			.into(),
	)
}

fn safety_factor(size: Vector, seconds: i64) -> usize {
	let (mut q0, mut q1, mut q2, mut q3) = (0, 0, 0, 0);
	read_lines("input/14.txt").for_each(|line| {
		let (p0, v) = parse_line(&line);
		let p = p0 + seconds * v;
		let px = ((p.x % size.x) + size.x) % size.x;
		let py = ((p.y % size.y) + size.y) % size.y;
		println!("{px} {py}");
		if py < size.y / 2 {
			if px < size.x / 2 {
				q0 += 1;
			} else if px > size.x / 2 {
				q1 += 1;
			}
		} else if py > size.y / 2 {
			if px < size.x / 2 {
				q2 += 1;
			} else if px > size.x / 2 {
				q3 += 1;
			}
		}
	});
	println!("{q0} {q1} {q2} {q3}");
	q0 * q1 * q2 * q3
}

pub fn part1() -> usize {
	safety_factor(Vector::new(101, 103), 100)
}

pub fn part2() -> usize {
	0
}
