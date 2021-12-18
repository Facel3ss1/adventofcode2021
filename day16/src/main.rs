mod parser;

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u32),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u32,
            PacketType::Operator(packets) => {
                (self.version as u32) + packets.iter().map(|p| p.version_sum()).sum::<u32>()
            }
        }
    }
}

fn main() {
    let packet = Packet::parse(include_str!("input.txt"));

    println!("Task 1: {}", packet.version_sum());
}
