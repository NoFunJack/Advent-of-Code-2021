use std::fmt;
use std::fmt::Display;

use crate::floor::Tile::*;

pub struct Floor {
    rowsize: usize,
    fields: Vec<Option<Tile>>,
}

#[derive(Debug)]
pub enum Tile {
    East,
    South,
}

impl Floor {
    pub fn new(floorStr: String) -> Floor {
        Floor {
            rowsize: floorStr.find("\n").unwrap_or(floorStr.len()),
            fields: floorStr
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '>' => Some(East),
                            'v' => Some(South),
                            '.' => None,
                            _ => panic!("Unkown floor tile {}", c),
                        })
                        .collect::<Vec<Option<Tile>>>()
                })
                .collect(),
        }
    }

    pub fn step(&mut self) {
        self.move_east();
        self.move_south();
    }

    fn move_east(&mut self) {
        for i in (0..self.rowsize).step_by(self.rowsize) {
            Floor::advance(&mut self.fields[i..i + self.rowsize]);
        }
    }

    fn move_south(&mut self) {
        todo!("move south");
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
        let mut re = String::with_capacity(self.fields.len() + (self.fields.len() / self.rowsize));
        for (i, tile) in self.fields.iter().enumerate() {
            re.push(match tile {
                Some(East) => '>',
                Some(South) => 'v',
                None => '.',
            });
            if (i + 1) % self.rowsize == 0 {
                re.push('\n');
            }
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

    #[test]
    fn move_south() {
        let mut f = Floor::new(String::from(".\nv\nv\n.\n."));

        f.step();
        assert_eq!(format!("{f}"), ".\n.\nv\nv\n.\n");
        f.step();
        assert_eq!(format!("{f}"), ".\n.\nv\n.\nv\n");
        f.step();
        assert_eq!(format!("{f}"), "v\n.\n.\nv\n.\n");
    }
}
