use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
pub struct Scanner {
    pub beacons: Vec<Beacon>,
    min: (i32, i32, i32),
    max: (i32, i32, i32),
}

impl Scanner {
    pub fn new(beacons: Vec<Beacon>) -> Self {
        let min = beacons.iter().fold((0, 0, 0), |a, b| {
            (min(a.0, b.0), min(a.1, b.1), min(a.2, b.2))
        });
        let max = beacons.iter().fold((0, 0, 0), |a, b| {
            (max(a.0, b.0), max(a.1, b.1), max(a.2, b.2))
        });
        Self { beacons, min, max }
    }

    pub fn rotate(&self, f: u8, r: u8) -> Vec<Beacon> {
        self.beacons.iter().map(|b| b.rotate(f, r)).collect()
    }

    pub fn is_in_range(&self, b: Beacon) -> bool {
        self.min.0 <= b.0
            && self.min.1 <= b.1
            && self.min.2 <= b.2
            && self.max.0 >= b.0
            && self.max.1 >= b.1
            && self.max.2 >= b.2
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Beacon(pub i32, pub i32, pub i32);

impl Beacon {
    fn rotate(&self, f: u8, r: u8) -> Beacon {
        let Beacon(x, y, z) = self.clone();
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
        Beacon(b.0, b.1, b.2)
    }

    pub fn shift(&self, v: (i32, i32, i32)) -> Beacon {
        let Beacon(x, y, z) = self;
        Beacon(x + v.0, y + v.1, z + v.2)
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn orientations() {
        let b = Beacon(1, 2, 3);

        // change faceing
        assert_eq!(b.rotate(0, 0), Beacon(1, 2, 3));
        assert_eq!(b.rotate(1, 0), Beacon(-2, 1, 3));
        assert_eq!(b.rotate(2, 0), Beacon(-3, 2, 1));
        assert_eq!(b.rotate(3, 0), Beacon(-1, 2, -3));
        assert_eq!(b.rotate(4, 0), Beacon(2, -1, 3));
        assert_eq!(b.rotate(5, 0), Beacon(3, 2, -1));
        assert_eq!(b.rotate(0, 1), Beacon(1, -3, 2));
        assert_eq!(b.rotate(0, 2), Beacon(1, -2, -3));
        assert_eq!(b.rotate(0, 3), Beacon(1, 3, -2));
        assert_eq!(b.rotate(1, 1), Beacon(-3, 1, -2));
        assert_eq!(b.rotate(1, 2), Beacon(2, 1, -3));
        assert_eq!(b.rotate(1, 3), Beacon(3, 1, 2));
    }
}
