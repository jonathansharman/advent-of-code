use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 36679);
crate::test::test_part!(test2, part2, 88007104020978);

pub fn part1() -> i64 {
	let instructions = read_lines("input/2023/18.txt")
		.map(parse1)
		.collect::<Vec<_>>();
	area(&instructions)
}

pub fn part2() -> i64 {
	let vectors = read_lines("input/2023/18.txt")
		.map(parse2)
		.collect::<Vec<_>>();
	area(&vectors)
}

fn parse1(s: String) -> (i64, i64) {
	let parts = s.split_whitespace().collect::<Vec<_>>();
	let len = parts[1].parse().unwrap();
	match parts[0] {
		"U" => (0, len),
		"D" => (0, -len),
		"R" => (len, 0),
		"L" => (-len, 0),
		_ => panic!("invalid direction"),
	}
}

fn parse2(s: String) -> (i64, i64) {
	let start = s.find('#').unwrap() + 1;
	let len = i64::from_str_radix(
		&s.chars().skip(start).take(5).collect::<String>(),
		16,
	)
	.unwrap();
	match s.chars().nth(start + 5).unwrap() {
		'0' => (len, 0),
		'1' => (0, -len),
		'2' => (-len, 0),
		'3' => (0, len),
		_ => panic!("invalid direction"),
	}
}

fn area(vectors: &[(i64, i64)]) -> i64 {
	// Use the shoelace formula to find the area of the polygon with vertices
	// centered on the holes.
	let a = vectors
		.iter()
		.copied()
		.fold(vec![(0, 0)], |mut acc, (vx, vy)| {
			let (x, y) = acc.last().unwrap();
			acc.push((x + vx, y + vy));
			acc
		})
		.into_iter()
		.tuple_windows()
		.map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
		.sum::<i64>()
		.abs() / 2;

	// Pick's theorem states that for a polygon whose vertices all have integer
	// coordinates, A = i + b/2 - 1, where:
	//   1. A is the area of the polygon.
	//   2. i is the number of interior points with integer coordinates.
	//   3. b is the number of boundary points with integer coordinates.
	//
	// The area of the polygon enclosing the holes (as opposed to just passing
	// through their centers) is equal to the sum of the interior and boundary
	// points with integer coordinates of the smaller polygon. Rearranging
	// Pick's theorem, we have i + b = A + b/2 + 1. We have computed A and can
	// compute b as the perimeter of the smaller polygon. (There's no need to
	// compute i explicitly.)
	let b: i64 = vectors.iter().map(|v| v.0.abs() + v.1.abs()).sum();
	a + b / 2 + 1
}
