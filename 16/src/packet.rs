use std::usize;

use crate::bitsreader::BitsStream;

#[derive(Debug)]
pub struct Packet {
    version: usize,
    content: PacketContent,
}

impl Packet {
    pub fn new(bits: &mut BitsStream, limit: &mut Option<LengthType>) -> Packet {
        let init_bit_pos = bits.char_iter;
        let v = bits.next_n_as_number(3);

        let re = Packet {
            version: v,
            content: match bits.next_n_as_number(3) {
                4 => PacketContent::lit_val(bits),
                x => PacketContent::operator(bits, x),
            },
        };

        if let Some(l) = limit {
            l.reduce_bits(bits.char_iter - init_bit_pos);
            l.reduce_package();
        }

        re
    }

    pub fn get_version_sum(&self) -> usize {
        let mut sum = self.version;
        if let PacketContent::Operator(_, subs) = &self.content {
            for s in subs {
                sum += s.get_version_sum();
            }
        }
        sum
    }

    pub fn eval(&self) -> usize {
        match &self.content {
            PacketContent::LiteralValue(x) => *x,
            PacketContent::Operator(OperatorType::Sum, v) => v.iter().map(|p| p.eval()).sum(),
            PacketContent::Operator(OperatorType::Product, v) => {
                v.iter().map(|p| p.eval()).product()
            }
            PacketContent::Operator(OperatorType::Minimum, v) => {
                v.iter().map(|p| p.eval()).min().unwrap()
            }
            PacketContent::Operator(OperatorType::Maximum, v) => {
                v.iter().map(|p| p.eval()).max().unwrap()
            }
            PacketContent::Operator(OperatorType::LessThan, v) => {
                match v.get(0).unwrap().eval() < v.get(1).unwrap().eval() {
                    true => 1,
                    false => 0,
                }
            }
            PacketContent::Operator(OperatorType::GreaterThan, v) => {
                match v.get(0).unwrap().eval() > v.get(1).unwrap().eval() {
                    true => 1,
                    false => 0,
                }
            }
            PacketContent::Operator(OperatorType::EqualTo, v) => {
                match v.get(0).unwrap().eval() == v.get(1).unwrap().eval() {
                    true => 1,
                    false => 0,
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketContent {
    LiteralValue(usize),
    Operator(OperatorType, Vec<Packet>),
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl PacketContent {
    fn lit_val(bits: &mut BitsStream) -> PacketContent {
        let mut all = "".to_string();
        loop {
            let data = bits.get_bin_slice(5);
            all.push_str(&data[1..5]);
            if let "0" = &data[0..1] {
                break;
            }
        }
        Self::LiteralValue(usize::from_str_radix(&all, 2).unwrap())
    }

    fn operator(bits: &mut BitsStream, type_id: usize) -> PacketContent {
        // does the length even matter?
        let mut length_id = match bits.next_n_as_number(1) {
            0 => Some(LengthType::BitLength(bits.next_n_as_number(15))),
            1 => Some(LengthType::NumberOfPackets(bits.next_n_as_number(11))),
            _ => unreachable!(),
        };

        let mut subpackets = Vec::new();

        while length_id.as_ref().unwrap().contains_packages() {
            subpackets.push(Packet::new(bits, &mut length_id));
        }

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

        Self::Operator(op_type, subpackets)
    }
}

pub enum LengthType {
    BitLength(usize),
    NumberOfPackets(usize),
}

impl LengthType {
    fn contains_packages(&self) -> bool {
        match self {
            Self::BitLength(n) => *n > 0,
            Self::NumberOfPackets(n) => *n > 0,
        }
    }

    fn reduce_bits(&mut self, remove: usize) {
        if let Self::BitLength(bits) = self {
            *bits -= remove;
        }
    }

    fn reduce_package(&mut self) {
        if let Self::NumberOfPackets(n) = self {
            *n -= 1;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitsreader::BitsStream;

    #[test]
    fn example_lit_val() {
        let p = Packet::new(&mut BitsStream::new("D2FE28".to_string()), &mut None);
        assert_eq!(p.version, 6);

        let val = match p.content {
            PacketContent::LiteralValue(x) => x,
            _ => panic!("Packet id 4 should be LiteralValue"),
        };

        assert_eq!(val, 2021);
    }

    #[test]
    fn example_operator_bitlength() {
        let p = Packet::new(
            &mut BitsStream::new("38006F45291200".to_string()),
            &mut None,
        );
        assert_eq!(p.version, 1);

        let val = match p.content {
            PacketContent::Operator(_, x) => x,
            _ => panic!("Packet not id 4 should be Operator"),
        };

        let mut subs = val.iter();
        assert!(matches!(
            subs.next().unwrap().content,
            PacketContent::LiteralValue(10)
        ));
        assert!(matches!(
            subs.next().unwrap().content,
            PacketContent::LiteralValue(20)
        ));
        assert!(matches!(subs.next(), None));
    }

    #[test]
    fn example_operator_packetlen() {
        let p = Packet::new(
            &mut BitsStream::new("EE00D40C823060".to_string()),
            &mut None,
        );
        assert_eq!(p.version, 7);

        let val = match p.content {
            PacketContent::Operator(_, x) => x,
            _ => panic!("Packet not id 4 should be Operator"),
        };

        let mut subs = val.iter();
        assert!(matches!(
            subs.next().unwrap().content,
            PacketContent::LiteralValue(1)
        ));
        assert!(matches!(
            subs.next().unwrap().content,
            PacketContent::LiteralValue(2)
        ));
        assert!(matches!(
            subs.next().unwrap().content,
            PacketContent::LiteralValue(3)
        ));
        assert!(matches!(subs.next(), None));
    }

    #[test]
    fn eval_sum() {
        assert_eq!(eval("C200B40A82"), 3);
    }

    #[test]
    fn eval_prod() {
        assert_eq!(eval("04005AC33890"), 54);
    }

    #[test]
    fn eval_min() {
        assert_eq!(eval("880086C3E88112"), 7);
    }

    #[test]
    fn eval_max() {
        assert_eq!(eval("CE00C43D881120"), 9);
    }

    #[test]
    fn eval_less_than() {
        assert_eq!(eval("D8005AC2A8F0"), 1);
    }

    #[test]
    fn eval_greater_than() {
        assert_eq!(eval("F600BC2D8F"), 0);
    }

    #[test]
    fn eval_equal() {
        assert_eq!(eval("9C005AC2F8F0"), 0);
    }

    #[test]
    fn eval_mixed() {
        assert_eq!(eval("9C0141080250320F1802104A08"), 1);
    }

    fn eval(input: &str) -> usize {
        Packet::new(&mut BitsStream::new(input.to_string()), &mut None).eval()
    }
}
