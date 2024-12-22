use aoc::input;

aoc::test::test_part!(test1, part1, 901);
aoc::test::test_part!(test2, part2, 110434737925);

pub fn part1() -> u64 {
	parse_packet(&mut &read_bits()[..]).version_sum()
}

pub fn part2() -> u64 {
	parse_packet(&mut &read_bits()[..]).eval()
}

fn read_bits() -> Vec<u8> {
	input!()
		.lines()
		.next()
		.unwrap()
		.chars()
		.map(|c| {
			let mut hex = c.to_digit(16).unwrap() as u8;
			let mut bits = Vec::new();
			for _ in 0..4 {
				bits.insert(0, hex & 1);
				hex >>= 1;
			}
			bits
		})
		.reduce(|mut a, b| {
			a.extend(b);
			a
		})
		.unwrap()
}

struct Packet {
	version: u64,
	contents: PacketContents,
}

enum PacketContents {
	Literal {
		value: u64,
	},
	Operation {
		operator: Operator,
		args: Vec<Packet>,
	},
}

enum Operator {
	Sum,
	Product,
	Minimum,
	Maximum,
	GreaterThan,
	LessThan,
	EqualTo,
}

impl Packet {
	fn version_sum(&self) -> u64 {
		let contents_version_sum = match &self.contents {
			PacketContents::Literal { .. } => 0,
			PacketContents::Operation { args, .. } => {
				args.iter().map(Packet::version_sum).sum()
			}
		};
		self.version + contents_version_sum
	}

	fn eval(&self) -> u64 {
		match &self.contents {
			PacketContents::Literal { value } => *value,
			PacketContents::Operation { operator, args } => match operator {
				Operator::Sum => args.iter().map(Packet::eval).sum(),
				Operator::Product => args.iter().map(Packet::eval).product(),
				Operator::Minimum => {
					args.iter().map(Packet::eval).min().unwrap()
				}
				Operator::Maximum => {
					args.iter().map(Packet::eval).max().unwrap()
				}
				Operator::GreaterThan => {
					(args[0].eval() > args[1].eval()) as u64
				}
				Operator::LessThan => (args[0].eval() < args[1].eval()) as u64,
				Operator::EqualTo => (args[0].eval() == args[1].eval()) as u64,
			},
		}
	}
}

fn parse_number(bits: &mut &[u8], length: usize) -> u64 {
	let mut result = 0;
	for bit in bits[..length].iter() {
		result = (result << 1) + *bit as u64;
	}
	*bits = &bits[length..];
	result
}

fn parse_packet(bits: &mut &[u8]) -> Packet {
	let version = parse_number(bits, 3);
	let type_id = parse_number(bits, 3);
	let contents = if type_id == 4 {
		// Literal
		let mut value = 0;
		while parse_number(bits, 1) == 1 {
			value = (value << 4) + parse_number(bits, 4);
		}
		value = (value << 4) + parse_number(bits, 4);
		PacketContents::Literal { value }
	} else {
		// Operation
		let operator = match type_id {
			0 => Operator::Sum,
			1 => Operator::Product,
			2 => Operator::Minimum,
			3 => Operator::Maximum,
			5 => Operator::GreaterThan,
			6 => Operator::LessThan,
			7 => Operator::EqualTo,
			_ => panic!("invalid type_id"),
		};
		let length_type_id = parse_number(bits, 1);
		let mut args = Vec::new();
		if length_type_id == 0 {
			// Length-based subpackets.
			let mut subpackets_length = parse_number(bits, 15);
			while subpackets_length > 0 {
				let initial_len = bits.len() as u64;
				args.push(parse_packet(bits));
				subpackets_length -= initial_len - bits.len() as u64;
			}
		} else {
			// Count-based subpackets.
			let subpackets_count = parse_number(bits, 11);
			for _ in 0..subpackets_count {
				args.push(parse_packet(bits))
			}
		}
		PacketContents::Operation { operator, args }
	};
	Packet { version, contents }
}
