use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 45283905029161);
aoc::test::test_part!(test2, part2, ?);

enum Expr {
	Number(u64),
	Sum(Box<(Expr, Expr)>),
	Product(Box<(Expr, Expr)>),
}

fn parse(line: &str) -> Expr {
	parse_expr(&line.bytes().filter(|b| *b != b' ').collect::<Vec<_>>())
		.unwrap()
		.0
}

fn parse_expr(tokens: &[u8]) -> Option<(Expr, &[u8])> {
	let (mut expr, mut rest) = parse_term(tokens)?;
	while let Some((op, after_op)) = rest.split_first() {
		match op {
			b'+' => {
				let (term, after_term) = parse_term(after_op)?;
				expr = Expr::Sum(Box::new((expr, term)));
				rest = after_term;
			}
			b'*' => {
				let (term, after_term) = parse_term(after_op)?;
				expr = Expr::Product(Box::new((expr, term)));
				rest = after_term;
			}
			_ => break,
		}
	}
	Some((expr, rest))
}

fn parse_term(tokens: &[u8]) -> Option<(Expr, &[u8])> {
	let (first, rest) = tokens.split_first()?;
	match first {
		b'0'..=b'9' => Some((Expr::Number((*first - b'0').into()), rest)),
		b'(' => {
			let (expr, rest) = parse_expr(rest)?;
			let (close_paren, rest) = rest.split_first()?;
			(*close_paren == b')').then_some((expr, rest))
		}
		_ => None,
	}
}

fn evaluate(expr: &Expr) -> u64 {
	match expr {
		Expr::Number(n) => *n,
		Expr::Sum(sum) => evaluate(&sum.0) + evaluate(&sum.1),
		Expr::Product(product) => evaluate(&product.0) * evaluate(&product.1),
	}
}

pub fn part1() -> u64 {
	read_lines("input/18.txt")
		.map(|line| evaluate(&parse(&line)))
		.sum()
}

pub fn part2() -> usize {
	0
}
