use std::fmt;
use std::fmt::Display;

use crate::floor::Tile::*;

struct Floor {
    fields: Vec<Vec<Option<Tile>>>,
}

enum Tile {
    East,
    South,
}

impl Floor {
    fn new(floorStr: String) -> Floor {
        Floor {
            fields: floorStr
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '>' => Some(East),
                            'v' => Some(South),
                            '.' => None,
                            _ => panic!("Unkown floor tile {}", c),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut re =
            String::with_capacity(self.fields.len() * (self.fields.get(0).unwrap().len() + 1));
        for line in &self.fields {
            for tile in line {
                re.push(match tile {
                    Some(East) => '>',
                    Some(South) => 'v',
                    None => '.',
                });
            }
            re.push('\n');
        }

        write!(f, "{}", re)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn print_floor() {
        let f = Floor::new(String::from(".>\nv."));

        assert_eq!(format!("{f}"), ".>\nv.\n");
    }
}
