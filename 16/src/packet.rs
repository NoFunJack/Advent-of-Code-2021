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
                _ => PacketContent::operator(bits),
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
        if let PacketContent::Operator(subs) = &self.content {
            for s in subs {
                sum += s.get_version_sum();
            }
        }
        sum
    }
}

#[derive(Debug)]
enum PacketContent {
    LiteralValue(usize),
    Operator(Vec<Packet>),
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

    fn operator(bits: &mut BitsStream) -> PacketContent {
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

        Self::Operator(subpackets)
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
            PacketContent::Operator(x) => x,
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
            PacketContent::Operator(x) => x,
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
}
