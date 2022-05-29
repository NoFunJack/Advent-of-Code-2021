fn main() {
    let hw = Hallway::new(
        [
            [Some(Pod::B), Some(Pod::A)],
            [Some(Pod::C), Some(Pod::D)],
            [Some(Pod::B), Some(Pod::C)],
            [Some(Pod::D), Some(Pod::A)],
        ],
        0,
    );

    // init state
    let mut states = vec![hw];
    let mut debug_count = 0;
    loop {
        println!("States {}", states.len());

        for it in &states {
            if it.is_done() {
                println!("Solution found! {:?}", it);
                return;
            }
        }

        states = states.iter().flat_map(|h| h.get_children()).collect();

        if states.is_empty() {
            println!("no more ways to to");
            return;
        }

        debug_count += 1;
        if debug_count > 10 {
            //return;
        }
    }
}

const HALLWAY_SIZE: usize = 11;
const ROOM_POS: [usize; 4] = [2, 4, 6, 8];

#[derive(Debug, Clone, PartialEq)]
struct Hallway {
    content: [Option<Pod>; HALLWAY_SIZE],
    rooms: [Room; 4],
    energy: usize,
}

impl Hallway {
    fn new(rooms: [[Option<Pod>; 2]; 4], energy: usize) -> Hallway {
        Hallway {
            content: [None; 11],
            rooms: [
                Room::new(Pod::A, rooms[0][0], rooms[0][1]),
                Room::new(Pod::B, rooms[1][0], rooms[1][1]),
                Room::new(Pod::C, rooms[2][0], rooms[2][1]),
                Room::new(Pod::D, rooms[3][0], rooms[3][1]),
            ],
            energy,
        }
    }

    pub fn get_children(&self) -> Vec<Hallway> {
        self.move_to_hw()
    }

    pub fn is_done(&self) -> bool {
        self.rooms
            == [
                Room::new(Pod::A, Some(Pod::A), Some(Pod::A)),
                Room::new(Pod::B, Some(Pod::B), Some(Pod::B)),
                Room::new(Pod::C, Some(Pod::C), Some(Pod::C)),
                Room::new(Pod::D, Some(Pod::D), Some(Pod::D)),
            ]
    }

    fn move_to_hw(&self) -> Vec<Hallway> {
        let mut re = Vec::new();
        for (i, _) in self.rooms.iter().enumerate() {
            let mut clone = self.clone();
            if let Some((pod, cost)) = clone.rooms[i].get() {
                clone.energy += cost;
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
                        Some(mut hw_clone) => {
                            hw_clone.energy += starting_pos.checked_sub(i).unwrap() * pod.val();
                            in_hw.push(hw_clone);
                        }
                        None => break,
                    }
                }
            }
            // try right
            for i in starting_pos + 1..HALLWAY_SIZE {
                if !front_of_door(i) {
                    match clone_if_free(&hw, pod, i) {
                        Some(mut hw_clone) => {
                            hw_clone.energy += i.checked_sub(starting_pos).unwrap() * pod.val();
                            in_hw.push(hw_clone)
                        }
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

#[derive(Debug, Clone, PartialEq)]
struct Room {
    dest_for: Pod,
    content: [Option<Pod>; 2],
}

impl Room {
    fn new(dest_for: Pod, upper: Option<Pod>, lower: Option<Pod>) -> Room {
        Room {
            content: [upper, lower],
            dest_for,
        }
    }

    // returns cost
    pub fn add(&mut self, pod: Pod) -> Result<usize, ()> {
        match self.content[0] {
            None => {
                self.content[0] = Some(pod);
                Ok(2 * pod.val())
            }
            Some(_) => match self.content[1] {
                None => {
                    self.content[1] = Some(pod);
                    Ok(pod.val())
                }
                Some(_) => Err(()),
            },
        }
    }

    pub fn get(&mut self) -> Option<(Pod, usize)> {
        if self.all_pods_done() {
            return None;
        }

        if let Some(pod) = self.content[0] {
            Some((self.content[0].take().unwrap(), pod.val()))
        } else if let Some(pod) = self.content[1] {
            Some((self.content[1].take().unwrap(), 2 * pod.val()))
        } else {
            None
        }
    }

    fn all_pods_done(&self) -> bool {
        self.content
            .iter()
            .filter(|t| t.is_some())
            .all(|p| p.unwrap() == self.dest_for)
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
    fn val(&self) -> usize {
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
        let mut room = Room::new(Pod::A, None, None);

        // move 2 fields deep
        let first_insert = room.add(Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 2);

        // move 1 field deep
        let first_insert = room.add(Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 1);

        assert!(room.add(Pod::A).is_err());
    }

    #[test]
    fn get_pods_from_room() {
        let mut room = Room::new(Pod::A, Some(Pod::A), Some(Pod::B));
        assert_eq!(room.get().unwrap(), (Pod::A, 1));
        assert_eq!(room.get().unwrap(), (Pod::B, 20));
        assert_eq!(room.get(), None);
    }

    #[test]
    fn do_not_take_out_finished_pod() {
        let mut room = Room::new(Pod::A, None, Some(Pod::A));
        assert_eq!(room.get(), None);
        let mut room = Room::new(Pod::A, Some(Pod::A), Some(Pod::A));
        assert_eq!(room.get(), None);
    }

    #[test]
    fn dont_block_other_non_test_pod() {
        let mut room = Room::new(Pod::A, Some(Pod::A), Some(Pod::B));
        assert_eq!(room.get(), Some((Pod::A, 1)));
    }

    #[test]
    fn move_into_empty_hw() {
        let hw = Hallway::new(
            [
                [None, None],
                [None, Some(Pod::A)],
                [None, None],
                [None, None],
            ],
            0,
        );
        // Amphipods will never stop on the space immediately outside any room.
        let children = hw.move_to_hw();
        assert_eq!(children.len(), 7);
        list_contains_n_of_cost(&children, 2, 3);
        list_contains_n_of_cost(&children, 2, 5);
        list_contains_n_of_cost(&children, 1, 6);
        list_contains_n_of_cost(&children, 1, 7);
        list_contains_n_of_cost(&children, 1, 8);
    }

    fn list_contains_n_of_cost(children: &[Hallway], expected_num: usize, expected_cost: usize) {
        assert_eq!(
            children
                .iter()
                .filter(|&h| h.energy == expected_cost)
                .count(),
            expected_num,
            "expcted {} with energy {} in:\n{:#?}",
            expected_num,
            expected_cost,
            children
        )
    }

    #[test]
    fn move_into_hw_with_one_pod() {
        let hw = Hallway::new(
            [
                [None, None],
                [Some(Pod::B), Some(Pod::A)],
                [None, None],
                [None, None],
            ],
            0,
        );
        // if one pod blocks path, there are less spaces
        assert!(hw.move_to_hw()[0].move_to_hw().len() < 7);
    }

    #[test]
    fn move_all_pods_to_hw() {
        let mut hw = Hallway::new(
            [
                [None, None],
                [Some(Pod::B), Some(Pod::A)],
                [None, None],
                [None, None],
            ],
            0,
        );

        for i in 0..2 {
            hw = match hw.move_to_hw().into_iter().next() {
                Some(h) => h,
                None => panic!("No more pods to move after {} steps", i),
            };
        }

        assert_eq!(hw.move_to_hw().len(), 0)
    }
}