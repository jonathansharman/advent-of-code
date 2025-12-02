use aoc::input;

aoc::test::test_part!(test1, part1, 2237);
aoc::test::test_part!(test2, part2, 66681);

struct Game {
	id: usize,
	draws: Vec<Draw>,
}

#[derive(Default)]
struct Draw {
	red: usize,
	green: usize,
	blue: usize,
}

impl Draw {
	fn power(&self) -> usize {
		self.red * self.green * self.blue
	}
}

fn parse_game(line: &str) -> Game {
	let (game_id, draws) = line.split_once(": ").unwrap();
	Game {
		id: game_id[5..].parse().unwrap(),
		draws: draws
			.split("; ")
			.map(|draw| {
				draw.split(", ").fold(Draw::default(), |mut draw, n_color| {
					let (n, color) = n_color.split_once(' ').unwrap();
					let n: usize = n.parse().unwrap();
					match color {
						"red" => draw.red += n,
						"green" => draw.green += n,
						"blue" => draw.blue += n,
						_ => panic!("invalid color"),
					}
					draw
				})
			})
			.collect(),
	}
}

pub fn part1() -> usize {
	input!()
		.lines()
		.filter_map(|line| {
			let game = parse_game(line);
			for draw in game.draws {
				if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
					return None;
				}
			}
			Some(game.id)
		})
		.sum()
}

pub fn part2() -> usize {
	input!()
		.lines()
		.map(|line| {
			let game = parse_game(line);
			let draw = game
				.draws
				.into_iter()
				.reduce(|acc, next| Draw {
					red: acc.red.max(next.red),
					green: acc.green.max(next.green),
					blue: acc.blue.max(next.blue),
				})
				.unwrap();
			draw.power()
		})
		.sum()
}
