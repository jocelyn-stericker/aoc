#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

pub fn solve(input: &str, rows: usize) -> usize {
    let mut row = Vec::new();
    for c in input.trim().chars() {
        match c {
            '.' => {
                row.push(Tile::Safe);
            }
            '^' => {
                row.push(Tile::Trap);
            }
            _ => {
                panic!()
            } //
        }
    }

    let mut count = row.iter().filter(|t| t == &&Tile::Safe).count();

    for _ in 1..rows {
        let mut next_row = Vec::new();
        for i in 0..row.len() {
            let left = row.get(i - 1).unwrap_or(&Tile::Safe);
            let center = row.get(i).unwrap_or(&Tile::Safe);
            let right = row.get(i + 1).unwrap_or(&Tile::Safe);
            next_row.push(match (left, center, right) {
                (&Tile::Trap, &Tile::Trap, &Tile::Safe)
                | (&Tile::Safe, &Tile::Trap, &Tile::Trap)
                | (&Tile::Trap, &Tile::Safe, &Tile::Safe)
                | (&Tile::Safe, &Tile::Safe, &Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            });
        }

        row = next_row;

        count += row.iter().filter(|t| t == &&Tile::Safe).count();
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::solve(".^^.^.^^^^\n", 10), 38);
    }

    #[test]
    fn part_a() {
        // 1660 too low
        assert_eq!(super::solve(include_str!("input.txt"), 40), 1989);
    }

    #[test]
    fn part_b() {
        // 1660 too low
        assert_eq!(super::solve(include_str!("input.txt"), 400000), 19999894);
    }
}
