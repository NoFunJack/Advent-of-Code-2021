pub fn check(line: String) -> CheckResult {
    let mut stack = Stack::new();

    for c in line.chars() {
        match stack.push(c) {
            Ok(_) => {}
            Err(score) => return CheckResult::Corrupted(score),
        }
    }

    println!("DEBUG {:#?}", stack);
    CheckResult::Inclomplete(
        stack.autocomplete_score(),
        stack.expected_closer.iter().rev().collect(),
    )
}

#[derive(PartialEq, Debug)]
pub enum CheckResult {
    Corrupted(usize),
    Inclomplete(usize, String),
}

#[derive(Debug)]
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
                        Err(error_score_of(c))
                    }
                } else {
                    println!("expected {:?} but found {}", self.expected_closer.last(), c);
                    Err(error_score_of(c))
                }
            }
        }
    }

    fn autocomplete_score(&self) -> usize {
        let mut score = 0;

        for c in self.expected_closer.iter().rev() {
            score = (5 * score) + autocomplete_score_of(*c);
        }

        score
    }
}

fn error_score_of(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown char {}", c),
    }
}
fn autocomplete_score_of(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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
        assert_eq!(
            check("()".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("[]".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("([])".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("{()()()}".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("<([{}])>".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("[<>({}){}[([])<>]]".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
        assert_eq!(
            check("(((((((((())))))))))".to_string()),
            CheckResult::Inclomplete(0, "".to_string())
        );
    }
    #[test]
    fn incomplete_is_ok() {
        //assert_eq!(check("[".to_string()), build_incl("]", 2));
        //assert_eq!(check("<<[<>]".to_string()), build_incl(">>", 24));
        //assert_eq!(check("{{{[{}]".to_string()), build_incl("}}}", 93));
        //assert_eq!(check("([{<".to_string()), build_incl(">}])", 194));
        //assert_eq!(check("(<()<>>".to_string()), build_incl(")", 1));
        assert_eq!(
            check("[({([[{{".to_string()),
            build_incl("}}]])})]", 288957)
        );

        fn build_incl(exp_end: &str, score: usize) -> CheckResult {
            CheckResult::Inclomplete(score, exp_end.to_string())
        }
    }

    #[test]
    fn simple_examples() {
        assert_eq!(
            check("(]".to_string()),
            CheckResult::Corrupted(error_score_of(']'))
        );
        assert_eq!(
            check("{()()()>".to_string()),
            CheckResult::Corrupted(error_score_of('>'))
        );
        assert_eq!(
            check("(((()))}".to_string()),
            CheckResult::Corrupted(error_score_of('}'))
        );

        assert_eq!(
            check("<([]){()}[{}])".to_string()),
            CheckResult::Corrupted(error_score_of(')'))
        );
    }

    #[test]
    fn complex_examples() {
        assert_error(
            check("{([(<{}[<>[]}>{[]{[(<()>".to_string()),
            error_score_of('}'),
        );
        assert_error(
            check("[[<[([]))<([[{}[[()]]]".to_string()),
            error_score_of(')'),
        );
        assert_error(
            check("[{[{({}]{}}([{[{{{}}([]".to_string()),
            error_score_of(']'),
        );
        assert_error(
            check("[<(<(<(<{}))><([]([]()".to_string()),
            error_score_of(')'),
        );
        assert_error(
            check("<{([([[(<>()){}]>(<<{{".to_string()),
            error_score_of('>'),
        );

        fn assert_error(result: CheckResult, score: usize) {
            match result {
                CheckResult::Corrupted(s) => assert_eq!(s, score),
                _ => panic!("Line not corrupted"),
            }
        }
    }

    #[test]
    fn empty_ac_score() {
        let s = Stack::new();
        assert_eq!(s.autocomplete_score(), 0);
    }

    #[test]
    fn ac_score_examples() {
        assert_eq!(call_score_calc("}}]])})]"), 288957);
        assert_eq!(call_score_calc(")}>]})"), 5566);
        assert_eq!(call_score_calc("}}>}>))))"), 1480781);
        assert_eq!(call_score_calc("]]}}]}]}>"), 995444);
        assert_eq!(call_score_calc("])}>"), 294);
    }
    fn call_score_calc(input: &str) -> usize {
        let s = Stack {
            expected_closer: input.chars().rev().collect(),
        };

        return s.autocomplete_score();
    }
}
