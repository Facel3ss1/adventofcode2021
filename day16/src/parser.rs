use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map, recognize},
    multi::{length_count, length_value, many1, many_m_n, many_till},
    sequence::{preceded, tuple},
    IResult,
};

use bitvec::{mem::BitMemory, prelude::*};
use nom_bitvec::BSlice;

use crate::{Operator, Packet, PacketType};

type BitInput<'a> = BSlice<'a, Msb0, u8>;

fn hexadecimal(input: &str) -> IResult<&str, Vec<u8>> {
    many1(map(
        many_m_n(1, 2, recognize(one_of("0123456789ABCDEF"))),
        |digits| {
            let hex_number: String = digits.into_iter().collect();

            let output = u8::from_str_radix(&hex_number, 16).unwrap();

            if hex_number.len() == 2 {
                output
            } else {
                output << 4
            }
        },
    ))(input)
}

fn take_bits<'a, M: BitMemory>(
    count: usize,
) -> impl FnMut(BitInput<'a>) -> IResult<BitInput<'a>, M> {
    move |i: BitInput| {
        map(take(count), |BSlice(bits): BSlice<Msb0, u8>| {
            bits.load_be::<M>()
        })(i)
    }
}

fn group(input: BitInput) -> IResult<BitInput, u8> {
    preceded(tag(BSlice(bits![1])), take_bits(4))(input)
}

fn last_group(input: BitInput) -> IResult<BitInput, u8> {
    preceded(tag(BSlice(bits![0])), take_bits(4))(input)
}

fn literal(input: BitInput) -> IResult<BitInput, u64> {
    map(many_till(group, last_group), |(groups, last)| {
        let groups_num = groups
            .into_iter()
            .fold(0, |acc, num| (acc << 4) | (num as u64));

        (groups_num << 4) | (last as u64)
    })(input)
}

fn sub_packets_len(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
    length_value(take_bits::<u16>(15), many1(packet))(input)
}

fn sub_packets_count(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
    length_count(take_bits::<u16>(11), packet)(input)
}

fn operator(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
    let (input, length_type_id) = take(1_usize)(input)?;

    if length_type_id[0] {
        sub_packets_count(input)
    } else {
        sub_packets_len(input)
    }
}

fn packet_type(input: BitInput) -> IResult<BitInput, PacketType> {
    let (input, packet_type_id) = take_bits::<u8>(3)(input)?;

    match packet_type_id {
        4 => map(literal, PacketType::Literal)(input),
        _ => map(operator, |operands| {
            PacketType::Operator(Operator::new(packet_type_id, operands))
        })(input),
    }
}

fn packet(input: BitInput) -> IResult<BitInput, Packet> {
    map(
        tuple((take_bits(3), packet_type)),
        |(version, packet_type)| Packet {
            version,
            packet_type,
        },
    )(input)
}

impl Packet {
    pub fn parse(input: &str) -> Self {
        let (_, bytes) = hexadecimal(input).unwrap();
        let (_, packet) = packet(BSlice(bytes.view_bits())).unwrap();
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hexadecimal() {
        let hex = "D2FE28A";
        assert_eq!(hexadecimal(hex), Ok(("", vec![0xd2, 0xfe, 0x28, 0xa0])));
    }

    #[test]
    fn test_group() {
        let input = [0b10111000];
        let output = group(BSlice(input.view_bits()));
        assert_eq!(output, Ok((BSlice(bits![Msb0, u8; 0, 0, 0]), 7)));

        let input = [0b11110000];
        let output = group(BSlice(input.view_bits()));
        assert_eq!(output, Ok((BSlice(bits![Msb0, u8; 0, 0, 0]), 14)));

        let input = [0b01010000];
        let output = group(BSlice(input.view_bits()));
        assert!(output.is_err());
    }

    #[test]
    fn test_last_group() {
        let input = [0b01010000];
        let output = last_group(BSlice(input.view_bits()));
        assert_eq!(output, Ok((BSlice(bits![Msb0, u8; 0, 0, 0]), 10)));

        let input = [0b11110000];
        let output = last_group(BSlice(input.view_bits()));
        assert!(output.is_err());
    }

    #[test]
    fn test_literal() {
        let input = [0b10111111, 0b10001010];
        println!("{:?}", input.view_bits::<Msb0>());
        let output = literal(BSlice(input.view_bits()));
        assert_eq!(output, Ok((BSlice(bits![Msb0, u8; 0]), 2021)));
    }

    #[test]
    fn test_literal_packet() {
        let input = [0b11010010, 0b11111110, 0b00101000];
        let output = packet(BSlice(input.view_bits()));
        assert_eq!(
            output,
            Ok((
                BSlice(bits![Msb0, u8; 0, 0, 0]),
                Packet {
                    version: 6,
                    packet_type: PacketType::Literal(2021)
                }
            ))
        );
    }

    #[test]
    fn test_literal_packet_hexadecimal() {
        let input = "D2FE28";
        let output = Packet::parse(input);
        assert_eq!(
            output,
            Packet {
                version: 6,
                packet_type: PacketType::Literal(2021)
            }
        );
    }

    #[test]
    fn test_task1_examples() {
        let input = "8A004A801A8002F478";
        let output = Packet::parse(input);
        assert_eq!(output.version_sum(), 16);

        let input = "620080001611562C8802118E34";
        let output = Packet::parse(input);
        assert_eq!(output.version_sum(), 12);

        let input = "C0015000016115A2E0802F182340";
        let output = Packet::parse(input);
        assert_eq!(output.version_sum(), 23);

        let input = "A0016C880162017C3686B18A3D4780";
        let output = Packet::parse(input);
        assert_eq!(output.version_sum(), 31);
    }

    #[test]
    fn test_task2_examples() {
        let input = "C200B40A82";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 3);

        let input = "04005AC33890";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 54);

        let input = "880086C3E88112";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 7);

        let input = "CE00C43D881120";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 9);

        let input = "D8005AC2A8F0";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 1);

        let input = "F600BC2D8F";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 0);

        let input = "9C005AC2F8F0";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 0);

        let input = "9C0141080250320F1802104A08";
        let output = Packet::parse(input);
        assert_eq!(output.evaluate(), 1);
    }
}
