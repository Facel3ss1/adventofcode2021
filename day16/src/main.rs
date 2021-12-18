use itertools::Itertools;

mod parser;

#[derive(Debug, PartialEq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq)]
struct Operator {
    op_type: OperatorType,
    operands: Vec<Packet>,
}

impl Operator {
    fn new(type_id: u8, operands: Vec<Packet>) -> Self {
        let op_type = match type_id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!(),
        };

        Self { op_type, operands }
    }
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u64,
            PacketType::Operator(op) => {
                (self.version as u64) + op.operands.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn evaluate(self) -> u64 {
        match self.packet_type {
            PacketType::Literal(num) => num,
            PacketType::Operator(op) => {
                let operands = op.operands.into_iter().map(|p| p.evaluate());

                match op.op_type {
                    OperatorType::Sum => operands.sum(),
                    OperatorType::Product => operands.product(),
                    OperatorType::Minimum => operands.min().unwrap(),
                    OperatorType::Maximum => operands.max().unwrap(),
                    OperatorType::GreaterThan => {
                        operands.take(2).tuple_windows().all(|(a, b)| a > b) as u64
                    }
                    OperatorType::LessThan => {
                        operands.take(2).tuple_windows().all(|(a, b)| a < b) as u64
                    }
                    OperatorType::EqualTo => {
                        operands.take(2).tuple_windows().all(|(a, b)| a == b) as u64
                    }
                }
            }
        }
    }
}

fn main() {
    let packet = Packet::parse(include_str!("input.txt"));

    println!("Task 1: {}", packet.version_sum());
    println!("Task 2: {}", packet.evaluate());
}
