fn main() {
    println!("Hello, world!");
}

const HALLWAY_SIZE: usize = 11;
const ROOM_POS: [usize; 4] = [2, 4, 6, 8];

#[derive(Debug, Clone)]
struct Hallway {
    content: [Option<Pod>; HALLWAY_SIZE],
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
        let mut re = Vec::new();
        for (i, _) in self.rooms.iter().enumerate() {
            let mut clone = self.clone();
            if let Some(pod) = clone.rooms[i].get() {
                re.append(&mut move_pod_lr(clone, pod, ROOM_POS[i]));
            }
        }

        // one clone for each reachable space
        fn move_pod_lr(hw: Hallway, pod: Pod, starting_pos: usize) -> Vec<Hallway> {
            let mut in_hw = Vec::new();
            // try left
            for i in (0..starting_pos).rev() {
                if !front_of_door(i) {
                    match clone_if_free(&hw, pod, i) {
                        Some(hw_clone) => in_hw.push(hw_clone),
                        None => break,
                    }
                }
            }
            // try right
            for i in starting_pos + 1..HALLWAY_SIZE {
                if !front_of_door(i) {
                    match clone_if_free(&hw, pod, i) {
                        Some(hw_clone) => in_hw.push(hw_clone),
                        None => break,
                    }
                }
            }

            fn front_of_door(pos: usize) -> bool {
                ROOM_POS.contains(&pos)
            }

            fn clone_if_free(hw: &Hallway, pod: Pod, pos: usize) -> Option<Hallway> {
                if hw.content[pos].is_none() {
                    let mut new_hw = hw.clone();
                    new_hw.content[pos] = Some(pod);
                    Some(new_hw)
                } else {
                    None
                }
            }

            in_hw
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
            Room::new(None, None),
            Room::new(None, Some(Pod::A)),
            Room::new(None, None),
            Room::new(None, None),
        ]);
        // Amphipods will never stop on the space immediately outside any room.
        assert_eq!(hw.move_to_hw().len(), 7);
    }

    #[test]
    fn move_into_hw_with_one_pod() {
        let hw = Hallway::new([
            Room::new(None, None),
            Room::new(Some(Pod::B), Some(Pod::A)),
            Room::new(None, None),
            Room::new(None, None),
        ]);
        // if one pod blocks path, there are less spaces
        assert!(hw.move_to_hw()[0].move_to_hw().len() < 7);
    }

    #[test]
    fn move_all_pods_to_hw() {
        let mut hw = Hallway::new([
            Room::new(None, None),
            Room::new(Some(Pod::B), Some(Pod::A)),
            Room::new(None, None),
            Room::new(None, None),
        ]);

        for i in 0..2 {
            hw = match hw.move_to_hw().into_iter().next() {
                Some(h) => h,
                None => panic!("No more pods to move after {} steps", i),
            };
        }

        assert_eq!(hw.move_to_hw().len(), 0)
    }
}
