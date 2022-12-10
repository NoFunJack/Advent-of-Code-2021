use std::{cell::RefCell, fmt::Debug, rc::Rc, str::Chars, usize};
use Number::{Pair, Value};

#[derive(PartialEq, Eq)]
pub enum Number {
    Pair(Box<Number>, Box<Number>),
    Value(isize),
}

impl Number {
    pub fn phase(s: String) -> Number {
        build(&mut s.chars(), NumberBuilderState::Init)
    }

    fn new(left: Number, right: Number) -> Number {
        Pair(Box::new(left), Box::new(right))
    }

    fn explode_first_deep_number(&mut self) {
        if let Pair(left, right) = self {
            if !left.explode_int(1, None, Some(right)) {
                right.explode_int(1, Some(left), None);
            }
        } else {
            panic!("root number is not a pair");
        }
    }

    fn explode_int(
        &mut self,
        depth: usize,
        next_left: Option<&mut Box<Number>>,
        next_right: Option<&mut Box<Number>>,
    ) -> bool {
        println!("Checking {:?} depth: {}", self, depth);
        println!("left {:?} right {:?}", next_left, next_right);
        if let Pair(l, r) = self {
            if depth == 3 {
                // Left one is Pair
                if let Pair(il, ir) = &**l {
                    println!("Exploding {:?}", l);
                    if let Some(nl) = next_left {
                        nl.add_right_value(il.get_value_if_value());
                    }
                    r.add_left_value(ir.get_value_if_value());
                    *l = Box::new(Value(0));
                    return true;
                }
                // Right one is Pair
                if let Pair(il, ir) = &**r {
                    println!("Exploding {:?}", r);
                    if let Some(nr) = next_right {
                        nr.add_left_value(ir.get_value_if_value());
                    }
                    l.add_right_value(il.get_value_if_value());
                    *r = Box::new(Value(0));
                    return true;
                }

                return false;
            } else {
                l.explode_int(depth + 1, next_left, Some(r))
                    || r.explode_int(depth + 1, Some(l), next_right)
            }
        } else {
            false
        }
    }

    fn add_left_value(&mut self, to_add: Option<isize>) {
        if let Value(x) = self {
            if let Some(a) = to_add {
                println!("Adding {} to {}", a, x);
                *x += a;
            }
        } else {
            if let Pair(l, _) = self {
                l.add_left_value(to_add);
            }
        }
    }

    fn add_right_value(&mut self, to_add: Option<isize>) {
        if let Value(x) = self {
            if let Some(a) = to_add {
                println!("Adding {} to {}", a, x);
                *x += a;
            }
        } else {
            if let Pair(_, r) = self {
                r.add_right_value(to_add);
            }
        }
    }

    fn get_value_if_value(&self) -> Option<isize> {
        if let Value(x) = self {
            Some(*x)
        } else {
            None
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(l, r) => write!(f, "[{:?},{:?}]", l, r),
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
                    left = Some(build(iter, NumberBuilderState::InBracket))
                }
                NumberBuilderState::AfterComma => {
                    right = Some(build(iter, NumberBuilderState::InBracket))
                }
            },
            '0'..='9' => {
                if left.is_none() {
                    left = Some(Number::Value(c.to_digit(10).unwrap().try_into().unwrap()));
                } else {
                    right = Some(Number::Value(c.to_digit(10).unwrap().try_into().unwrap()));
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

    use super::Number::Pair;
    use super::Number::Value;
    use super::*;

    #[test]
    fn test_input_simple() {
        let n = phase_str("[1,2]");
        assert_eq!(n, Number::new(Value(1), Value(2)));
    }

    #[test]
    fn test_input_left_nested() {
        let n = phase_str("[[1,3],2]");
        match n {
            Pair(left, right) => {
                assert_eq!(*left, Number::new(Value(1), Value(3)));
                assert_eq!(*right, Value(2));
            }
            _ => panic!("left is not a pair"),
        }
    }

    #[test]
    fn test_input_right_nested() {
        let n = phase_str("[1,[3,2]]");

        match n {
            Pair(left, right) => {
                assert_eq!(*left, Value(1));
                assert_eq!(*right, Number::new(Value(3), Value(2)));
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
        n.explode_first_deep_number();
        assert_eq!(n, expected_after)
    }

    fn phase_str(s: &str) -> Number {
        let n = Number::phase(s.to_string());

        println!("{:#?}", n);
        n
    }
}
