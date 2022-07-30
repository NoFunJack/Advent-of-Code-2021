use std::fmt;
use std::fmt::Display;

use crate::floor::Tile::*;

pub struct Floor {
    fields: Vec<Vec<Option<Tile>>>,
}

#[derive(Debug)]
pub enum Tile {
    East,
    South,
}

impl Floor {
    pub fn new(floorStr: String) -> Floor {
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

    pub fn step(&mut self) {
        self.move_east();
    }

    fn move_east(&mut self) {
        for row in &mut self.fields {
            Floor::advance(row);
        }
    }

    fn advance(cucumbers: &mut [Option<Tile>]) {
        let mut i = 0;
        while i < cucumbers.len() {
            let j;
            if i == cucumbers.len() - 1 {
                j = 0;
            } else {
                j = i + 1;
            }
            println!("{}/{} {:?}:{:?}", i, j, cucumbers[i], cucumbers[j]);
            if let Some(East) = cucumbers[i] {
                if let None = cucumbers[j] {
                    println!("movement");
                    cucumbers[i] = None;
                    cucumbers[j] = Some(East);
                    // skip next one
                    i += 1;
                }
            }
            i += 1;
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

    #[test]
    fn move_east() {
        let mut f = Floor::new(String::from("...>>>>>..."));

        f.step();
        assert_eq!(format!("{f}"), "...>>>>.>..\n");
        f.step();
        assert_eq!(format!("{f}"), "...>>>.>.>.\n");
    }
}
