use bitvec::prelude::*;
use std::fmt::Debug;
use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Packet {
    packet_version: u8,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    VecOperator(VecOp, Vec<Packet>),
    BinaryOperator(BinaryOp, Box<Packet>, Box<Packet>),
}

#[derive(Debug)]
enum VecOp {
    Sum,
    Product,
    Minimum,
    Maximum,
}

#[derive(Debug)]
enum BinaryOp {
    GreaterThan,
    LessThan,
    EqualTo,
}

fn main() -> Result<()> {
    let input = read_input()?;
    let bits = input.view_bits::<Msb0>();
    let packet = parse_packet(&mut &bits[..])?;
    println!("sum versions = {}", sum_versions(&packet));
    println!("result = {}", eval(&packet));

    Ok(())
}

fn read_input() -> Result<Vec<u8>> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;
    let input = input.trim();
    let mut offset = 0;
    let mut bytes = Vec::new();
    while offset < input.len() {
        let byte_hex = &input[offset..offset + 2];
        assert_eq!(byte_hex.len(), 2);
        bytes.push(u8::from_str_radix(byte_hex, 16)?);
        offset += 2;
    }
    Ok(bytes)
}

fn parse_packet(bits: &mut &BitSlice<Msb0, u8>) -> Result<Packet> {
    let mut take_bits = |count: usize| {
        let head = &bits[..count];
        *bits = &bits[count..];
        head
    };
    let packet_version: u8 = take_bits(3).load_be();
    let packet_type_id: u8 = take_bits(3).load_be();
    let packet = if packet_type_id == 4 {
        // literal
        let mut literal_bits = bitvec![Msb0, u64;];
        while take_bits(1)[0] {
            literal_bits.append(&mut take_bits(4).to_bitvec());
        }
        literal_bits.append(&mut take_bits(4).to_bitvec());
        Packet {
            packet_version,
            packet_type: PacketType::Literal(literal_bits.load()),
        }
    } else {
        // operator
        let mut packets = vec![];
        let length_type_id = take_bits(1)[0];
        if length_type_id {
            // number of sub-packets
            let count_subpackets: u16 = (take_bits(11)).load_be();
            for _ in 0..count_subpackets {
                packets.push(parse_packet(bits)?);
            }
        } else {
            // total length in bits
            let count_bits: usize = (take_bits(15)).load_be();
            let mut packets_buf = take_bits(count_bits);
            while packets_buf.any() {
                packets.push(parse_packet(&mut packets_buf)?);
            }
        }
        Packet {
            packet_type: parse_operator(packet_type_id, packets)?,
            packet_version,
        }
    };
    Ok(packet)
}

fn parse_operator(packet_type_id: u8, packets: Vec<Packet>) -> Result<PacketType> {
    use PacketType::*;
    let binary_op = |op, packets: Vec<_>| -> Result<_> {
        let mut iter = packets.into_iter();
        let mut next = || {
            iter.next()
                .ok_or(format!("not enough arguments for {:?} op", op))
        };
        let a = Box::new(next()?);
        let b = Box::new(next()?);
        Ok(BinaryOperator(op, a, b))
    };
    let packet_type = match packet_type_id {
        0 => VecOperator(VecOp::Sum, packets),
        1 => VecOperator(VecOp::Product, packets),
        2 => VecOperator(VecOp::Minimum, packets),
        3 => VecOperator(VecOp::Maximum, packets),
        5 => binary_op(BinaryOp::GreaterThan, packets)?,
        6 => binary_op(BinaryOp::LessThan, packets)?,
        7 => binary_op(BinaryOp::EqualTo, packets)?,
        _ => Err(format!("invalid operator type {}", packet_type_id))?,
    };
    Ok(packet_type)
}

fn sum_versions(packet: &Packet) -> u64 {
    use PacketType::*;
    packet.packet_version as u64
        + match &packet.packet_type {
            VecOperator(_, packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            BinaryOperator(_, a, b) => sum_versions(a) + sum_versions(b),
            _ => 0,
        }
}

fn eval(packet: &Packet) -> u64 {
    use BinaryOp::*;
    use PacketType::*;
    use VecOp::*;
    match &packet.packet_type {
        Literal(value) => *value,
        VecOperator(op, values) => match op {
            Sum => values.iter().map(eval).sum(),
            Product => values.iter().map(eval).product(),
            Maximum => values.iter().map(eval).max().unwrap(),
            Minimum => values.iter().map(eval).min().unwrap(),
        },
        BinaryOperator(op, a, b) => match op {
            GreaterThan => {
                if eval(a) > eval(b) {
                    1
                } else {
                    0
                }
            }
            LessThan => {
                if eval(a) < eval(b) {
                    1
                } else {
                    0
                }
            }
            EqualTo => {
                if eval(a) == eval(b) {
                    1
                } else {
                    0
                }
            }
        },
    }
}
