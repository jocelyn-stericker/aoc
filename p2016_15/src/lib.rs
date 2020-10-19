// use std::collections::HashSet;
//
#[derive(Copy, Clone)]
struct Disk {
    positions: i64,
    position: i64,
}

impl Disk {
    fn next(&mut self) {
        self.position = (self.position + 1) % self.positions;
    }
}

fn works(mut disks: Vec<Disk>) -> bool {
    for i in 0..disks.len() {
        for disk in disks.iter_mut() {
            disk.next();
        }
        if disks[i].position != 0 {
            return false;
        }
    }

    true
}

pub fn part_a(input: &str) -> i64 {
    let mut disks = Vec::new();
    for line in input.trim().split('\n') {
        let c: Vec<i64> = line.trim().split(' ').map(|c| c.parse().unwrap()).collect();
        disks.push(Disk {
            positions: c[0],
            position: c[1],
        });
    }

    for t in 0.. {
        if works(disks.clone()) {
            return t;
        }

        for disk in disks.iter_mut() {
            disk.next();
        }
    }
    unreachable!();
}
pub fn part_b(input: &str) -> i64 {
    let mut disks = Vec::new();
    for line in input.trim().split('\n') {
        let c: Vec<i64> = line.trim().split(' ').map(|c| c.parse().unwrap()).collect();
        disks.push(Disk {
            positions: c[0],
            position: c[1],
        });
    }
    disks.push(Disk {
        positions: 11,
        position: 0,
    });

    for t in 0.. {
        if works(disks.clone()) {
            return t;
        }

        for disk in disks.iter_mut() {
            disk.next();
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("5 4\n2 1\n"), 5);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 122318); // 8:00
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 3208583); // 9:32
    }
}
