use std::{fmt::Debug, str::Chars, usize};

#[derive(PartialEq, Eq)]
pub struct Number {
    left: Box<Content>,
    right: Box<Content>,
}

#[derive(PartialEq, Eq)]
pub enum Content {
    Pair(Number),
    Value(isize),
}

impl Number {
    pub fn phase(s: String) -> Number {
        build(&mut s.chars(), NumberBuilderState::Init)
    }

    fn new(left: Content, right: Content) -> Number {
        Number {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn explode_deep_numbers(&mut self) {
        let nested_counter = 1;
        if let Content::Pair(p) = &mut *self.left {
            find_to_explode(p, nested_counter + 1);
        }

        fn find_to_explode(num: &mut Number, depth: usize) {
            if let Content::Pair(p) = &mut *num.left {
                if depth == 3 {
                    match &mut *num.right {
                        Content::Value(x) => match &*p.right {
                            Content::Value(v) => num.right = Box::new(Content::Value(*x + v)),
                            Content::Pair(p) => panic!(),
                        },
                        Content::Pair(p) => {
                            panic!("exploding pair not next to regular value: {:?}", p)
                        }
                    }
                }

                find_to_explode(p, depth + 1);
            }
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number: [{:?},{:?}]", &self.left, &self.right)
    }
}

impl Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(p) => write!(f, "[{:?},{:?}]", p.left, p.right),
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

enum NumberBuilderState {
    Init,
    InBracket,
    AfterComma,
}

fn build(iter: &mut Chars, mut state: NumberBuilderState) -> Number {
    let mut left = None;
    let mut right = None;

    while let Some(c) = iter.next() {
        match c {
            '[' => match state {
                NumberBuilderState::Init => state = NumberBuilderState::InBracket,
                NumberBuilderState::InBracket => {
                    left = Some(Content::Pair(build(iter, NumberBuilderState::InBracket)))
                }
                NumberBuilderState::AfterComma => {
                    right = Some(Content::Pair(build(iter, NumberBuilderState::InBracket)))
                }
            },
            '0'..='9' => {
                if left.is_none() {
                    left = Some(Content::Value(c.to_digit(10).unwrap().try_into().unwrap()));
                } else {
                    right = Some(Content::Value(c.to_digit(10).unwrap().try_into().unwrap()));
                }
            }
            ',' => state = NumberBuilderState::AfterComma,
            ']' => break,
            _ => panic!("unkown char '{}'", c),
        }
    }
    Number::new(left.unwrap(), right.unwrap())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_input_simple() {
        let n = phaseStr("[1,2]");
        assert!(matches!(*n.left, Content::Value(1)));
        assert!(matches!(*n.right, Content::Value(2)));
    }

    #[test]
    fn test_input_left_nested() {
        let n = phaseStr("[[1,3],2]");
        match *n.left {
            Content::Pair(sub) => {
                assert!(matches!(*sub.left, Content::Value(1)));
                assert!(matches!(*sub.right, Content::Value(3)));
            }
            _ => panic!("left is not a pair"),
        }

        assert!(matches!(*n.right, Content::Value(2)));
    }

    #[test]
    fn test_input_right_nested() {
        let n = phaseStr("[1,[3,2]]");

        assert!(matches!(*n.left, Content::Value(1)));
        match *n.right {
            Content::Pair(sub) => {
                assert!(matches!(*sub.left, Content::Value(3)));
                assert!(matches!(*sub.right, Content::Value(2)));
            }
            _ => panic!("left is not a pair"),
        }
    }

    #[test]
    fn explode_depp_nested() {
        let mut n = phaseStr("[[[[[9,8],1],2],3],4]");
        let expected_after = phaseStr("[[[[0,9],2],3],4]");
        n.explode_deep_numbers();

        assert_eq!(n, expected_after);
    }

    fn phaseStr(s: &str) -> Number {
        let n = Number::phase(s.to_string());

        println!("{:#?}", n);
        n
    }
}
