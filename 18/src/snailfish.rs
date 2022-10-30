use std::{cell::RefCell, fmt::Debug, rc::Rc, str::Chars, usize};
use Number::{Pair, Value};

#[derive(PartialEq, Eq)]
pub enum Number {
    Pair(Rc<RefCell<Number>>, Rc<RefCell<Number>>),
    Value(isize),
}

impl Number {
    pub fn phase(s: String) -> Number {
        build(&mut s.chars(), NumberBuilderState::Init)
    }

    fn new(left: Number, right: Number) -> Number {
        Pair(Rc::new(RefCell::new(left)), Rc::new(RefCell::new(right)))
    }

    fn leafs_mut(start: Rc<RefCell<Number>>) -> SnailNumberIter {
        SnailNumberIter::from(start)
    }

    fn explode_first_deep_number(&mut self) {
        self.explode_int(0);
    }

    fn explode_int(&mut self, depth: usize) -> (Option<isize>, Option<isize>) {
        todo!();
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

struct SnailNumberIter {
    current: Rc<RefCell<Number>>,
}

impl SnailNumberIter {
    fn from(root: Rc<RefCell<Number>>) -> SnailNumberIter {
        SnailNumberIter { current: root }
    }
}

impl Iterator for SnailNumberIter {
    type Item = Rc<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
                assert_eq!(*left.borrow(), Number::new(Value(1), Value(3)));
                assert_eq!(*right.borrow(), Value(2));
            }
            _ => panic!("left is not a pair"),
        }
    }

    #[test]
    fn test_input_right_nested() {
        let n = phase_str("[1,[3,2]]");

        match n {
            Pair(left, right) => {
                assert_eq!(*left.borrow(), Value(1));
                assert_eq!(*right.borrow(), Number::new(Value(3), Value(2)));
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
            "[[3,[2,[8,0]]],[9,[5,[7,[3,2]]]]]",
        );
    }
    #[test]
    fn explode_deep_nested_5() {
        assert_explotion(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn leaf_iter() {
        let n = phase_str("[1,0]");
        let mut iter = Number::leafs_mut(Rc::new(RefCell::new(n)));
        assert_eq!(iter.next(), Some(Rc::new(phase_str("[1,0]"))));
        assert_eq!(iter.next(), None);
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
