use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');
    let mut p1: i64 = lines
        .next()
        .unwrap()
        .trim_start_matches("Player 1 starting position: ")
        .parse()
        .unwrap();
    let mut p2: i64 = lines
        .next()
        .unwrap()
        .trim_start_matches("Player 2 starting position: ")
        .parse()
        .unwrap();

    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice_state = 1;
    let mut dice_roles = 0;
    loop {
        for _ in 0..3 {
            p1 += dice_state;
            dice_state += 1;
            dice_state = (dice_state - 1) % 100 + 1;
            dice_roles += 1;
        }
        p1 = (p1 - 1) % 10 + 1;
        p1_score += p1;

        if p1_score >= 1000 {
            return p2_score * dice_roles;
        }

        for _ in 0..3 {
            p2 += dice_state;
            dice_state += 1;
            dice_state = (dice_state - 1) % 100 + 1;
            dice_roles += 1;
        }
        p2 = (p2 - 1) % 10 + 1;
        p2_score += p2;

        if p2_score >= 1000 {
            return p1_score * dice_roles;
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    is_p1_turn: bool,
    p1_state: i64,
    p1_score: i64,
    p2_state: i64,
    p2_score: i64,
}

impl State {
    fn p1_wins(&self) -> bool {
        self.p1_score >= 21
    }

    fn p2_wins(&self) -> bool {
        self.p2_score >= 21
    }

    fn p1_roll(&self) -> Vec<State> {
        let mut new_states = Vec::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let p1_state = (self.p1_state + i + j + k - 1) % 10 + 1;
                    let p1_score = self.p1_score + p1_state;
                    new_states.push(State {
                        p1_state,
                        p1_score,
                        p2_state: self.p2_state,
                        p2_score: self.p2_score,
                        is_p1_turn: false,
                    });
                }
            }
        }
        new_states
    }

    fn p2_roll(&self) -> Vec<State> {
        let mut new_states = Vec::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let p2_state = (self.p2_state + i + j + k - 1) % 10 + 1;
                    let p2_score = self.p2_score + p2_state;
                    new_states.push(State {
                        p2_state,
                        p2_score,
                        p1_state: self.p1_state,
                        p1_score: self.p1_score,
                        is_p1_turn: true,
                    });
                }
            }
        }
        new_states
    }

    fn roll(&self) -> Vec<State> {
        if self.is_p1_turn {
            self.p1_roll()
        } else {
            self.p2_roll()
        }
    }
}

fn play(memo: &mut HashMap<State, (usize, usize)>, state: State) -> (usize, usize) {
    if let Some((p1_wins, p2_wins)) = memo.get(&state) {
        return (*p1_wins, *p2_wins);
    }
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for case in state.roll() {
        if case.p1_wins() {
            p1_wins += 1;
        } else if case.p2_wins() {
            p2_wins += 1;
        } else {
            let (sub_p1, sub_p2) = play(memo, case);
            p1_wins += sub_p1;
            p2_wins += sub_p2;
        }
    }

    memo.insert(state.clone(), (p1_wins, p2_wins));
    return (p1_wins, p2_wins);
}

pub fn part_b(input: &str) -> usize {
    let mut lines = input.trim().split('\n');
    let p1: i64 = lines
        .next()
        .unwrap()
        .trim_start_matches("Player 1 starting position: ")
        .parse()
        .unwrap();
    let p2: i64 = lines
        .next()
        .unwrap()
        .trim_start_matches("Player 2 starting position: ")
        .parse()
        .unwrap();

    let state = State {
        p1_state: p1,
        p1_score: 0,
        p2_state: p2,
        p2_score: 0,
        is_p1_turn: true,
    };

    let mut memo: HashMap<State, (usize, usize)> = HashMap::new();
    let (p1_wins, p2_wins) = play(&mut memo, state);

    p1_wins.max(p2_wins)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "Player 1 starting position: 4
Player 2 starting position: 8\n"
            ),
            739785
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "Player 1 starting position: 4
Player 2 starting position: 8\n"
            ),
            444356092776315
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 306621346123766);
    }
}
