fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Hallway {
    content: [Option<Pod>; 11],
    rooms: [Room; 4],
}

impl Hallway {
    fn new(rooms: [Room; 4]) -> Hallway {
        Hallway {
            content: [None; 11],
            rooms,
        }
    }

    fn move_to_hw(&self) -> Vec<Hallway> {
        let re = Vec::new();
        for (i, _) in self.rooms.iter().enumerate() {
            let mut clone = self.clone();
            if let Some(pod) = clone.rooms[i].get() {
                // one clone for each reachable space
                todo!();
            }
        }

        re
    }
}

#[derive(Debug, Clone)]
struct Room {
    content: [Option<Pod>; 2],
}

impl Room {
    fn new(upper: Option<Pod>, lower: Option<Pod>) -> Room {
        Room {
            content: [upper, lower],
        }
    }

    pub fn add(&mut self, pod: Pod) -> Result<(), ()> {
        match self.content[0] {
            None => {
                self.content[0] = Some(pod);
                Ok(())
            }
            Some(_) => match self.content[1] {
                None => {
                    self.content[1] = Some(pod);
                    Ok(())
                }
                Some(_) => Err(()),
            },
        }
    }

    pub fn get(&mut self) -> Option<Pod> {
        self.content[0].take().or_else(|| self.content[1].take())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn val(&self) -> u32 {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn pod_values() {
        assert_eq!(Pod::A.val(), 1);
        assert_eq!(Pod::B.val(), 10);
        assert_eq!(Pod::C.val(), 100);
        assert_eq!(Pod::D.val(), 1000);
    }

    #[test]
    fn two_pods_per_room() {
        let mut room = Room::new(None, None);
        assert!(room.add(Pod::A).is_ok());
        assert!(room.add(Pod::A).is_ok());
        assert!(room.add(Pod::A).is_err());
    }

    #[test]
    fn get_pods_from_room() {
        let mut room = Room::new(Some(Pod::A), Some(Pod::B));
        assert_eq!(room.get().unwrap(), Pod::A);
        assert_eq!(room.get().unwrap(), Pod::B);
        assert_eq!(room.get(), None);
    }

    #[test]
    fn move_into_empty_hw() {
        let hw = Hallway::new([
            Room::new(None, Some(Pod::A)),
            Room::new(None, None),
            Room::new(None, None),
            Room::new(None, None),
        ]);
        // Amphipods will never stop on the space immediately outside any room.
        assert_eq!(hw.move_to_hw().len(), 7);
    }
}
