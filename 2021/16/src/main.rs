use std::{
    fs::OpenOptions,
    io::{self, Read},
};

#[derive(Debug)]
enum LengthTypeID {
    TotalLength(usize),
    Subpackets(usize),
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    Operator(Operator, LengthTypeID, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    Version: u8,
    PacketType: PacketType,
}

fn parse(st: &String) -> Result<Vec<u8>, std::num::ParseIntError> {
    let mut st_cloned = st.clone().trim().to_string();
    for _ in 0..(st_cloned.len() % 4) {
        st_cloned.push('0');
    }
    (0..st_cloned.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&st_cloned[i..i + 2], 16))
        .collect()
}

fn parse_to_st(st: &String) -> Result<String, std::num::ParseIntError> {
    let mut st_cloned = st.clone().trim().to_string();
    for _ in 0..(st_cloned.len() % 4) {
        st_cloned.push('0');
    }
    let st: Vec<String> = (0..st_cloned.len())
        .map(|i| u8::from_str_radix(&st_cloned[i..i + 1], 16).unwrap())
        .map(|i| format!("{:04b}", i))
        .collect();

    //println!("lis: {:?}", st);
    let joined = st.join("");
    Ok(joined)
}

impl Packet {
    fn version_sum(&self) -> u32 {
        use PacketType::*;
        let mut version_sum: u32 = self.Version as u32;

        version_sum += match &self.PacketType {
            Literal(_) => 0,
            Operator(_, _, subpackets) => subpackets.iter().map(|p| p.version_sum()).sum(),
        };

        version_sum
    }

    fn value(&self) -> u64 {
        use crate::Operator::*;
        use PacketType::*;

        match &self.PacketType {
            Literal(n) => *n,
            Operator(op, _, subpackets) => match op {
                Sum => subpackets.iter().map(|p| p.value()).sum(),
                Product => subpackets.iter().map(|p| p.value()).product(),
                Minimum => subpackets.iter().map(|p| p.value()).min().unwrap(),
                Maximum => subpackets.iter().map(|p| p.value()).max().unwrap(),
                GreaterThan => {
                    if subpackets[0].value() > subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                LessThan => {
                    if subpackets[0].value() < subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                Equal => {
                    if subpackets[0].value() == subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }

    fn parse_as_literal(bits: &str) -> (PacketType, usize) {
        let mut more = true;
        let mut val: u64 = 0;

        let mut offset = 0;

        while more {
            let msb = u8::from_str_radix(&bits[offset..offset + 1], 2).unwrap();
            if msb < 1 {
                more = false
            }
            val <<= 4;
            // println!("group {} = {}", offset, &bits[offset + 1..offset + 5]);
            val |= u64::from_str_radix(&bits[offset + 1..offset + 5], 2).unwrap();

            offset += 5;
        }

        (PacketType::Literal(val), offset)
    }

    fn parse_as_operator(n: u8, bits: &str) -> (PacketType, usize) {
        use LengthTypeID::*;
        use Operator::*;

        let mut parsed_len = 0;

        let length_type = match u8::from_str_radix(&bits[0..1], 2).unwrap() {
            0 => {
                parsed_len += 16;
                TotalLength(usize::from_str_radix(&bits[1..16], 2).unwrap())
            }
            1 => {
                parsed_len += 12;
                Subpackets(usize::from_str_radix(&bits[1..12], 2).unwrap())
            }
            a => panic!("impossible length id: {}", a),
        };

        let packets = match length_type {
            TotalLength(len) => {
                let mut v = vec![];
                let target_len = parsed_len + len;
                while parsed_len < len + 16 {
                    let (parsed, consumed) = Self::parse(&bits[parsed_len..target_len]);
                    parsed_len += consumed;
                    v.push(parsed);
                }
                v
            }
            Subpackets(num) => {
                let mut v = vec![];
                for i in 0..num {
                    let (parsed, consumed) = Self::parse(&bits[parsed_len..]);
                    parsed_len += consumed;
                    v.push(parsed);
                }
                v
            }
        };

        let o = match n {
            0 => PacketType::Operator(Sum, length_type, packets),
            1 => PacketType::Operator(Product, length_type, packets),
            2 => PacketType::Operator(Minimum, length_type, packets),
            3 => PacketType::Operator(Maximum, length_type, packets),
            5 => PacketType::Operator(GreaterThan, length_type, packets),
            6 => PacketType::Operator(LessThan, length_type, packets),
            7 => PacketType::Operator(Equal, length_type, packets),
            _ => panic!("impossible packet type {}", n),
        };

        (o, parsed_len)
    }

    pub fn parse(bits: &str) -> (Self, usize) {
        let version = u8::from_str_radix(&bits[0..3], 2).unwrap();
        let (type_id, parsed_len) = match u8::from_str_radix(&bits[3..6], 2).unwrap() {
            4 => Self::parse_as_literal(&bits[6..]),
            n => Self::parse_as_operator(n, &bits[6..]),
        };
        (
            Packet {
                Version: version,
                PacketType: type_id,
            },
            parsed_len + 6,
        )
    }
}

mod tests {
    #[test]
    fn test_parse() {
        use super::*;

        let p = Packet::parse("110100101111111000101000");
        println!("{:?}", p);

        let p = Packet::parse("00111000000000000110111101000101001010010001001000000000");
        println!("{:?}", p);

        //let p = Packet::parse("111 011 1 0000000001 101010000001100100000100011000001100000");
        let p = Packet::parse("11101110000000001101010000001100100000100011000001100000");
        println!("{:?}", p)
    }

    #[test]
    fn test_parse_to_string() {
        use super::*;

        for st in ["38006F45291200"] {
            let bin = parse_to_st(&st.to_string()).unwrap();
            println!("bin: {}", bin);
        }
    }

    #[test]
    fn simple_example_version_sum() {
        use super::*;

        for st in [
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ] {
            let bin = parse_to_st(&st.to_string()).unwrap();
            let p = Packet::parse(bin.as_str());
            println!("{:?}", p.0.version_sum());
        }
    }

    #[test]
    fn simple_example_value() {
        use super::*;

        for st in [
            "C200B40A82",
            "04005AC33890",
            "880086C3E88112",
            "CE00C43D881120",
            "D8005AC2A8F0",
            "F600BC2D8F",
            "9C005AC2F8F0",
            "9C0141080250320F1802104A08",
        ] {
            let bin = parse_to_st(&st.to_string()).unwrap();
            let p = Packet::parse(bin.as_str());
            println!("{:?}", p.0.value());
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    // Part 1
    //println!("{:?}", st);
    let bin = parse_to_st(&st.to_string()).unwrap();
    let p = Packet::parse(bin.as_str());
    println!("{:?}", p.0.version_sum());
    // Part 2
    println!("{:?}", p.0.value());
}
