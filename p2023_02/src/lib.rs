use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut count = 0;
    for line in input.trim().split('\n') {
        let (game_id, game) = line.split_once(": ").unwrap();
        let (_, game_id) = game_id.split_once(" ").unwrap();
        let game_id = game_id.parse::<i64>().unwrap();
        let mut ok = true;

        'a: for game in game.split("; ") {
            for (num, color) in game.split(", ").map(|game| game.split_once(" ").unwrap()) {
                let num = num.parse::<i64>().unwrap();
                if color == "red" && num > 12
                    || color == "green" && num > 13
                    || color == "blue" && num > 14
                {
                    ok = false;
                    break 'a;
                }
            }
        }

        if ok == true {
            count += game_id;
        }
    }
    count
}

pub fn part_b(input: &str) -> i64 {
    let mut count = 0;
    for line in input.trim().split('\n') {
        let (_game_id, game) = line.split_once(": ").unwrap();

        let mut required_cubes = HashMap::new();

        for game in game.split("; ") {
            for (num, color) in game.split(", ").map(|game| game.split_once(" ").unwrap()) {
                let num = num.parse::<i64>().unwrap();
                let required_cubes = required_cubes.entry(color).or_insert(0);
                *required_cubes = (*required_cubes).max(num)
            }
        }

        count += required_cubes.get("red").unwrap_or(&0)
            * required_cubes.get("green").unwrap_or(&0)
            * required_cubes.get("blue").unwrap_or(&0);
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2348);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 76008);
    }
}
