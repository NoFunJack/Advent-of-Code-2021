#[derive(Debug)]
pub struct Scanner {
    pub beacons: Vec<Beacon>,
}

impl Scanner {
    pub fn new(mut beacons: Vec<Beacon>) -> Self {
        let list_clone = beacons.clone();
        beacons.iter_mut().for_each(|b| b.set_dists(&list_clone));

        Self { beacons }
    }

    pub fn rotate(&self, f: u8, r: u8) -> Vec<Beacon> {
        self.beacons.iter().map(|b| b.rotate(f, r)).collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Beacon {
    pub pos: (i32, i32, i32),
    dists: Option<Vec<i32>>,
}

impl Beacon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: (x, y, z),
            dists: None,
        }
    }

    pub fn could_be_same(&self, other: &Beacon) -> bool {
        if let Some(my_dists) = &self.dists {
            if let Some(other_dists) = &other.dists {
                return 10
                    <= my_dists
                        .iter()
                        .filter(|my| other_dists.contains(my))
                        .count();
            }
        }
        false
    }

    fn rotate(&self, f: u8, r: u8) -> Beacon {
        let (x, y, z) = self.pos;
        let mut b = match f {
            0 => (x, y, z),
            1 => Beacon::rotate_around_axis((x, y, z), 2, 1),
            2 => Beacon::rotate_around_axis((x, y, z), 1, 1),
            3 => Beacon::rotate_around_axis((x, y, z), 1, 2),
            4 => Beacon::rotate_around_axis((x, y, z), 2, 3),
            5 => Beacon::rotate_around_axis((x, y, z), 1, 3),
            _ => panic!(),
        };

        b = Beacon::rotate_around_axis(b, f % 3, r);
        Beacon {
            pos: b,
            dists: self.dists.clone(),
        }
    }

    pub fn shift(&self, v: (i32, i32, i32)) -> Beacon {
        let (x, y, z) = self.pos;
        Beacon {
            pos: (x + v.0, y + v.1, z + v.2),
            dists: self.dists.clone(),
        }
    }

    fn rotate_around_axis(v: (i32, i32, i32), fixed: u8, r: u8) -> (i32, i32, i32) {
        let (mut x, mut y, mut z) = v;
        match fixed {
            0 => {
                let re = Beacon::twod_rotation((y, z), r);
                y = re.0;
                z = re.1;
            }
            1 => {
                let re = Beacon::twod_rotation((x, z), r);
                x = re.0;
                z = re.1;
            }
            2 => {
                let re = Beacon::twod_rotation((x, y), r);
                x = re.0;
                y = re.1;
            }
            _ => panic!(),
        }

        (x, y, z)
    }

    fn twod_rotation(x: (i32, i32), r: u8) -> (i32, i32) {
        let (a, b) = x;
        match r % 4 {
            0 => x,
            1 => (-b, a),
            2 => (-a, -b),
            3 => (b, -a),
            _ => panic!(),
        }
    }

    fn set_dists(&mut self, others: &Vec<Beacon>) {
        self.dists = Some(others.iter().map(|o| man_dist(o.pos, self.pos)).collect());
    }
}

fn man_dist(x: (i32, i32, i32), y: (i32, i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn orientations() {
        let b = Beacon::new(1, 2, 3);

        // change faceing
        assert_eq!(b.rotate(0, 0), Beacon::new(1, 2, 3));
        assert_eq!(b.rotate(1, 0), Beacon::new(-2, 1, 3));
        assert_eq!(b.rotate(2, 0), Beacon::new(-3, 2, 1));
        assert_eq!(b.rotate(3, 0), Beacon::new(-1, 2, -3));
        assert_eq!(b.rotate(4, 0), Beacon::new(2, -1, 3));
        assert_eq!(b.rotate(5, 0), Beacon::new(3, 2, -1));
        assert_eq!(b.rotate(0, 1), Beacon::new(1, -3, 2));
        assert_eq!(b.rotate(0, 2), Beacon::new(1, -2, -3));
        assert_eq!(b.rotate(0, 3), Beacon::new(1, 3, -2));
        assert_eq!(b.rotate(1, 1), Beacon::new(-3, 1, -2));
        assert_eq!(b.rotate(1, 2), Beacon::new(2, 1, -3));
        assert_eq!(b.rotate(1, 3), Beacon::new(3, 1, 2));
    }
}
