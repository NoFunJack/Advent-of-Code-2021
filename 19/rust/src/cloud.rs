use crate::scanner::{Beacon, Scanner};

#[derive(Debug)]
pub struct Cloud {
    pub beacons: Vec<Beacon>,
    req_number: usize,
}

impl Cloud {
    pub fn new(scanner: Scanner) -> Cloud {
        Cloud {
            beacons: scanner.beacons,
            req_number: 12,
            scanner_pos: Vec<(i32,i32,i32)>,
        }
    }
    pub fn new_var_match(scanner: Scanner, req_number: usize) -> Cloud {
        Cloud {
            beacons: scanner.beacons,
            req_number,
        }
    }

    pub fn add_scanner(&mut self, scanner: &Scanner) -> Result<(), ()> {
        let mut candidates = Vec::new();
        for (cbi, cb) in self.beacons.iter().enumerate() {
            for (sbi, sb) in scanner.beacons.iter().enumerate() {
                if cb.could_be_same(sb) {
                    candidates.push((cbi, sbi))
                }
            }
        }
        for can in candidates {
            if self.add_scanner_int(scanner, can.0, can.1).is_ok() {
                return Ok(());
            }
        }
        Err(())
    }

    fn add_scanner_int(&mut self, scanner: &Scanner, cbi: usize, sbi: usize) -> Result<(), ()> {
        for f in 0..=5 {
            for r in 0..=3 {
                if self.add_beacons(scanner.rotate(f, r), cbi, sbi).is_ok() {
                    return Ok(());
                }
            }
        }
        Err(())
    }

    fn add_beacons(
        &mut self,
        other_beacons: Vec<Beacon>,
        cbi: usize,
        sbi: usize,
    ) -> Result<(), ()> {
        let cloud_ancor = self.beacons.get(cbi).unwrap();
        let other = other_beacons.get(sbi).unwrap();
        let (c1, c2, c3) = cloud_ancor.pos;
        let (n1, n2, n3) = other.pos;
        let shift = (c1 - n1, c2 - n2, c3 - n3);
        let (match_beacons, extra_beacons): (Vec<&Beacon>, Vec<&Beacon>) =
            other_beacons.iter().partition(|other| {
                self.beacons
                    .iter()
                    .any(|my| other.shift(shift).pos == my.pos)
            });

        if match_beacons.len() >= self.req_number {
            extra_beacons
                .into_iter()
                .map(|b| b.shift(shift))
                .for_each(|b| self.beacons.push(b));

            return Ok(());
        }
        Err(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cloud::Cloud,
        scanner::{Beacon, Scanner},
    };

    #[test]
    fn match_trans() {
        let mut cloud = Cloud::new_var_match(
            Scanner {
                beacons: vec![Beacon::new(1, 0, 0), Beacon::new(0, 0, 0)],
            },
            2,
        );
        let add_scan = Scanner {
            beacons: vec![
                Beacon::new(1, 5, 0),
                Beacon::new(0, 5, 0),
                Beacon::new(-1, 5, 0),
            ],
        };
        assert!(cloud.add_scanner(&add_scan).is_ok());
        assert_eq!(
            cloud.beacons,
            vec![
                Beacon::new(1, 0, 0),
                Beacon::new(0, 0, 0),
                Beacon::new(-1, 0, 0)
            ]
        )
    }

    #[test]
    fn match_rot() {
        let mut cloud = Cloud::new_var_match(
            Scanner {
                beacons: vec![
                    Beacon::new(3, 0, 0),
                    Beacon::new(1, 1, 0),
                    Beacon::new(0, 0, 0),
                ],
            },
            3,
        );
        let add_scan = Scanner {
            beacons: vec![
                Beacon::new(0, 0, 0),
                Beacon::new(-1, 1, 0),
                Beacon::new(0, 3, 0),
                Beacon::new(5, 0, 0),
            ],
        };
        // after rot (1,0,0) (2,0,0) (4,0,0) (0,-5,0)
        // after trans[x-1] (0,0,0) (1,0,0),(3,0,0),(-1,-5,0)
        assert!(cloud.add_scanner(&add_scan).is_ok());
        assert_eq!(
            cloud.beacons,
            vec![
                Beacon::new(3, 0, 0),
                Beacon::new(1, 1, 0),
                Beacon::new(0, 0, 0),
                Beacon::new(0, -5, 0)
            ]
        )
    }

    #[test]
    fn match_rot_and_trans() {
        let mut cloud = Cloud::new_var_match(
            Scanner {
                beacons: vec![
                    Beacon::new(3, 0, 0),
                    Beacon::new(1, 1, 0),
                    Beacon::new(0, 0, 0),
                ],
            },
            3,
        );
        let add_scan = Scanner {
            beacons: vec![
                Beacon::new(0, 1, 0),
                Beacon::new(-1, 2, 0),
                Beacon::new(0, 4, 0),
                Beacon::new(5, 0, 0),
            ],
        };
        // after rot (1,0,0) (2,0,0) (4,0,0) (0,-5,0)
        // after trans[x-1] (0,0,0) (1,0,0),(3,0,0),(-1,-5,0)
        assert!(cloud.add_scanner(&add_scan).is_ok());
        assert_eq!(
            cloud.beacons,
            vec![
                Beacon::new(3, 0, 0),
                Beacon::new(1, 1, 0),
                Beacon::new(0, 0, 0),
                Beacon::new(-1, -5, 0)
            ]
        )
    }
}
