use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Node {
    used: i64,
    avail: i64,
    x: i64,
    y: i64,
    has_goal: bool,
}

impl Node {
    fn fits(&self, other: &Node) -> bool {
        other.used != 0 && other.used <= self.avail
    }
}

pub fn part_a(input: &str) -> i64 {
    let line_re = Regex::new(r"/dev/grid/node-x(?P<x>\d*)-y(?P<y>\d*)\s*(?P<size>[^ ]*)T\s*(?P<used>[^ ]*)T\s*(?P<avail>[^ ]*)T\s*(?P<usep>[^ ]*)%").unwrap();
    let mut nodes = Vec::new();

    for line in input.trim().split('\n').skip(2) {
        let caps = line_re.captures(line).unwrap();
        let used: i64 = caps["used"].parse().unwrap();
        let avail: i64 = caps["avail"].parse().unwrap();
        let x: i64 = caps["x"].parse().unwrap();
        let y: i64 = caps["y"].parse().unwrap();
        nodes.push(Node {
            x,
            y,
            used,
            avail,
            has_goal: false,
        });
    }

    let mut viable = 0;
    for a in nodes.iter() {
        for b in nodes.iter() {
            if (a.x != b.x || a.y != b.y) && b.fits(a) {
                viable += 1;
            }
        }
    }
    viable
}

pub fn part_b(input: &str) -> i64 {
    let line_re = Regex::new(r"/dev/grid/node-x(?P<x>\d*)-y(?P<y>\d*)\s*(?P<size>[^ ]*)T\s*(?P<used>[^ ]*)T\s*(?P<avail>[^ ]*)T\s*(?P<usep>[^ ]*)%").unwrap();
    let mut visited = BTreeSet::new();
    let mut nodes = BTreeMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.trim().split('\n').skip(2) {
        let caps = line_re.captures(line).unwrap();
        let used: i64 = caps["used"].parse().unwrap();
        let avail: i64 = caps["avail"].parse().unwrap();
        let x: i64 = caps["x"].parse().unwrap();
        let y: i64 = caps["y"].parse().unwrap();
        nodes.insert(
            (x, y),
            Node {
                x,
                y,
                used,
                avail,
                has_goal: false,
            },
        );
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    nodes.get_mut(&(max_x, 0)).unwrap().has_goal = true;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), nodes));

    while let Some((Reverse(turns), nodes)) = queue.pop() {
        if nodes[&(0, 0)].has_goal {
            return turns;
        }
        let mut c = 0;
        for x in 0..=max_x {
            for y in 0..=max_y {
                let me = nodes[&(x, y)];
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if let Some(other) = nodes.get(&(x + dx, y + dy)) {
                        if (other.used == 0 || me.has_goal) && other.fits(&me) {
                            c += 1;
                            let mut next_states = nodes.clone();
                            let mut next_other = next_states.get_mut(&(x + dx, y + dy)).unwrap();

                            next_other.used += me.used;
                            next_other.avail -= me.used;
                            assert!(next_other.avail >= 0);

                            next_other.has_goal = other.has_goal || me.has_goal;

                            let mut next_me = next_states.get_mut(&(x, y)).unwrap();
                            next_me.used = 0;
                            next_me.avail += me.used;
                            next_me.has_goal = false;

                            let o = next_states.values().find(|f| f.has_goal).unwrap();
                            let p = next_states.values().find(|f| f.used == 0).unwrap();

                            if !visited.contains(&(o.x, o.y, p.x, p.y)) {
                                visited.insert((o.x, o.y, p.x, p.y));
                                queue.push((Reverse(turns + 1), next_states));
                            }
                        }
                    }
                }
            }
        }
        assert!(c <= 4);
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 901);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 238);
    }
}
