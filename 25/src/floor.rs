use std::fmt;
use std::fmt::Display;

use crate::floor::Tile::*;

pub struct Floor {
    rowsize: usize,
    fields: Vec<Option<Tile>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
        println!("~move east");
        for i in (0..self.fields.len()).step_by(self.rowsize) {
            Floor::advance(&mut self.fields[i..i + self.rowsize], 1, Tile::East);
        }
    }

    fn move_south(&mut self) {
        println!("~move south");
        for i in 0..self.rowsize {
            Floor::advance(&mut self.fields[i..], self.rowsize, Tile::South);
        }
    }

    fn advance(cucumbers: &mut [Option<Tile>], stepsize: usize, moving: Tile) {
        let mut i = 0;
        while i < cucumbers.len() {
            let j;
            println!("DEBUG {} {stepsize}", cucumbers.len());
            if i + stepsize >= cucumbers.len() {
                print!("wrap: ");
                j = i % stepsize;
            } else {
                j = i + stepsize;
            }
            println!("{}/{} {:?}:{:?}", i, j, cucumbers[i], cucumbers[j]);
            if cucumbers[i] == Some(moving) {
                if let None = cucumbers[j] {
                    println!("movement");
                    cucumbers[i] = None;
                    cucumbers[j] = Some(moving);
                    // skip next one
                    i += stepsize;
                }
            }
            i += stepsize;
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
        assert_eq!(format!("{f}"), ".\nv\n.\nv\n.\n");
        f.step();
        assert_eq!(format!("{f}"), ".\n.\nv\n.\nv\n");
        f.step();
        assert_eq!(format!("{f}"), "v\n.\n.\nv\n.\n");
    }

    #[test]
    fn lr_wrap() {
        let mut f = Floor::new(String::from("..>"));

        f.step();
        assert_eq!(format!("{f}"), ">..\n");
    }

    #[test]
    fn up_down_wrap() {
        let mut f = Floor::new(String::from(".\n.\nv"));

        f.step();
        assert_eq!(format!("{f}"), "v\n.\n.\n");
    }

    #[test]
    fn website_example_1() {
        let mut f = Floor::new(String::from(
            "...>...\n\
             .......\n\
             ......>\n\
             v.....>\n\
             ......>\n\
             .......\n\
             ..vvv..",
        ));

        f.step();
        assert_eq!(
            format!("{f}"),
            "..vv>..\n\
             .......\n\
             >......\n\
             v.....>\n\
             >......\n\
             .......\n\
             ....v..\n"
        );
        f.step();
        assert_eq!(
            format!("{f}"),
            "....v>.\n\
             ..vv...\n\
             .>.....\n\
             ......>\n\
             v>.....\n\
             .......\n\
             .......\n"
        );
        f.step();
        assert_eq!(
            format!("{f}"),
            "......>\n\
             ..v.v..\n\
             ..>v...\n\
             >......\n\
             ..>....\n\
             v......\n\
             .......\n"
        );
        f.step();
        assert_eq!(
            format!("{f}"),
            ">......\n\
             ..v....\n\
             ..>.v..\n\
             .>.v...\n\
             ...>...\n\
             .......\n\
             v......\n"
        );
    }
}
