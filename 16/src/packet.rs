use crate::bitsreader::BitsStream;

struct Packet {
    version: usize,
    content: PacketContent,
}

impl Packet {
    fn new(mut bits: BitsStream) -> Packet {
        let v = bits.next_n_as_number(3);

        Packet {
            version: v,
            content: match bits.next_n_as_number(3) {
                4 => PacketContent::lit_val(&mut bits),
                _ => todo!(),
            },
        }
    }
}

enum PacketContent {
    LiteralValue(usize),
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
        PacketContent::LiteralValue(usize::from_str_radix(&all, 2).unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitsreader::BitsStream;

    #[test]
    fn example() {
        let p = Packet::new(BitsStream::new("D2FE28".to_string()));
        assert_eq!(p.version, 6);

        let val = match p.content {
            PacketContent::LiteralValue(x) => x,
            _ => panic!("Version 6 should be LiteralValue"),
        };

        assert_eq!(val, 2021);
    }
}
