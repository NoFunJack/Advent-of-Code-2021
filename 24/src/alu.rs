#[derive(Debug)]
pub struct Memory {
    pub w: i64,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn update(&mut self, reg: char, data: i64) {
        match reg {
            'w' => self.w = data,
            'x' => self.x = data,
            'y' => self.y = data,
            'z' => self.z = data,
            _ => panic!("unkown register: {}", reg),
        }
    }
    fn get_vall(&self, ls: &LeftSide) -> i64 {
        match ls {
            LeftSide::Value(v) => *v,
            LeftSide::Addr(r) => self.get_valr(*r),
        }
    }

    fn get_valr(&self, reg: char) -> i64 {
        match reg {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("unkown register: {}", reg),
        }
    }
}

pub enum Instuction {
    Inp(char),
    Add(char, LeftSide),
    Mul(char, LeftSide),
    Div(char, LeftSide),
    Mod(char, LeftSide),
    Eql(char, LeftSide),
}

// Left side of operator can be value or adress
pub enum LeftSide {
    Value(i64),
    Addr(char),
}

pub struct ALU {
    inst: Vec<Instuction>,
}

impl ALU {
    pub fn new(instructions: String) -> ALU {
        ALU {
            inst: instructions.lines().map(|s| ALU::read_inst(s)).collect(),
        }
    }

    pub fn input(&mut self, data: String) -> Memory {
        let mut mem = Memory::new();
        let mut vals = data.chars();

        for inst in &self.inst {
            match inst {
                Instuction::Inp(r) => {
                    let next_input = i64::from(
                        vals.next()
                            .expect("more input expected")
                            .to_digit(10)
                            .unwrap(),
                    );
                    mem.update(*r, next_input);
                }
                Instuction::Add(r, l) => mem.update(*r, mem.get_valr(*r) + mem.get_vall(l)),
                Instuction::Mul(r, l) => mem.update(*r, mem.get_valr(*r) * mem.get_vall(l)),
                Instuction::Div(r, l) => mem.update(*r, mem.get_valr(*r) / mem.get_vall(l)),
                Instuction::Mod(r, l) => mem.update(*r, mem.get_valr(*r) % mem.get_vall(l)),
                Instuction::Eql(r, l) => mem.update(
                    *r,
                    match mem.get_valr(*r) == mem.get_vall(l) {
                        true => 1,
                        false => 0,
                    },
                ),
            }
        }

        mem
    }

    fn read_inst(s: &str) -> Instuction {
        fn nextchar(word: Option<&str>) -> char {
            word.unwrap().chars().next().unwrap()
        }

        fn nextval(word: Option<&str>) -> LeftSide {
            let word = word.unwrap();
            let c = word.chars().next().unwrap();
            match c {
                'w'..='z' => LeftSide::Addr(c),
                _ => LeftSide::Value(word.parse().unwrap()),
            }
        }

        let mut parts = s.split(' ');

        match parts.next() {
            Some("inp") => Instuction::Inp(nextchar(parts.next())),
            Some("add") => Instuction::Add(nextchar(parts.next()), nextval(parts.next())),
            Some("mul") => Instuction::Mul(nextchar(parts.next()), nextval(parts.next())),
            Some("div") => Instuction::Div(nextchar(parts.next()), nextval(parts.next())),
            Some("mod") => Instuction::Mod(nextchar(parts.next()), nextval(parts.next())),
            Some("eql") => Instuction::Eql(nextchar(parts.next()), nextval(parts.next())),
            Some(i) => panic!("unknown instruction: {}", i),
            _ => panic!("no first word"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_mem(actual: Memory, expected: [i64; 4]) {
        println!("{:?}", actual);
        assert_eq!(actual.w, expected[0], "wrong value at reg w");
        assert_eq!(actual.x, expected[1], "wrong value at reg x");
        assert_eq!(actual.y, expected[2], "wrong value at reg y");
        assert_eq!(actual.z, expected[3], "wrong value at reg z");
    }

    #[test]
    fn inp_w() {
        let result = build("inp w", "1");

        check_mem(result, [1, 0, 0, 0])
    }

    #[test]
    fn inp_x() {
        let result = build("inp x", "2");

        check_mem(result, [0, 2, 0, 0])
    }

    #[test]
    fn inp_y() {
        let result = build("inp y", "3");

        check_mem(result, [0, 0, 3, 0])
    }

    #[test]
    fn inp_z() {
        let result = build("inp z", "5");

        check_mem(result, [0, 0, 0, 5])
    }

    #[test]
    fn inp_many() {
        let result = build("inp w\ninp x\ninp y\ninp z", "9876");

        check_mem(result, [9, 8, 7, 6])
    }

    #[test]
    fn input_left_to_right() {
        let result = build("inp w\ninp w", "12");
        assert_eq!(result.w, 2);
    }

    #[test]
    fn add_pos() {
        check_mem(build("add w 6", ""), [6, 0, 0, 0]);
        check_mem(build("add x 5", ""), [0, 5, 0, 0]);
        check_mem(build("add w 6\nadd w 4", ""), [10, 0, 0, 0]);
    }

    #[test]
    fn add_neg() {
        check_mem(build("add w -6", ""), [-6, 0, 0, 0]);
        check_mem(build("add x -5", ""), [0, -5, 0, 0]);
        check_mem(build("add w 6\nadd w -4", ""), [2, 0, 0, 0]);
    }

    #[test]
    fn add_reg() {
        check_mem(build("inp z\ninp y\nadd z y", "23"), [0, 0, 3, 5]);
    }
    #[test]
    fn mul_reg() {
        check_mem(build("inp z\ninp y\nmul z y", "23"), [0, 0, 3, 6]);
    }

    #[test]
    fn div_reg() {
        check_mem(build("inp z\ninp y\ndiv z y", "93"), [0, 0, 3, 3]);
    }

    #[test]
    fn div_rounding() {
        check_mem(build("inp y\ndiv y 2", "5"), [0, 0, 2, 0]);
        check_mem(build("inp y\ndiv y -2", "5"), [0, 0, -2, 0]);
    }

    #[test]
    fn mod_reg() {
        check_mem(build("inp z\ninp y\nmod z y", "95"), [0, 0, 5, 4]);
    }

    #[test]
    fn eql_reg() {
        check_mem(build("inp z\ninp y\neql z y", "95"), [0, 0, 5, 0]);
        check_mem(build("inp z\ninp y\neql z y", "99"), [0, 0, 9, 1]);
    }

    #[test]
    fn ex_inverter() {
        let mut alu = ALU::new(String::from("inp x\nmul x -1"));

        check_mem(alu.input(String::from("0")), [0, 0, 0, 0]);
        check_mem(alu.input(String::from("3")), [0, -3, 0, 0]);
    }

    #[test]
    fn ex_comperator() {
        let mut alu = ALU::new(String::from(
            "inp z\n\
            inp x\n\
            mul z 3\n\
            eql z x",
        ));

        assert_eq!(alu.input(String::from("22")).z, 0);
        assert_eq!(alu.input(String::from("26")).z, 1);
        assert_eq!(alu.input(String::from("27")).z, 0);
    }

    #[test]
    fn ex_binary() {
        let mut alu = ALU::new(String::from(
            "inp w\n\
        add z w\n\
        mod z 2\n\
        div w 2\n\
        add y w\n\
        mod y 2\n\
        div w 2\n\
        add x w\n\
        mod x 2\n\
        div w 2\n\
        mod w 2",
        ));

        check_mem(alu.input(String::from("0")), [0, 0, 0, 0]);
        check_mem(alu.input(String::from("3")), [0, 0, 1, 1]);
        check_mem(alu.input(String::from("8")), [1, 0, 0, 0]);
    }

    fn build(instr: &str, input: &str) -> Memory {
        let mut alu = ALU::new(String::from(instr));
        return alu.input(String::from(input));
    }
}
