pub struct ModelNumbers {
    number: Vec<u8>,
    plan: Vec<Step>,
    exausted: bool,
}

impl ModelNumbers {
    pub fn new(plan: Vec<Step>) -> ModelNumbers {
        ModelNumbers {
            number: vec![9; plan.len()],
            plan,
            exausted: false,
        }
    }

    fn to_str(&mut self) -> String {
        return self.number.iter().map(|p| p.to_string()).collect();
    }

    fn is_valid(&self, until: usize) -> bool {
        let mut z = 0;

        for i in 0..=until {
            let n = i32::from(self.number[i]);
            let x = (z % 26) + self.plan[i].a;
            //println!("x: {} z: {} a: {}", x, z, self.plan[i].a);
            if n == x {
                println!("invalid!");
                // never multiply
                return false;
            } else {
                z += n + self.plan[i].b;
            }
        }

        return true;
    }

    fn decrease_at(&mut self, pos: usize) -> Result<(), ()> {
        match self.number[pos] {
            2..=9 => {
                self.number[pos] = self.number[pos].checked_sub(1).unwrap();
                Ok(())
            }
            1 => {
                if pos > 0 {
                    for i in pos..self.number.len() {
                        self.number[i] = 9;
                    }
                    self.decrease_at(pos - 1)
                } else {
                    Err(())
                }
            }
            _ => panic!("bad number {}", self.number[pos]),
        }
    }
}

impl Iterator for ModelNumbers {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        //        println!("current: {:?}", self.number);
        if self.exausted {
            return None;
        }
        let current = self.number.clone();

        let mut i = 0;
        loop {
            //println!("loop {}", i);
            if self.is_valid(i) {
                i += 1;
            } else {
                match self.decrease_at(i - 1) {
                    Ok(_) => break,
                    Err(_) => {
                        self.exausted = true;
                        return Some(current.iter().map(|p| p.to_string()).collect());
                    }
                };
            }

            if i == self.plan.len() {
                match self.decrease_at(i - 1) {
                    Ok(_) => break,
                    Err(_) => {
                        self.exausted = true;
                        return Some(current.iter().map(|p| p.to_string()).collect());
                    }
                };
            }
        }

        Some(current.iter().map(|p| p.to_string()).collect())
    }
}

#[derive(Clone, Debug)]
struct NumberPart {
    number: u8,
    exausted: bool,
}

pub struct Step {
    a: i32,
    b: i32,
}

impl Step {
    pub fn new(a: i32, b: i32) -> Step {
        Step { a, b }
    }
}

impl NumberPart {
    pub fn new() -> NumberPart {
        NumberPart {
            number: 9,
            exausted: false,
        }
    }
}

impl Iterator for NumberPart {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exausted {
            return None;
        } else {
            let current = self.number;
            match current {
                2..=9 => {
                    self.number = self.number.checked_sub(1).unwrap();
                    return Some(current);
                }
                1 => {
                    self.exausted = true;
                    return Some(current);
                }
                _ => panic!("bad number {}", current),
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn use_9_to_1() {
        let mut iter = NumberPart::new();

        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn skip_invalid_candiates() {
        let plan = vec![Step::new(11, 14), Step::new(-3, 14)];

        //let mut iter = ModelNumbers::new(plan);

        println!("{:?}", ModelNumbers::new(plan).collect::<Vec<String>>());

        //assert_eq!(iter.next(), Some("99".to_string()));
        //assert_eq!(iter.next(), None);

        todo!()
    }
}
