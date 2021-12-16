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
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Box<(Packet, Packet)>),
    LessThan(Box<(Packet, Packet)>),
    EqualTo(Box<(Packet, Packet)>),
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
        let packet_type = PacketType::Literal(literal_bits.load());
        Packet {
            packet_version,
            packet_type,
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
                let parsed = parse_packet(&mut packets_buf)?;
                packets.push(parsed);
            }
        }
        Packet {
            packet_type: parse_operator(packet_type_id, packets)?,
            packet_version,
        }
    };
    Ok(packet)
}

fn parse_operator(packet_type_id: u8, subpackets: Vec<Packet>) -> Result<PacketType> {
    fn first_two(packets: Vec<Packet>) -> Option<Box<(Packet, Packet)>> {
        let mut iter = packets.into_iter();
        Some(Box::new((iter.next()?, iter.next()?)))
    }
    let packet_type = match packet_type_id {
        0 => PacketType::Sum(subpackets),
        1 => PacketType::Product(subpackets),
        2 => PacketType::Minimum(subpackets),
        3 => PacketType::Maximum(subpackets),
        5 => PacketType::GreaterThan(first_two(subpackets).ok_or("too few packets for > op")?),
        6 => PacketType::LessThan(first_two(subpackets).ok_or("too few packets for < op")?),
        7 => PacketType::EqualTo(first_two(subpackets).ok_or("too few packets for = op")?),
        _ => Err(format!("invalid operator type {}", packet_type_id))?,
    };
    Ok(packet_type)
}

fn sum_versions(packet: &Packet) -> u64 {
    fn sum_subpacket_versions(packets: &Vec<Packet>) -> u64 {
        packets.iter().map(|p| sum_versions(p)).sum()
    }
    fn sum_pair_versions(pair: &Box<(Packet, Packet)>) -> u64 {
        let (a, b) = pair.as_ref();
        sum_versions(a) + sum_versions(b)
    }
    packet.packet_version as u64
        + match &packet.packet_type {
            PacketType::Sum(packets) => sum_subpacket_versions(packets),
            PacketType::Product(packets) => sum_subpacket_versions(packets),
            PacketType::Minimum(packets) => sum_subpacket_versions(packets),
            PacketType::Maximum(packets) => sum_subpacket_versions(packets),
            PacketType::GreaterThan(packets) => sum_pair_versions(packets),
            PacketType::LessThan(packets) => sum_pair_versions(packets),
            PacketType::EqualTo(packets) => sum_pair_versions(packets),
            _ => 0,
        }
}

fn eval(packet: &Packet) -> u64 {
    match &packet.packet_type {
        PacketType::Literal(value) => *value,
        PacketType::Sum(values) => values.iter().map(eval).sum(),
        PacketType::Product(values) => values.iter().map(eval).product(),
        PacketType::Maximum(values) => values.iter().map(eval).max().unwrap(),
        PacketType::Minimum(values) => values.iter().map(eval).min().unwrap(),
        PacketType::GreaterThan(pair) => {
            let (a, b) = pair.as_ref();
            if eval(a) > eval(b) {
                1
            } else {
                0
            }
        }
        PacketType::LessThan(pair) => {
            let (a, b) = pair.as_ref();
            if eval(a) < eval(b) {
                1
            } else {
                0
            }
        }
        PacketType::EqualTo(pair) => {
            let (a, b) = pair.as_ref();
            if eval(a) == eval(b) {
                1
            } else {
                0
            }
        }
    }
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
