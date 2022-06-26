pub struct ModelNumbers {
    // most significant number on the right
    number: [u8; 14],
}

impl ModelNumbers {
    pub fn new() -> ModelNumbers {
        ModelNumbers { number: [9; 14] }
    }

    fn to_str(&mut self) -> String {
        self.number
            .iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<String>()
    }
}

impl Iterator for ModelNumbers {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.number.iter().all(|&x| x == 0) {
            return None;
        } else {
            for (i, l) in self.number.clone().iter().enumerate() {
                match l {
                    2..=9 => {
                        self.number[i] = l.checked_sub(1).unwrap();
                        return Some(self.to_str());
                    }
                    1 => self.number[i] = 9,
                    _ => panic!("bad number {}", l),
                }
            }
            return None;
        }
    }
}
