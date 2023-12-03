#[derive(Debug, PartialEq)]
enum Type {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    pkt_type: u8,
    value: Type,
}

fn literal_group(bits: &mut BitsIter) -> Option<(bool, u8)> {
    let more = bits.take_value::<u8>(1)? != 0;
    let nibble = bits.take_value::<u8>(4)?;
    Some((more, nibble))
}

impl Packet {
    fn version_sum(&self) -> usize {
        let subpacket_sum = match &self.value {
            Type::Operator(v) => v.iter().fold(0, |s, p| s + p.version_sum()),
            _ => 0,
        };
        (self.version as usize) + subpacket_sum
    }

    fn value(&self) -> usize {
        match &self.value {
            Type::Literal(x) => *x as usize,
            Type::Operator(v) => match &self.pkt_type {
                0 => v.iter().fold(0, |s, p| s + p.value()),
                1 => v.iter().fold(1, |prod, p| prod * p.value()),
                2 => v.iter().map(|p: &Packet| p.value()).min().expect("value"),
                3 => v.iter().map(|p: &Packet| p.value()).max().expect("value"),
                5 => {
                    if v[0].value() > v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if v[0].value() < v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if v[0].value() == v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            },
        }
    }

    fn parse_literal(bits: &mut BitsIter) -> Option<Type> {
        let mut val = 0u64;
        while let Some(value) = literal_group(bits) {
            val = (val << 4) | (value.1 as u64);
            if !value.0 {
                break;
            }
        }
        Some(Type::Literal(val))
    }

    fn parse_operator_length(bits: &mut BitsIter) -> Option<Type> {
        let num_bits = bits.take_value::<u32>(15)? as usize;
        let mut pkts = Vec::<Packet>::default();
        let start_i = bits.bit_index;
        let mut end_i = bits.bit_index;
        while (end_i - start_i) < num_bits {
            let pkt = Packet::parse(bits)?;
            end_i = bits.bit_index;
            pkts.push(pkt);
        }
        Some(Type::Operator(pkts))
    }

    fn parse_operator_groups(bits: &mut BitsIter) -> Option<Type> {
        let num_groups = bits.take_value::<u32>(11)?;
        let pkts = (0..num_groups).fold(Some(Vec::<Packet>::default()), |acc, _| {
            let mut pkts = acc?;
            let pkt = Packet::parse(bits)?;
            pkts.push(pkt);
            Some(pkts)
        })?;
        Some(Type::Operator(pkts))
    }

    fn parse_operator(bits: &mut BitsIter) -> Option<Type> {
        match bits.take_value::<u8>(1) {
            Some(0) => Packet::parse_operator_length(bits),
            Some(1) => Packet::parse_operator_groups(bits),
            _ => None,
        }
    }

    fn parse(bits: &mut BitsIter) -> Option<Packet> {
        let version = bits.take_value(3)?;
        let pkt_type = bits.take_value(3)?;
        let value = match pkt_type {
            4 => Packet::parse_literal(bits)?,
            _ => Packet::parse_operator(bits)?,
        };
        Some(Packet {
            version,
            pkt_type,
            value,
        })
    }
}

#[derive(Debug)]
struct Bits {
    bits: Vec<u8>,
}

#[derive(Debug, Copy, Clone)]
struct BitsIter<'a> {
    bits: &'a Bits,
    bit_index: usize,
}

impl Bits {
    fn iter(&self) -> BitsIter {
        BitsIter {
            bits: self,
            bit_index: 0,
        }
    }

    fn as_binary_string(&self) -> String {
        self.iter().map(|b| if b { '1' } else { '0' }).collect()
    }
}

impl<'a> BitsIter<'a> {
    fn take_value<V>(&mut self, n: usize) -> Option<V>
    where
        V: num_traits::Unsigned + std::ops::BitOr<Output = V> + std::ops::Shl<Output = V>,
    {
        Some(self.take(n).fold(V::zero(), |acc, b| {
            (acc << V::one()) | if b { V::one() } else { V::zero() }
        }))
    }
}

impl<'a> Iterator for BitsIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_index < self.bits.bits.len() * 4 {
            let nibble = self.bits.bits[self.bit_index / 4];
            let bit = (nibble & (1 << (3 - (self.bit_index % 4)))) != 0;
            self.bit_index += 1;
            Some(bit)
        } else {
            None
        }
    }
}

fn parse_hex_stream(input: &str) -> Bits {
    let bits: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).expect("not hex digit") as u8)
        .collect();
    Bits { bits }
}

fn solve_part1(input: &str) -> usize {
    let bits = parse_hex_stream(input);
    let packet = Packet::parse(&mut bits.iter()).expect("packet");
    packet.version_sum()
}

fn solve_part2(input: &str) -> usize {
    let bits = parse_hex_stream(input);
    let packet = Packet::parse(&mut bits.iter()).expect("packet");
    packet.value()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
    solve_part2(INPUT);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("8A004A801A8002F478"), 16);
        assert_eq!(solve_part1("620080001611562C8802118E34"), 12);
        assert_eq!(solve_part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(solve_part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("C200B40A82"), 3);
        assert_eq!(solve_part2("880086C3E88112"), 7);
        assert_eq!(solve_part2("CE00C43D881120"), 9);
        assert_eq!(solve_part2("D8005AC2A8F0"), 1);
        assert_eq!(solve_part2("F600BC2D8F"), 0);
        assert_eq!(solve_part2("9C005AC2F8F0"), 0);
        assert_eq!(solve_part2("9C0141080250320F1802104A08"), 1);
    }

    #[test]
    fn test_parse_hex_stream() {
        assert_eq!(
            parse_hex_stream("D2FE28").as_binary_string(),
            "110100101111111000101000"
        );
        assert_eq!(
            parse_hex_stream("38006F45291200").as_binary_string(),
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            parse_hex_stream("EE00D40C823060").as_binary_string(),
            "11101110000000001101010000001100100000100011000001100000"
        );
    }

    #[test]
    fn test_parse_literal() {
        let bits = parse_hex_stream("D2FE28");
        let mut iter = bits.iter();
        let _ = iter.take_value::<u8>(6).expect("hdr");
        assert_eq!(
            Packet::parse_literal(&mut iter).expect("literal"),
            Type::Literal(2021)
        );
    }

    #[test]
    fn test_parse_operator_groups() {
        let bits = parse_hex_stream("EE00D40C823060");
        let mut iter = bits.iter();
        let _ = iter.take_value::<u8>(7).expect("hdr");
        assert_eq!(
            Packet::parse_operator_groups(&mut iter).expect("literal"),
            Type::Operator(vec![
                Packet {
                    version: 2,
                    pkt_type: 4,
                    value: Type::Literal(1)
                },
                Packet {
                    version: 4,
                    pkt_type: 4,
                    value: Type::Literal(2)
                },
                Packet {
                    version: 1,
                    pkt_type: 4,
                    value: Type::Literal(3)
                }
            ])
        );
    }

    #[test]
    fn test_parse_operator_length() {
        let bits = parse_hex_stream("38006F45291200");
        let mut iter = bits.iter();
        let _ = iter.take_value::<u8>(7).expect("hdr");
        assert_eq!(
            Packet::parse_operator_length(&mut iter).expect("literal"),
            Type::Operator(vec![
                Packet {
                    version: 6,
                    pkt_type: 4,
                    value: Type::Literal(10)
                },
                Packet {
                    version: 2,
                    pkt_type: 4,
                    value: Type::Literal(20)
                }
            ])
        );
    }
}
