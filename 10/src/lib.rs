pub fn check(line: String) -> usize {
    let mut stack = Stack::new();

    for c in line.chars() {
        match stack.push(c) {
            Ok(_) => {}
            Err(score) => return score,
        }
    }

    0
}

struct Stack {
    expected_closer: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            expected_closer: Vec::new(),
        }
    }

    fn push(&mut self, c: char) -> Result<(), usize> {
        match closer_of(c) {
            MirrorChar::Closer(closer) => {
                self.expected_closer.push(closer);
                Ok(())
            }
            MirrorChar::IsCloser => {
                if let Some(exp_closer) = self.expected_closer.pop() {
                    if exp_closer == c {
                        return Ok(());
                    } else {
                        println!("expected {:?} but found {}", self.expected_closer.last(), c);
                        Err(score_of(c))
                    }
                } else {
                    println!("expected {:?} but found {}", self.expected_closer.last(), c);
                    Err(score_of(c))
                }
            }
        }
    }
}

fn score_of(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown char {}", c),
    }
}

fn closer_of(c: char) -> MirrorChar {
    match c {
        '(' => MirrorChar::Closer(')'),
        '[' => MirrorChar::Closer(']'),
        '{' => MirrorChar::Closer('}'),
        '<' => MirrorChar::Closer('>'),
        _ => MirrorChar::IsCloser,
    }
}

enum MirrorChar {
    Closer(char),
    IsCloser,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn legal_input_no_points() {
        assert_eq!(check("()".to_string()), 0);
        assert_eq!(check("[]".to_string()), 0);
        assert_eq!(check("([])".to_string()), 0);
        assert_eq!(check("{()()()}".to_string()), 0);
        assert_eq!(check("<([{}])>".to_string()), 0);
        assert_eq!(check("[<>({}){}[([])<>]]".to_string()), 0);
        assert_eq!(check("(((((((((())))))))))".to_string()), 0);
    }

    #[test]
    fn simple_examples() {
        assert_eq!(check("(]".to_string()), score_of(']'));
        assert_eq!(check("{()()()>".to_string()), score_of('>'));
        assert_eq!(check("(((()))}".to_string()), score_of('}'));
        assert_eq!(check("<([]){()}[{}])".to_string()), score_of(')'));
    }

    #[test]
    fn incomplete_is_ok() {
        assert_eq!(check("[".to_string()), 0);
        assert_eq!(check("<<[<>]".to_string()), 0);
        assert_eq!(check("{{{[{}]".to_string()), 0);
    }

    #[test]
    fn complex_examples() {
        assert_eq!(check("{([(<{}[<>[]}>{[]{[(<()>".to_string()), score_of('}'));
        assert_eq!(check("[[<[([]))<([[{}[[()]]]".to_string()), score_of(')'));
        assert_eq!(check("[{[{({}]{}}([{[{{{}}([]".to_string()), score_of(']'));
        assert_eq!(check("[<(<(<(<{}))><([]([]()".to_string()), score_of(')'));
        assert_eq!(check("<{([([[(<>()){}]>(<<{{".to_string()), score_of('>'));
    }
}
