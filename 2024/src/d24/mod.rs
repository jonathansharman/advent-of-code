use std::collections::HashMap;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 65635066541798);
aoc::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy)]
enum Op {
	And,
	Or,
	Xor,
}

#[derive(Clone)]
struct Gate {
	op: Op,
	in1: String,
	in2: String,
}

struct State {
	wires_on: HashMap<String, bool>,
	gates: HashMap<String, Gate>,
	output_bit_count: usize,
}

impl State {
	fn parse() -> State {
		let input = input!();
		let mut lines = input.lines();
		let wires_on = lines
			.by_ref()
			.map_while(|line| {
				let (wire, on) = line.split_once(": ")?;
				Some((wire.to_owned(), on == "1"))
			})
			.collect();
		let gates = lines
			.map(|line| {
				let (lhs, out) = line.split_once(" -> ").unwrap();
				let (in1, op, in2) =
					lhs.split_whitespace().collect_tuple().unwrap();
				let op = match op {
					"AND" => Op::And,
					"OR" => Op::Or,
					"XOR" => Op::Xor,
					_ => panic!("invalid op"),
				};
				(
					out.to_owned(),
					Gate {
						op,
						in1: in1.to_owned(),
						in2: in2.to_owned(),
					},
				)
			})
			.collect();
		let output_bit_count = input.chars().filter(|&c| c == 'z').count();
		State {
			wires_on,
			gates,
			output_bit_count,
		}
	}

	fn eval(&mut self, wire: &str) -> bool {
		if let Some(&wire_on) = self.wires_on.get(wire) {
			return wire_on;
		}
		let gate = self.gates[wire].clone();
		let in1 = self.eval(&gate.in1);
		let in2 = self.eval(&gate.in2);
		let wire_on = match gate.op {
			Op::And => in1 && in2,
			Op::Or => in1 || in2,
			Op::Xor => in1 ^ in2,
		};
		self.wires_on.insert(wire.to_owned(), wire_on);
		wire_on
	}
}

pub fn part1() -> usize {
	let mut state = State::parse();
	let mut answer = 0;
	for i in 0..state.output_bit_count {
		let wire = format!("z{:0>2}", state.output_bit_count - 1 - i);
		answer = (answer << 1) + state.eval(&wire) as usize;
	}
	answer
}

pub fn part2() -> usize {
	0
}
