pub fn get_layers(input: &str) -> Vec<Vec<i64>> {
    let width = 25;
    let height = 6;
    let data: Vec<i64> = input
        .trim()
        .chars()
        .map(|line| line.to_string().parse::<i64>().expect("Invalid number"))
        .collect();

    let mut layers: Vec<Vec<i64>> = vec![];
    let mut h = 0;
    while h < data.len() {
        layers.push(vec![]);
        for _i in 0..width {
            for _j in 0..height {
                layers.last_mut().unwrap().push(data[h]);
                h += 1;
            }
        }
    }

    layers
}

pub fn part_a(input: &str) -> i64 {
    let layers = get_layers(input);

    let l = layers
        .iter()
        .map(|l| {
            l.iter()
                .fold(0, |sum, i| if *i == 0 { sum + 1 } else { sum })
        })
        .enumerate()
        .min_by_key(|&(_, i)| i)
        .unwrap()
        .0;

    let ones = layers[l]
        .iter()
        .fold(0, |sum, i| if *i == 1 { sum + 1 } else { sum });
    let twos = layers[l]
        .iter()
        .fold(0, |sum, i| if *i == 2 { sum + 1 } else { sum });

    ones * twos
}

pub fn part_b(input: &str) -> String {
    let layers = get_layers(input);
    let mut image: Vec<i64> = (0..25 * 6).map(|_| 2).collect();

    for l in layers.iter().rev() {
        for (i, pixel) in l.iter().enumerate() {
            image[i] = match pixel {
                2 => image[i],
                _ => *pixel,
            }
        }
    }

    let mut msg = Vec::with_capacity(26 * 6);
    for i in 0..6 {
        for j in 0..25 {
            match image[i * 25 + j] {
                0 => msg.push(' '),
                _ => msg.push('#'),
            }
        }
        msg.push('\n')
    }

    msg.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_ne!(super::part_a(include_str!("input.txt")), 2288);
        assert_eq!(super::part_a(include_str!("input.txt")), 1206);
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("input.txt")).trim(),
            include_str!("answer.txt").trim()
        );
    }
}
