fn fft(input: &Vec<i64>, pattern: &Vec<i64>) -> Vec<i64> {
    let mut output = input.clone();
    let pl = pattern.len();
    let il = input.len();
    for phase in 1..(input.len() + 1) {
        let mut score = 0;

        let mut i = phase - 1;
        let mut p;
        loop {
            p = pattern[((i + 1) / phase) % pl];
            if p == 0 {
                i += phase;
                p = pattern[((i + 1) / phase) % pl];
            }
            if i >= il {
                break;
            }
            let nc = phase.min(il - i);
            if p == 1 {
                for _ in 0..nc {
                    score += unsafe { input.get_unchecked(i) };
                    i += 1;
                }
            } else {
                for _ in 0..nc {
                    score -= unsafe { input.get_unchecked(i) };
                    i += 1;
                }
            }

            if i + 1 == il {
                break;
            }
        }

        output[phase - 1] = score.abs() % 10;
    }

    output
}

pub fn part_a(input: &str) -> String {
    let mut v: Vec<i64> = input
        .chars()
        .filter(|line| line != &'\n')
        .map(|line| line.to_string().parse::<i64>().expect("Invalid number"))
        .collect();

    let base = vec![0, 1, 0, -1];
    for _ in 0..100 {
        v = fft(&v, &base);
    }

    v.into_iter().map(|v| v.to_string()).collect()
}

pub fn part_b(input: &str) -> String {
    let inp: Vec<i64> = input
        .chars()
        .filter(|line| line != &'\n')
        .map(|line| line.to_string().parse::<i64>().expect("Invalid number"))
        .collect();

    let offset = inp
        .iter()
        .take(7)
        .map(|v| v.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut cur = Vec::with_capacity(inp.len() * 10000);

    for _ in 0..10000 {
        for j in 0..inp.len() {
            cur.push(inp[j]);
        }
    }

    assert!(offset >= inp.len() / 2);

    let mut next = cur.clone();

    for _ in 0..100 {
        let mut cs = 0;
        for i in 0..(cur.len() / 2) {
            cs += cur[cur.len() - 1 - i];
            next[cur.len() - 1 - i] = cs.abs() % 10;
        }
        std::mem::swap(&mut next, &mut cur);
    }

    let mut res: Vec<String> = vec![];
    for i in (offset)..(offset + 8) {
        res.push(cur[i].to_string());
    }

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("80871224585914546619083218645595\n"),
            "24176176480919046114038763195595"
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "77038830653233361255314046818347110691571207860972826750703528036072647137275835157934865244753436827100638642075752850257221737315334180111899482275873821050397765752162740703857084466158829110765095716409457347494374275616497668507627323833234935306896546517713172097671580519699518571036198691652639192629112328924296569895346103757993804590618706801045733244530365266348359432626365639551250687317888261896589020351163534902963636024105155028396916635622607564613260090550294620751980641832094142222897182373851955141758381749266214573978629986756691594740935351826234697302504217334139643483903966326398329406895250802553412058415096652193339331");
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("03036732577212944063491565474664\n"),
            "84462026"
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "28135104");
    }
}
