pub struct BitsStream {
    raw: String,
    pub char_iter: usize,
}

impl BitsStream {
    pub fn new(input: String) -> BitsStream {
        BitsStream {
            raw: input
                .chars()
                .map(|c| BitsStream::to_binary(c))
                .collect::<String>(),
            char_iter: 0,
        }
    }

    pub fn next_n_as_number(&mut self, n: usize) -> usize {
        let bin = self.get_bin_slice(n);
        usize::from_str_radix(&bin, 2).unwrap()
    }

    pub fn get_bin_slice(&mut self, n: usize) -> String {
        let re = &self.raw[self.char_iter..(self.char_iter + n)];
        self.char_iter += n;
        println!("return slice {}", re);
        re.to_string()
    }

    fn to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_hex_code() {
        assert_eq!(BitsStream::new("0".to_string()).next_n_as_number(4), 0);
        assert_eq!(BitsStream::new("1".to_string()).next_n_as_number(4), 1);
        assert_eq!(BitsStream::new("2".to_string()).next_n_as_number(4), 2);
        assert_eq!(BitsStream::new("3".to_string()).next_n_as_number(4), 3);
        assert_eq!(BitsStream::new("4".to_string()).next_n_as_number(4), 4);
        assert_eq!(BitsStream::new("5".to_string()).next_n_as_number(4), 5);
        assert_eq!(BitsStream::new("6".to_string()).next_n_as_number(4), 6);
        assert_eq!(BitsStream::new("7".to_string()).next_n_as_number(4), 7);
        assert_eq!(BitsStream::new("8".to_string()).next_n_as_number(4), 8);
        assert_eq!(BitsStream::new("9".to_string()).next_n_as_number(4), 9);
        assert_eq!(BitsStream::new("A".to_string()).next_n_as_number(4), 10);
        assert_eq!(BitsStream::new("B".to_string()).next_n_as_number(4), 11);
        assert_eq!(BitsStream::new("C".to_string()).next_n_as_number(4), 12);
        assert_eq!(BitsStream::new("D".to_string()).next_n_as_number(4), 13);
        assert_eq!(BitsStream::new("E".to_string()).next_n_as_number(4), 14);
        assert_eq!(BitsStream::new("F".to_string()).next_n_as_number(4), 15);
    }

    #[test]
    fn iter_test() {
        // 1101 0110 1011
        let mut bs = BitsStream::new("D6B".to_string());
        // 110
        assert_eq!(bs.next_n_as_number(3), 6);
        // 101
        assert_eq!(bs.next_n_as_number(3), 5);
        // 101
        assert_eq!(bs.next_n_as_number(3), 5);
        // 011
        assert_eq!(bs.next_n_as_number(3), 3);
    }
}
