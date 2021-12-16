use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	let input = read_lines("input/2021/16.txt").next().unwrap();
	let bits: Vec<u8> = input
		.chars()
		.map(|c| {
			let mut hex = c.to_digit(16).unwrap();
			let mut bits = Vec::new();
			for _ in 0..4 {
				bits.insert(0, if hex & 1 == 1 { 1 } else { 0 });
				hex >>= 1;
			}
			bits
		})
		.reduce(|mut a, b| {
			a.extend(b.into_iter());
			a
		})
		.unwrap();
	parse_packet(&mut &bits[..]).version_sum()
}

pub fn part2() -> u32 {
	read_lines("input/2021/16.txt");
	0
}

struct Packet {
	version: u32,
	type_id: u32,
	contents: PacketContents,
}

enum PacketContents {
	Literal,
	Operator { subpackets: Vec<Packet> },
}

impl Packet {
	fn version_sum(&self) -> u32 {
		let contents_version_sum = match &self.contents {
			PacketContents::Literal => 0,
			PacketContents::Operator { subpackets } => {
				subpackets.iter().fold(0, |acc, s| acc + s.version_sum())
			}
		};
		self.version + contents_version_sum
	}
}

fn parse_number(bits: &mut &[u8], length: usize) -> u32 {
	let mut result = 0;
	for bit in bits[..length].iter() {
		result = (result << 1) + *bit as u32;
	}
	*bits = &bits[length..];
	result
}

fn parse_packet(bits: &mut &[u8]) -> Packet {
	let version = parse_number(bits, 3);
	let type_id = parse_number(bits, 3);
	let contents = if type_id == 4 {
		// Literal
		while parse_number(bits, 1) == 1 {
			parse_number(bits, 4);
		}
		parse_number(bits, 4);
		PacketContents::Literal
	} else {
		// Operator
		let length_type_id = parse_number(bits, 1);
		let mut subpackets = Vec::new();
		if length_type_id == 0 {
			// Length-based subpackets.
			let mut subpackets_length = parse_number(bits, 15);
			while subpackets_length > 0 {
				let initial_len = bits.len() as u32;
				subpackets.push(parse_packet(bits));
				subpackets_length -= initial_len - bits.len() as u32;
			}
		} else {
			// Count-based subpackets.
			let subpackets_count = parse_number(bits, 11);
			for _ in 0..subpackets_count {
				subpackets.push(parse_packet(bits))
			}
		}
		PacketContents::Operator { subpackets }
	};
	Packet {
		version,
		type_id,
		contents,
	}
}
