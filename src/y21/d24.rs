use crate::io::read_lines;
use itertools::Itertools;
use std::{
	fmt::{self, Display},
	time::Instant,
};

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> i64 {
	let start = Instant::now();
	let threads = (1..10)
		.map(|digit| {
			let mut input = [9; 14];
			input[0] = digit;
			std::thread::spawn(move || loop {
				if monad(input) == 0 {
					let answer = input
						.into_iter()
						.reduce(|acc, digit| 10 * acc + digit)
						.unwrap();
					println!("Found an answer: {}", answer);
					return answer;
				}
				if !decrement(&mut input) {
					return 0;
				}
			})
		})
		.collect_vec();
	let answers = threads
		.into_iter()
		.map(|thread| thread.join().unwrap())
		.collect_vec();
	println!("Finished searching in {} s", start.elapsed().as_secs_f64());
	for (i, answer) in answers.iter().enumerate() {
		println!("From thread {}: {}", i, answer);
	}
	answers.into_iter().max().unwrap()
}

#[test]
fn benchmark_part1() {
	let iters = 10_000_000;
	let start = Instant::now();
	let threads = (1..10)
		.map(|digit| {
			let mut input = [9; 14];
			input[0] = digit;
			std::thread::spawn(move || {
				for _ in 0..iters {
					if monad(input) == 0 {
						return input
							.into_iter()
							.reduce(|acc, digit| 10 * acc + digit)
							.unwrap();
					}
					if !decrement(&mut input) {
						return 0;
					}
				}
				0
			})
		})
		.collect_vec();
	let total_iters = threads.len() as u128 * iters;
	for (t, thread) in threads.into_iter().enumerate() {
		println!("{}: {}", t, thread.join().unwrap());
	}
	let elapsed = start.elapsed();
	println!(
		"{} checks in {} ms ({} ns / check)",
		total_iters,
		elapsed.as_millis(),
		elapsed.as_nanos() / total_iters,
	);
}

pub fn part2() -> u32 {
	0
}

#[must_use]
fn decrement(input: &mut [i64; 14]) -> bool {
	input[13] -= 1;
	for i in (1..input.len()).rev() {
		if input[i] == 0 {
			input[i] = 9;
			input[i - 1] -= 1;
		} else {
			break;
		}
	}
	input[0] > 0
}

fn increment(input: &mut [i64]) {
	*input.last_mut().unwrap() += 1;
	for i in (0..input.len()).rev() {
		if input[i] == 10 {
			input[i] = 1;
			input[i - 1] += 1;
		} else {
			break;
		}
	}
}

#[allow(clippy::many_single_char_names)]
fn monad(i: [i64; 14]) -> i64 {
	let mut w = 0;
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;

	w = i[0];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 15;
	y *= x;
	z += y;
	w = i[1];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 12;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 5;
	y *= x;
	z += y;
	w = i[2];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 13;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[3];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -14;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 7;
	y *= x;
	z += y;
	w = i[4];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 9;
	y *= x;
	z += y;
	w = i[5];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[6];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 14;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 14;
	y *= x;
	z += y;
	w = i[7];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 3;
	y *= x;
	z += y;
	w = i[8];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 1;
	y *= x;
	z += y;
	w = i[9];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 3;
	y *= x;
	z += y;
	w = i[10];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -8;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 4;
	y *= x;
	z += y;
	w = i[11];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[12];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -5;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 7;
	y *= x;
	z += y;
	w = i[13];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -10;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 1;
	y *= x;
	z += y;

	z
}

fn read_program() -> Vec<Instruction> {
	read_lines("input/2021/24.txt")
		// Extension to allow blank lines and // comments.
		.filter(|line| !line.is_empty() && !line.starts_with("//"))
		.map(|line| {
			let parts = line.split_whitespace().collect_vec();
			let opcode = parts[0];
			let v = Var::from(parts[1]);
			let get_arg = || Val::from(parts[2]);
			match opcode {
				"inp" => Instruction::Inp(v),
				"add" => Instruction::Add(v, get_arg()),
				"mul" => Instruction::Mul(v, get_arg()),
				"div" => Instruction::Div(v, get_arg()),
				"mod" => Instruction::Mod(v, get_arg()),
				"eql" => Instruction::Eql(v, get_arg()),
				_ => panic!("invalid operation"),
			}
		})
		.collect()
}

const W: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Var(usize);

impl From<&str> for Var {
	fn from(s: &str) -> Self {
		match s {
			"w" => Var(W),
			"x" => Var(X),
			"y" => Var(Y),
			"z" => Var(Z),
			_ => panic!("invalid variable"),
		}
	}
}

struct State {
	vars: [i64; 4],
}

impl Default for State {
	fn default() -> Self {
		Self { vars: [0; 4] }
	}
}

impl State {
	fn get(&self, value: Val) -> i64 {
		match value {
			Val::Var(v) => self.vars[v.0],
			Val::Literal(n) => n,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Val {
	Var(Var),
	Literal(i64),
}

impl From<&str> for Val {
	fn from(s: &str) -> Self {
		match s {
			"w" => Val::Var(Var(W)),
			"x" => Val::Var(Var(X)),
			"y" => Val::Var(Var(Y)),
			"z" => Val::Var(Var(Z)),
			n => Val::Literal(n.parse().unwrap()),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
	Inp(Var),
	Add(Var, Val),
	Mul(Var, Val),
	Div(Var, Val),
	Mod(Var, Val),
	Eql(Var, Val),
}

fn run(program: &[Instruction], mut input: &[i64]) -> State {
	let mut state = State::default();
	for instruction in program {
		match instruction {
			Instruction::Inp(v) => {
				state.vars[v.0] = input[0];
				input = &input[1..];
			}
			Instruction::Add(v, arg) => state.vars[v.0] = state.get(Val::Var(*v)) + state.get(*arg),
			Instruction::Mul(v, arg) => state.vars[v.0] = state.get(Val::Var(*v)) * state.get(*arg),
			Instruction::Div(v, arg) => state.vars[v.0] = state.get(Val::Var(*v)) / state.get(*arg),
			Instruction::Mod(v, arg) => state.vars[v.0] = state.get(Val::Var(*v)) % state.get(*arg),
			Instruction::Eql(v, arg) => {
				state.vars[v.0] = if state.get(Val::Var(*v)) == state.get(*arg) {
					1
				} else {
					0
				}
			}
		}
	}
	state
}

#[derive(Clone)]
enum Expr {
	Input(usize),
	Literal(i64),
	Add(Box<(Expr, Expr)>),
	Mul(Box<(Expr, Expr)>),
	Div(Box<(Expr, Expr)>),
	Mod(Box<(Expr, Expr)>),
	Eql(Box<(Expr, Expr)>),
}

impl Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Expr::Input(idx) => write!(f, "[{}]", idx),
			Expr::Literal(n) => write!(f, "{}", n),
			Expr::Add(add) => write!(f, "({} + {})", add.0, add.1),
			Expr::Mul(mul) => write!(f, "({} * {})", mul.0, mul.1),
			Expr::Div(div) => write!(f, "({} / {})", div.0, div.1),
			Expr::Mod(mod_) => write!(f, "({} % {})", mod_.0, mod_.1),
			Expr::Eql(eql) => write!(f, "({} = {})", eql.0, eql.1),
		}
	}
}

struct SymbolicState {
	vars: [Expr; 4],
	input_count: usize,
}

impl Default for SymbolicState {
	fn default() -> Self {
		Self {
			vars: [0; 4].map(Expr::Literal),
			input_count: 0,
		}
	}
}

impl SymbolicState {
	fn get(&self, value: Val) -> Expr {
		match value {
			Val::Var(v) => self.vars[v.0].clone(),
			Val::Literal(n) => Expr::Literal(n),
		}
	}
}

fn run_symbolically(program: &[Instruction]) -> SymbolicState {
	let mut state = SymbolicState::default();
	for instruction in program {
		match instruction {
			Instruction::Inp(v) => {
				state.vars[v.0] = Expr::Input(state.input_count);
				state.input_count += 1;
			}
			Instruction::Add(v, arg) => {
				state.vars[v.0] = match (state.get(Val::Var(*v)), state.get(*arg)) {
					(Expr::Literal(0), right) => right,
					(left, Expr::Literal(0)) => left,
					(left, right) => Expr::Add(Box::new((left, right))),
				};
			}
			Instruction::Mul(v, arg) => {
				state.vars[v.0] = match (state.get(Val::Var(*v)), state.get(*arg)) {
					(Expr::Literal(0), _) | (_, Expr::Literal(0)) => Expr::Literal(0),
					(Expr::Literal(1), right) => right,
					(left, Expr::Literal(1)) => left,
					(left, right) => Expr::Mul(Box::new((left, right))),
				};
			}
			Instruction::Div(v, arg) => {
				state.vars[v.0] = match (state.get(Val::Var(*v)), state.get(*arg)) {
					(Expr::Literal(0), _) => Expr::Literal(0),
					(left, Expr::Literal(1)) => left,
					(left, right) => Expr::Div(Box::new((left, right))),
				};
			}
			Instruction::Mod(v, arg) => {
				state.vars[v.0] = match (state.get(Val::Var(*v)), state.get(*arg)) {
					(Expr::Literal(0), _) => Expr::Literal(0),
					(_, Expr::Literal(1)) => Expr::Literal(0),
					(left, right) => Expr::Mod(Box::new((left, right))),
				};
			}
			Instruction::Eql(v, arg) => {
				state.vars[v.0] = Expr::Eql(Box::new((state.get(Val::Var(*v)), state.get(*arg))))
			}
		}
	}
	state
}
