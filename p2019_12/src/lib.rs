use num::Integer;
use std::collections::HashSet;

fn gravity(p: &Vec<[i64; 3]>, g: &mut Vec<[i64; 3]>) {
    for i in 0..p.len() {
        for j in 0..p.len() {
            for d in 0..3 {
                if p[i][d] > p[j][d] {
                    g[i][d] -= 1;
                } else if p[i][d] < p[j][d] {
                    g[i][d] += 1;
                }
            }
        }
    }
}

fn vel(p: &mut Vec<[i64; 3]>, g: &Vec<[i64; 3]>) {
    for i in 0..p.len() {
        for d in 0..3 {
            p[i][d] += g[i][d];
        }
    }
}

fn energy(p: &Vec<[i64; 3]>, g: &Vec<[i64; 3]>) -> i64 {
    let mut e = 0;
    for i in 0..p.len() {
        let mut pot = 0;
        let mut kin = 0;
        for d in 0..3 {
            pot += p[i][d].abs();
            kin += g[i][d].abs();
        }

        e += pot * kin;
    }

    e
}

pub fn part_a(input: &str) -> i64 {
    let mut p: Vec<[i64; 3]> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let mut it = line
                .split(',')
                .map(|line| line.parse::<i64>().expect("Invalid number"));
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        })
        .collect();

    let mut g: Vec<[i64; 3]> = p.iter().map(|_| [0, 0, 0]).collect();

    for _ in 0..1000 {
        gravity(&p, &mut g);
        vel(&mut p, &g);
    }

    energy(&p, &g)
}

pub fn info(x: &Vec<[i64; 3]>, y: &Vec<[i64; 3]>, d: usize) -> [i64; 8] {
    [
        x[0][d], y[0][d], x[1][d], y[1][d], x[2][d], y[2][d], x[3][d], y[3][d],
    ]
}

pub fn part_b(input: &str) -> i64 {
    let mut p: Vec<[i64; 3]> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let mut it = line
                .split(',')
                .map(|line| line.parse::<i64>().expect("Invalid number"));
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        })
        .collect();

    let mut g: Vec<[i64; 3]> = p.iter().map(|_| [0, 0, 0]).collect();

    let mut prod: i64 = 1;
    let mut hs: HashSet<[i64; 8]> = Default::default();
    for d in 0..3 {
        for i in 0.. {
            gravity(&p, &mut g);
            vel(&mut p, &g);

            let st = info(&p, &g, d);
            if hs.contains(&st) {
                prod = prod.lcm(&i);
                break;
            }
            hs.insert(st);
        }
    }

    prod
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6735);
    }

    #[test]
    fn example_b() {
        assert_eq!(
            super::part_b("-8,-10,0\n5,5,10\n2,-7,3\n9,-8,-3\n"),
            4686774924
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 326489627728984);
    }
}
