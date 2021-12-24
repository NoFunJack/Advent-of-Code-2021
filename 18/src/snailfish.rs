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

    fn get_values(&self) -> (isize, isize) {
        let l = match *self.left {
            Content::Pair(..) => panic!(),
            Content::Value(v) => v,
        };
        let r = match *self.right {
            Content::Pair(..) => panic!(),
            Content::Value(v) => v,
        };

        (l, r)
    }

    fn explode_deep_numbers(&mut self) {
        self.explode_int(0);
    }

    fn explode_int(&mut self, depth: usize) -> (Option<isize>, Option<isize>) {
        let mut re = (None, None);

        println!("Debug d:{} num:{:?}", depth, self);

        if depth == 3 {
            println!("exp num: {:?}", self);
            if let Content::Pair(p) = &*self.left {
                let (l, r) = p.get_values();
                re = (Some(l), None);
                match &*self.right {
                    Content::Pair(np) => panic!("exploding left has pair right {:?}", np),
                    Content::Value(v) => {
                        self.right = Box::new(Content::Value(v + r));
                        self.left = Box::new(Content::Value(0));
                    }
                }
            }
            if let Content::Pair(p) = &*self.right {
                let (l, r) = p.get_values();
                re = (None, Some(r));
                match &*self.left {
                    Content::Pair(np) => panic!("exploding right has pair left {:?}", np),
                    Content::Value(v) => {
                        self.left = Box::new(Content::Value(v + l));
                        self.right = Box::new(Content::Value(0));
                    }
                }
            }
        } else {
            if let Content::Pair(p) = &mut *self.left {
                re = p.explode_int(depth + 1);
            }
            if let Content::Pair(p) = &mut *self.right {
                re = p.explode_int(depth + 1);
            }
            println!("re {:?}", re);

            if let Some(l) = re.0 {
                if let Content::Value(v) = *self.left {
                    println!("moved value {} to {}", l, v);
                    self.left = Box::new(Content::Value(v + l));
                    re.0 = None;
                }
            }
            if let Some(r) = re.1 {
                if let Content::Value(v) = *self.right {
                    println!("moved value {} to {}", r, v);
                    self.right = Box::new(Content::Value(v + r));
                    re.1 = None;
                }
            }
        }

        re
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
        let n = phase_str("[1,2]");
        assert!(matches!(*n.left, Content::Value(1)));
        assert!(matches!(*n.right, Content::Value(2)));
    }

    #[test]
    fn test_input_left_nested() {
        let n = phase_str("[[1,3],2]");
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
        let n = phase_str("[1,[3,2]]");

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
    fn explode_deep_nested_1() {
        assert_explotion("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    }
    #[test]
    fn explode_deep_nested_2() {
        assert_explotion("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    }
    #[test]
    fn explode_deep_nested_3() {
        assert_explotion("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn explode_deep_nested_4() {
        assert_explotion(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
    }
    #[test]
    fn explode_deep_nested_5() {
        assert_explotion(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    fn assert_explotion(n: &str, expected: &str) {
        let mut n = phase_str(n);
        let expected_after = phase_str(expected);
        n.explode_deep_numbers();
        assert_eq!(n, expected_after)
    }

    fn phase_str(s: &str) -> Number {
        let n = Number::phase(s.to_string());

        println!("{:#?}", n);
        n
    }
}
