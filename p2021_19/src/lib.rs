use std::collections::{HashMap, HashSet};

fn roll(pt: (i64, i64, i64)) -> (i64, i64, i64) {
    (pt.0, pt.2, -pt.1)
}

fn turn(pt: (i64, i64, i64)) -> (i64, i64, i64) {
    (-pt.1, pt.0, pt.2)
}

fn rotations(mut pt: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut pts = Vec::new();

    for _cycle in 0..2 {
        for _step in 0..3 {
            pt = roll(pt);
            pts.push(pt);
            for _i in 0..3 {
                pt = turn(pt);
                pts.push(pt);
            }
        }
        pt = roll(turn(pt));
    }

    assert!(pts.len() == 24);
    let a = pts[0];
    let b = pts[11];
    pts[11] = a;
    pts[0] = b;
    pts
}

pub fn solve(input: &str) -> (usize, i64) {
    let mut lines = input.trim().split('\n');
    let mut scanners = Vec::new();

    loop {
        let mut beacons = Vec::new();
        for _ in 0..24 {
            beacons.push(Vec::new());
        }
        if lines.next().is_none() {
            break;
        }
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut xyz = line.split(',');
            let x = xyz.next().unwrap().parse::<i64>().unwrap();
            let y = xyz.next().unwrap().parse::<i64>().unwrap();
            let z = xyz.next().unwrap().parse::<i64>().unwrap();
            for (i, pt) in rotations((x, y, z)).into_iter().enumerate() {
                beacons[i].push(pt);
            }
        }
        scanners.push(beacons);
    }

    let mut systems: Vec<HashSet<usize>> = Vec::new();
    let mut moved: HashMap<usize, (i64, i64, i64)> = HashMap::new();

    loop {
        let mut aligned = Vec::new();
        let mut next_realignment = None;
        'outer: for (i, scanner) in scanners.iter().enumerate() {
            for (j, other_scanner) in scanners.iter().enumerate().skip(i + 1) {
                let other_beacons = &other_scanner[0];

                for (k, beacons) in scanner.iter().enumerate() {
                    let mut potential_offsets: HashMap<(i64, i64, i64), usize> = HashMap::new();
                    for beacon in beacons {
                        for other_beacon in other_beacons {
                            let offset = (
                                beacon.0 - other_beacon.0,
                                beacon.1 - other_beacon.1,
                                beacon.2 - other_beacon.2,
                            );
                            *potential_offsets.entry(offset).or_default() += 1;
                        }
                    }
                    let mut potential_offsets: Vec<_> = potential_offsets
                        .into_iter()
                        .filter(|entry| entry.1 >= 12)
                        .map(|(offset, _)| offset)
                        .collect();
                    assert!(potential_offsets.is_empty() || potential_offsets.len() == 1);
                    if let Some(offset) = potential_offsets.pop() {
                        aligned.push((i, j, offset));
                        if k == 0 && offset == (0, 0, 0) {
                            break;
                        }
                        if k != 0 || offset != (0, 0, 0) {
                            next_realignment = Some((i, j, k, offset));
                            break 'outer;
                        }
                    }
                }
            }
        }

        if let Some((i, j, rot_i, offset_i)) = next_realignment {
            let system_i = systems
                .iter()
                .enumerate()
                .find(|items| items.1.contains(&i))
                .map(|item| item.0);
            let system_j = systems
                .iter()
                .enumerate()
                .find(|items| items.1.contains(&j))
                .map(|item| item.0);

            match (system_i, system_j) {
                (None, None) => {
                    let mut pts = scanners[i][rot_i].clone();
                    for pt in &mut pts {
                        pt.0 -= offset_i.0;
                        pt.1 -= offset_i.1;
                        pt.2 -= offset_i.2;
                    }
                    let e = moved.entry(i).or_default();
                    *e = rotations(*e)[rot_i];
                    e.0 -= offset_i.0;
                    e.1 -= offset_i.1;
                    e.2 -= offset_i.2;
                    let mut beacons = Vec::new();
                    for _ in 0..24 {
                        beacons.push(Vec::new());
                    }
                    for pt in pts {
                        for (i, pt) in rotations(pt).into_iter().enumerate() {
                            beacons[i].push(pt);
                        }
                    }
                    scanners[i] = beacons;
                    let mut set = HashSet::new();
                    set.insert(i);
                    set.insert(j);
                    systems.push(set);
                    eprintln!("Rotate {} by {} to match {}", i, rot_i, j);
                }
                (Some(system_i), None) => {
                    eprintln!("{:?} {} {} {}", system_i, i, j, rot_i);
                    for i in &systems[system_i] {
                        let mut pts = scanners[*i][rot_i].clone();
                        for pt in &mut pts {
                            pt.0 -= offset_i.0;
                            pt.1 -= offset_i.1;
                            pt.2 -= offset_i.2;
                        }
                        let e = moved.entry(*i).or_default();
                        *e = rotations(*e)[rot_i];
                        e.0 -= offset_i.0;
                        e.1 -= offset_i.1;
                        e.2 -= offset_i.2;
                        let mut beacons = Vec::new();
                        for _ in 0..24 {
                            beacons.push(Vec::new());
                        }
                        for pt in pts {
                            for (i, pt) in rotations(pt).into_iter().enumerate() {
                                beacons[i].push(pt);
                            }
                        }
                        scanners[*i] = beacons;
                    }
                    systems[system_i].insert(j);
                }
                (Some(system_i), Some(system_j)) => {
                    eprintln!("{:?} {} {} {}", system_i, i, j, rot_i);
                    for i in &systems[system_i] {
                        let mut pts = scanners[*i][rot_i].clone();
                        for pt in &mut pts {
                            pt.0 -= offset_i.0;
                            pt.1 -= offset_i.1;
                            pt.2 -= offset_i.2;
                        }
                        let e = moved.entry(*i).or_default();
                        *e = rotations(*e)[rot_i];
                        e.0 -= offset_i.0;
                        e.1 -= offset_i.1;
                        e.2 -= offset_i.2;
                        let mut beacons = Vec::new();
                        for _ in 0..24 {
                            beacons.push(Vec::new());
                        }
                        for pt in pts {
                            for (i, pt) in rotations(pt).into_iter().enumerate() {
                                beacons[i].push(pt);
                            }
                        }
                        scanners[*i] = beacons;
                    }
                    assert!(system_i != system_j);
                    let system_min = system_i.min(system_j);
                    let system_max = system_i.max(system_j);
                    for item in systems.remove(system_max) {
                        systems[system_min].insert(item);
                    }
                }
                (None, Some(system_j)) => {
                    let mut pts = scanners[i][rot_i].clone();
                    for pt in &mut pts {
                        pt.0 -= offset_i.0;
                        pt.1 -= offset_i.1;
                        pt.2 -= offset_i.2;
                    }
                    let e = moved.entry(i).or_default();
                    *e = rotations(*e)[rot_i];
                    e.0 -= offset_i.0;
                    e.1 -= offset_i.1;
                    e.2 -= offset_i.2;
                    let mut beacons = Vec::new();
                    for _ in 0..24 {
                        beacons.push(Vec::new());
                    }
                    for pt in pts {
                        for (i, pt) in rotations(pt).into_iter().enumerate() {
                            beacons[i].push(pt);
                        }
                    }
                    scanners[i] = beacons;

                    systems[system_j].insert(i);
                }
            }
        } else {
            eprintln!("{:#?}", aligned);
            let scanners: Vec<_> = scanners
                .into_iter()
                .map(|rotations| rotations[0].clone())
                .collect();
            let scanner_count = scanners.len();
            let mut points = HashSet::new();
            for scanner in scanners {
                for pt in scanner {
                    points.insert(pt);
                }
            }

            let mut max_dis = 0;
            for i in 0..scanner_count {
                for j in 0..scanner_count {
                    let p1 = moved.get(&i).copied().unwrap_or_default();
                    let p2 = moved.get(&j).copied().unwrap_or_default();
                    max_dis = max_dis
                        .max((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs());
                }
            }
            eprintln!(
                "{:#?} {:#?} {:#?}",
                moved.get(&0),
                moved.get(&2),
                moved.get(&3)
            );
            return (points.len(), max_dis);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rotations1() {
        eprintln!("{:?}", super::rotations((-1, -1, 1)));
    }

    #[test]
    fn example1() {
        assert_eq!(super::solve(include_str!("example_1.txt")), (79, 3621));
    }

    #[test]
    fn part_a() {
        // 27641 is too high
        assert_eq!(super::solve(include_str!("input.txt")), (457, 13243));
    }
}
