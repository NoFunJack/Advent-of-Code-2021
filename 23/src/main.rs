use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let hw = Hallway::new(
        [
            [Some(Pod::D), Some(Pod::D), Some(Pod::A), Some(Pod::A)],
            [Some(Pod::C), Some(Pod::C), Some(Pod::B), Some(Pod::B)],
            [Some(Pod::A), Some(Pod::B), Some(Pod::C), Some(Pod::C)],
            [Some(Pod::B), Some(Pod::A), Some(Pod::D), Some(Pod::D)],
        ],
        0,
    );

    // init state
    let mut states = HashSet::new();
    states.insert(hw);
    loop {
        println!("States {}", states.len());

        let s = take_save(&mut states);
        println!("{:?}", s.energy);
        if s.is_done() {
            println!("Solution found! {:?}", s);
            return;
        } else {
            for c in s.get_children() {
                states.insert(c);
            }
        }

        //if s.energy > 11000 {
        //    println!("{:?}", s);
        //    return;
        //}
    }
}

fn take_save(set: &mut HashSet<Hallway>) -> Hallway {
    let min = set
        .iter()
        .min_by_key(|hw| hw.energy)
        .cloned()
        .expect("No more States");

    set.remove(&min);
    min
}

const HALLWAY_SIZE: usize = 11;
const ROOM_POS: [usize; 4] = [2, 4, 6, 8];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hallway {
    content: [Option<Pod>; HALLWAY_SIZE],
    rooms: [Room; 4],
    energy: usize,
}

impl Hallway {
    fn new(rooms: [[Option<Pod>; 4]; 4], energy: usize) -> Hallway {
        Hallway {
            content: [None; 11],
            rooms: [
                Room::new(rooms[0][0], rooms[0][1], rooms[0][2], rooms[0][3]),
                Room::new(rooms[1][0], rooms[1][1], rooms[1][2], rooms[1][3]),
                Room::new(rooms[2][0], rooms[2][1], rooms[2][2], rooms[2][3]),
                Room::new(rooms[3][0], rooms[3][1], rooms[3][2], rooms[3][3]),
            ],
            energy,
        }
    }

    pub fn get_children(&self) -> Vec<Hallway> {
        match self.move_to_room() {
            Some(hw) => vec![hw],
            None => self.move_to_hw(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.rooms
            == [
                Room::new(Some(Pod::A), Some(Pod::A), Some(Pod::A), Some(Pod::A)),
                Room::new(Some(Pod::B), Some(Pod::B), Some(Pod::B), Some(Pod::B)),
                Room::new(Some(Pod::C), Some(Pod::C), Some(Pod::C), Some(Pod::C)),
                Room::new(Some(Pod::D), Some(Pod::D), Some(Pod::D), Some(Pod::D)),
            ]
    }

    fn move_to_hw(&self) -> Vec<Hallway> {
        let mut re = Vec::new();
        for (i, _) in self.rooms.iter().enumerate() {
            let mut clone = self.clone();
            if let Some((pod, cost)) = clone.rooms[i].get(Hallway::room_dest(i)) {
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

    fn move_to_room(&self) -> Option<Hallway> {
        for (i, tile) in self.content.iter().enumerate() {
            if let Some(pod) = tile {
                if let Ok(to_room_cost) = self.pod_can_go_to_room(pod, i) {
                    let mut clone = self.clone();
                    clone.content[i] = None;
                    let room = &mut clone.rooms[Hallway::room_idx(*pod)];
                    if let Some(cost) = room.add(*pod, *pod).ok() {
                        clone.energy += cost + to_room_cost;
                        return Some(clone);
                    }
                }
            }
        }
        None
    }

    fn pod_can_go_to_room(&self, pod: &Pod, from: usize) -> Result<usize, ()> {
        let door_idx = ROOM_POS[Hallway::room_idx(*pod)];
        let range = match from.cmp(&door_idx) {
            Ordering::Less => (from + 1)..door_idx,
            Ordering::Equal => 0..0,
            Ordering::Greater => door_idx..from,
        };

        for i in range {
            if let Some(_) = self.content[i] {
                return Err(());
            }
        }
        Ok(from.abs_diff(door_idx) * pod.val())
    }

    fn room_idx(pod: Pod) -> usize {
        match pod {
            Pod::A => 0,
            Pod::B => 1,
            Pod::C => 2,
            Pod::D => 3,
        }
    }

    fn room_dest(i: usize) -> Pod {
        match i {
            0 => Pod::A,
            1 => Pod::B,
            2 => Pod::C,
            3 => Pod::D,
            _ => panic!("unknown room number"),
        }
    }
}

impl PartialOrd for Hallway {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.energy.cmp(&self.energy))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Room {
    content: [Option<Pod>; 4],
}

impl Room {
    fn new(
        upper: Option<Pod>,
        up_mid: Option<Pod>,
        down_mid: Option<Pod>,
        lower: Option<Pod>,
    ) -> Room {
        Room {
            content: [upper, up_mid, down_mid, lower],
        }
    }

    // returns cost
    pub fn add(&mut self, pod: Pod, ok_pod: Pod) -> Result<usize, ()> {
        if pod != ok_pod {
            return Err(());
        }

        for (i, tile) in self.content.clone().iter().enumerate().rev() {
            match tile {
                None => {
                    self.content[i] = Some(pod);
                    return Ok((i + 1) * pod.val());
                }
                Some(occ_pod) => {
                    if *occ_pod != ok_pod {
                        return Err(());
                    }
                }
            }
        }

        return Err(());
    }

    pub fn get(&mut self, ok_pod: Pod) -> Option<(Pod, usize)> {
        if self.all_pods_done(ok_pod) {
            return None;
        }

        for (i, tile) in self.content.clone().iter().enumerate() {
            if let Some(occ_pod) = tile {
                self.content[i] = None;
                return Some((*occ_pod, (i + 1) * occ_pod.val()));
            }
        }
        None
    }

    fn all_pods_done(&self, dest: Pod) -> bool {
        self.content
            .iter()
            .filter(|t| t.is_some())
            .all(|p| p.unwrap() == dest)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
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
        let mut room = Room::new(None, None, None, None);

        // move 4 fields deep
        let first_insert = room.add(Pod::A, Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 4);

        // move 3 field deep
        let first_insert = room.add(Pod::A, Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 3);

        // move 2 fields deep
        let first_insert = room.add(Pod::A, Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 2);

        // move 1 field deep
        let first_insert = room.add(Pod::A, Pod::A);
        assert!(first_insert.is_ok());
        assert_eq!(first_insert.unwrap(), 1);

        assert!(room.add(Pod::A, Pod::A).is_err());
    }

    #[test]
    fn get_pods_from_room() {
        let mut room = Room::new(Some(Pod::A), Some(Pod::B), Some(Pod::D), Some(Pod::C));
        assert_eq!(room.get(Pod::C).unwrap(), (Pod::A, 1));
        assert_eq!(room.get(Pod::C).unwrap(), (Pod::B, 20));
        assert_eq!(room.get(Pod::C).unwrap(), (Pod::D, 3000));
        assert_eq!(room.get(Pod::A).unwrap(), (Pod::C, 400));
        assert_eq!(room.get(Pod::C), None);
    }

    #[test]
    fn if_all_pods_ok_dont_give_1() {
        let mut room = Room::new(None, None, None, Some(Pod::A));
        assert_eq!(room.get(Pod::A), None);
    }

    #[test]
    fn if_all_pods_ok_dont_give_2() {
        let mut room = Room::new(None, None, Some(Pod::A), Some(Pod::A));
        assert_eq!(room.get(Pod::A), None);
    }

    #[test]
    fn if_all_pods_ok_dont_give_3() {
        let mut room = Room::new(None, Some(Pod::A), Some(Pod::A), Some(Pod::A));
        assert_eq!(room.get(Pod::A), None);
    }

    #[test]
    fn if_all_pods_ok_dont_give_4() {
        let mut room = Room::new(Some(Pod::A), Some(Pod::A), Some(Pod::A), Some(Pod::A));
        assert_eq!(room.get(Pod::A), None);
    }

    #[test]
    fn dont_add_to_room_if_other_pod_occupies() {
        let mut room = Room::new(None, None, None, Some(Pod::B));
        assert!(room.add(Pod::A, Pod::A).is_err());
    }

    #[test]
    fn dont_add_pod_to_wrong_room() {
        let mut room = Room::new(None, None, None, None);
        assert!(room.add(Pod::A, Pod::B).is_err());
    }

    #[test]
    fn dont_block_other_non_test_pod() {
        let mut room = Room::new(Some(Pod::A), Some(Pod::B), Some(Pod::A), Some(Pod::B));
        assert_eq!(room.get(Pod::C), Some((Pod::A, 1)));
    }

    #[test]
    fn move_into_empty_hw() {
        let hw = Hallway::new(
            [
                [None, None, None, None],
                [None, None, None, Some(Pod::A)],
                [None, None, None, None],
                [None, None, None, None],
            ],
            0,
        );
        // Amphipods will never stop on the space immediately outside any room.
        let children = hw.move_to_hw();
        assert_eq!(children.len(), 7);
        list_contains_n_of_cost(&children, 2, 5);
        list_contains_n_of_cost(&children, 2, 7);
        list_contains_n_of_cost(&children, 1, 9);
        list_contains_n_of_cost(&children, 1, 9);
        list_contains_n_of_cost(&children, 1, 10);
    }

    fn list_contains_n_of_cost(children: &[Hallway], expected_num: usize, expected_cost: usize) {
        assert_eq!(
            children
                .iter()
                .filter(|&h| h.energy == expected_cost)
                .count(),
            expected_num,
            "expcted {} with energy {} in:\n{:?}",
            expected_num,
            expected_cost,
            children
        )
    }

    #[test]
    fn move_into_hw_with_one_pod() {
        let hw = Hallway::new(
            [
                [None, None, None, None],
                [None, None, Some(Pod::B), Some(Pod::A)],
                [None, None, None, None],
                [None, None, None, None],
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
                [None, None, None, None],
                [None, None, Some(Pod::B), Some(Pod::A)],
                [None, None, None, None],
                [None, None, None, None],
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

    #[test]
    fn move_last_to_room() {
        let hw = Hallway {
            energy: 0,
            content: [
                Some(Pod::A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            rooms: [
                Room::new(None, Some(Pod::A), Some(Pod::A), Some(Pod::A)),
                Room::new(Some(Pod::B), Some(Pod::B), Some(Pod::B), Some(Pod::B)),
                Room::new(Some(Pod::C), Some(Pod::C), Some(Pod::C), Some(Pod::C)),
                Room::new(Some(Pod::D), Some(Pod::D), Some(Pod::D), Some(Pod::D)),
            ],
        };

        let children = hw.get_children();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].energy, 3);
        assert!(children[0].is_done());
    }

    #[test]
    fn move_last_to_room_blocked() {
        let hw = Hallway {
            energy: 0,
            content: [
                Some(Pod::D),
                None,
                None,
                Some(Pod::A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            rooms: [
                Room::new(Some(Pod::A), Some(Pod::A), Some(Pod::A), Some(Pod::A)),
                Room::new(Some(Pod::B), Some(Pod::B), Some(Pod::B), Some(Pod::B)),
                Room::new(Some(Pod::C), Some(Pod::C), Some(Pod::C), Some(Pod::C)),
                Room::new(None, None, None, Some(Pod::D)),
            ],
        };

        let children = hw.get_children();
        assert!(!hw.is_done());
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn only_return_one_child_if_pod_reaches_destination() {
        let hw = Hallway {
            energy: 0,
            content: [
                Some(Pod::A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Pod::A),
            ],
            rooms: [
                Room::new(None, None, None, None),
                Room::new(Some(Pod::B), Some(Pod::B), Some(Pod::B), Some(Pod::B)),
                Room::new(Some(Pod::D), Some(Pod::C), Some(Pod::D), Some(Pod::C)),
                Room::new(Some(Pod::C), Some(Pod::D), Some(Pod::C), Some(Pod::D)),
            ],
        };

        let children1 = hw.get_children();
        assert_eq!(children1.len(), 1);
        let children2 = children1[0].get_children();
        assert_eq!(children2.len(), 1);
    }
}
