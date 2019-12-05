pub fn get_output(mut ram: Vec<i64>) -> Result<i64, String> {
    let mut pc = 0;
    loop {
        let mod_idx;
        let mod_value;
        match ram.get(pc) {
            Some(1) => {
                let p1 = ram.get(pc + 1);
                let p2 = ram.get(pc + 2);
                let p3 = ram.get(pc + 3);
                if let (Some(p1), Some(p2), Some(p3)) = (
                    p1.and_then(|p1| ram.get(*p1 as usize)),
                    p2.and_then(|p2| ram.get(*p2 as usize)),
                    p3,
                ) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 + p2);
                } else {
                    return Err("1 read past end".to_owned());
                }
                pc += 4;
            }
            Some(2) => {
                let p1 = ram.get(pc + 1);
                let p2 = ram.get(pc + 2);
                let p3 = ram.get(pc + 3);
                if let (Some(p1), Some(p2), Some(p3)) = (
                    p1.and_then(|p1| ram.get(*p1 as usize)),
                    p2.and_then(|p2| ram.get(*p2 as usize)),
                    p3,
                ) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 * p2);
                } else {
                    return Err("2 read past end".to_owned());
                }
                pc += 4;
            }
            Some(99) => {
                break;
            }
            Some(_) => {
                return Err("Bad opcode".to_owned());
            }
            None => {
                return Err("Getting opcode read past end".to_owned());
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            let mod_idx = *mod_idx;
            if let Some(ram) = ram.get_mut(mod_idx as usize) {
                *ram = mod_value;
            } else {
                return Err("Write out of bounds".to_owned());
            }
        }
    }

    Ok(ram[0])
}

pub fn part_a(input: &str) -> Result<i64, String> {
    let mut ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    // 1202 program alarm state
    ram[1] = 12;
    ram[2] = 2;

    get_output(ram)
}

pub fn part_b(input: &str) -> Result<i64, String> {
    let mut ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    for noun in 0..99 {
        ram[1] = noun;
        for verb in 0..99 {
            ram[2] = verb;
            if get_output(ram.clone()) == Ok(19690720) {
                return Ok(noun * 100 + verb);
            }
        }
    }

    return Err("No solution".to_owned());
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("2,0,0,0,99"),
            Err("2 read past end".to_owned())
        );
        assert_eq!(super::part_a("2,0,0,0,99,0,0,0,0,0,0,0,3"), Ok(6));
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), Ok(9706670));
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), Ok(2552));
    }
}
