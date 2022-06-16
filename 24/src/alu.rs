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
}

pub enum Instuction {
    Inp(char),
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
                Instuction::Inp(r) => ALU::do_inp(&mut mem, vals.next().unwrap(), *r),
            }
        }

        mem
    }

    fn do_inp(mem: &mut Memory, data: char, reg: char) {
        let int = i64::from(data.to_digit(10).unwrap());
        match reg {
            'w' => mem.w = int,
            'x' => mem.x = int,
            'y' => mem.y = int,
            'z' => mem.z = int,
            _ => panic!("unkown register: {}", reg),
        }
    }

    fn read_inst(s: &str) -> Instuction {
        let mut parts = s.split(' ');
        match parts.next() {
            Some("inp") => Instuction::Inp(parts.next().unwrap().chars().next().unwrap()),
            Some(i) => panic!("unknown instruction: {}", i),
            _ => panic!("no first word"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_mem(actual: Memory, expected: [i64; 4]) {
        assert_eq!(actual.w, expected[0], "wrong value at reg w");
        assert_eq!(actual.x, expected[1], "wrong value at reg x");
        assert_eq!(actual.y, expected[2], "wrong value at reg y");
        assert_eq!(actual.z, expected[3], "wrong value at reg z");
    }

    #[test]
    fn inp_w() {
        let mut alu = ALU::new(String::from("inp w"));
        let result = alu.input(String::from("1"));

        check_mem(result, [1, 0, 0, 0])
    }

    #[test]
    fn inp_x() {
        let mut alu = ALU::new(String::from("inp x"));
        let result = alu.input(String::from("2"));

        check_mem(result, [0, 2, 0, 0])
    }

    #[test]
    fn inp_y() {
        let mut alu = ALU::new(String::from("inp y"));
        let result = alu.input(String::from("3"));

        check_mem(result, [0, 0, 3, 0])
    }

    #[test]
    fn inp_z() {
        let mut alu = ALU::new(String::from("inp z"));
        let result = alu.input(String::from("5"));

        check_mem(result, [0, 0, 0, 5])
    }

    #[test]
    fn inp_many() {
        let mut alu = ALU::new(String::from("inp w\ninp x\ninp y\ninp z"));
        let result = alu.input(String::from("9876"));

        check_mem(result, [9, 8, 7, 6])
    }
}
