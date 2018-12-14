pub fn part_a(input: usize) -> Vec<usize> {
    let mut state = vec![3, 7];
    let mut elves = (0, 1);
    while state.len() < input + 10 {
        let n = state[elves.0] + state[elves.1];
        if n >= 10 {
            state.push(n / 10);
        }
        state.push(n % 10);
        elves = (elves.0 + 1 + state[elves.0], elves.1 + 1 + state[elves.1]);
        elves = (elves.0 % state.len(), elves.1 % state.len());
        // println!("{:?}", state);
    }

    state[input..input + 10].iter().map(|s| *s).collect()
}

pub fn part_b(input: Vec<usize>) -> usize {
    let mut state = vec![3, 7];
    let mut elves = (0, 1);
    loop {
        let n = state[elves.0] + state[elves.1];
        if n >= 10 {
            state.push(n / 10);
            if state.len() >= input.len()
                && state[(state.len() - input.len())..state.len()]
                    .iter()
                    .map(|v| *v)
                    .collect::<Vec<usize>>()
                    == input
            {
                return state.len() - input.len();
            }
        }
        state.push(n % 10);
        if state.len() >= input.len()
            && state[(state.len() - input.len())..state.len()]
                .iter()
                .map(|v| *v)
                .collect::<Vec<usize>>()
                == input
        {
            return state.len() - input.len();
        }
        elves = (elves.0 + 1 + state[elves.0], elves.1 + 1 + state[elves.1]);
        elves = (elves.0 % state.len(), elves.1 % state.len());
    }
}

#[test]
fn test_sample_1() {
    assert_eq!(part_a(9), vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
    assert_eq!(part_a(5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
    assert_eq!(part_a(18), vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
    assert_eq!(part_a(2018), vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(047801), vec![1, 3, 4, 2, 3, 1, 6, 4, 1, 0]);
}

#[test]
fn test_sample_2() {
    assert_eq!(part_b(vec![5, 1, 5, 8, 9]), 9);
    assert_eq!(part_b(vec![0, 1, 2, 4, 5]), 5);
    assert_eq!(part_b(vec![9, 2, 5, 1, 0]), 18);
    assert_eq!(part_b(vec![5, 9, 4, 1, 4]), 2018);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(vec![0, 4, 7, 8, 0, 1]), 20235230);
}
