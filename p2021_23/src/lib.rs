use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    hallway: [Option<char>; 11],
    /// Only the first 2 are used for part 1.
    rooms: [[Option<char>; 4]; 4],
}

fn entry_at_hallway_x(i: usize) -> Option<char> {
    match i {
        2 => Some('A'),
        4 => Some('B'),
        6 => Some('C'),
        8 => Some('D'),
        _ => None,
    }
}

fn hallway_x_entry_for_room_idx(i: usize) -> usize {
    match i {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!(),
    }
}

fn room_for_pod(c: char) -> usize {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!(),
    }
}

fn move_cost_for(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

impl State {
    /// Given a tile in room room_idx at idx y, calculate the cost to move to the hallway at
    /// position hallway_x
    fn room_to_hallway(&self, room_idx: usize, y: usize, hallway_x: usize) -> Option<usize> {
        if entry_at_hallway_x(hallway_x).is_some() {
            // We can't stop at at a hallway entry.
            None
        } else if let Some(pod) = self.rooms[room_idx][y] {
            let mut moves = 1;
            for j in 1..=y {
                if self.rooms[room_idx][j - 1].is_some() {
                    return None;
                }
                moves += 1;
            }

            let hallway_x_start = hallway_x_entry_for_room_idx(room_idx);
            for x in hallway_x_start.min(hallway_x)..=hallway_x_start.max(hallway_x) {
                if self.hallway[x].is_some() {
                    return None;
                }
            }

            moves += hallway_x_start.max(hallway_x) - hallway_x_start.min(hallway_x);

            Some(moves * move_cost_for(pod))
        } else {
            None
        }
    }

    /// Bottom is 2 for part 1, 4 for part 2.
    fn hallway_to_dest(&self, i: usize, bottom: usize) -> Option<(usize, usize, usize)> {
        if let Some(pod) = self.hallway[i] {
            let dest = room_for_pod(pod);
            let j = hallway_x_entry_for_room_idx(dest);

            for x in i.min(j)..=i.max(j) {
                if self.hallway[x].is_some() && x != i {
                    return None;
                }
            }

            let moves = i.max(j) - i.min(j);
            for y in 0..bottom {
                if self.rooms[dest][y].is_some() && self.rooms[dest][y] != Some(pod) {
                    return None;
                }
            }
            'heights: for y in (0..bottom).rev() {
                for y in 0..=y {
                    if self.rooms[dest][y].is_some() {
                        continue 'heights;
                    }
                }

                return Some(((moves + y + 1) * move_cost_for(pod), dest, y));
            }

            None
        } else {
            None
        }
    }

    /// Return every state that that is one move away from the current state, along with the cost
    /// of that move.
    fn moves(&self, bottom: usize) -> Vec<(State, usize)> {
        let mut moves = Vec::new();
        for i in 0..11 {
            if let Some((cost, room, y)) = self.hallway_to_dest(i, bottom) {
                let mut n = *self;
                std::mem::swap(&mut n.hallway[i], &mut n.rooms[room][y]);
                moves.push((n, cost));
            }
        }
        for room_idx in 0..4 {
            for y in 0..4 {
                for hallway in 0..11 {
                    if let Some(cost) = self.room_to_hallway(room_idx, y, hallway) {
                        let mut n = *self;
                        std::mem::swap(&mut n.hallway[hallway], &mut n.rooms[room_idx][y]);
                        moves.push((n, cost));
                    }
                }
            }
        }
        moves
    }

    /// Heuristic cost to move a tile from a room to its destination.
    ///
    /// Pretends everything else is in place.
    fn room_to_dest_h(&self, room_idx: usize, y: usize) -> usize {
        if let Some(pod) = self.rooms[room_idx][y] {
            let dest = room_for_pod(pod);
            if room_idx == dest {
                return 0;
            }

            let pos_start = hallway_x_entry_for_room_idx(room_idx);
            let pos_end = hallway_x_entry_for_room_idx(dest);

            (pos_start.max(pos_end) - pos_start.min(pos_end)) * move_cost_for(pod)
        } else {
            0
        }
    }

    /// Heuristic cost to move the tile in a hallway to its destination.
    ///
    /// Pretends everything else is in place.
    fn hallway_to_dest_h(&self, pos: usize) -> usize {
        if let Some(pod) = self.hallway[pos] {
            let dest = room_for_pod(pod);
            let pos_end = hallway_x_entry_for_room_idx(dest);

            (pos.max(pos_end) - pos.min(pos_end)) * move_cost_for(pod)
        } else {
            0
        }
    }

    fn hcost(&self) -> usize {
        let mut hcost = 0;
        for i in 0..11 {
            hcost += self.hallway_to_dest_h(i);
        }
        for room in 0..4 {
            for y in 0..4 {
                hcost += self.room_to_dest_h(room, y);
            }
        }
        hcost
    }
}

pub fn solve(input: &str, part_b: bool) -> usize {
    let mut lines = input.trim().split('\n').skip(2);
    let room_line1: Vec<char> = lines.next().unwrap().chars().collect();
    let room_line2: Vec<char> = lines.next().unwrap().chars().collect();

    let state = if part_b {
        State {
            hallway: [None; 11],
            rooms: [
                [
                    Some(room_line1[3]),
                    Some('D'),
                    Some('D'),
                    Some(room_line2[3]),
                ],
                [
                    Some(room_line1[5]),
                    Some('C'),
                    Some('B'),
                    Some(room_line2[5]),
                ],
                [
                    Some(room_line1[7]),
                    Some('B'),
                    Some('A'),
                    Some(room_line2[7]),
                ],
                [
                    Some(room_line1[9]),
                    Some('A'),
                    Some('C'),
                    Some(room_line2[9]),
                ],
            ],
        }
    } else {
        State {
            hallway: [None; 11],
            rooms: [
                [Some(room_line1[3]), Some(room_line2[3]), None, None],
                [Some(room_line1[5]), Some(room_line2[5]), None, None],
                [Some(room_line1[7]), Some(room_line2[7]), None, None],
                [Some(room_line1[9]), Some(room_line2[9]), None, None],
            ],
        }
    };

    // eprintln!("{:?} {}", state, state.hcost());
    let mut best = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), Reverse(0), state));
    while let Some((_, Reverse(cost), state)) = queue.pop() {
        if cost >= *best.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }
        if state.hcost() == 0 {
            return cost;
        }
        best.insert(state, cost);
        for (next, move_cost) in state.moves(if part_b { 4 } else { 2 }) {
            let next_cost = cost + move_cost;
            if next_cost < *best.get(&next).unwrap_or(&usize::MAX) {
                let hcost = next_cost + next.hcost();
                queue.push((Reverse(hcost), Reverse(next_cost), next));
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve(
                "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
\n",
                false
            ),
            12521
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), false), 11536);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::solve(
                "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
\n",
                true
            ),
            44169
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), true), 55136);
    }
}
