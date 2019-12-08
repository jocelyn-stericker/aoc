use std::collections::HashMap;

pub fn get_map(input: &str) -> (HashMap<(&str, &str), usize>, HashMap<&str, Vec<&str>>) {
    let mut parents: HashMap<&str, Vec<&str>> = Default::default();
    let mut children: HashMap<&str, Vec<&str>> = Default::default();
    let mut pairs: HashMap<(&str, &str), usize> = Default::default();

    input
        .split('\n')
        .filter(|line| line != &"")
        .for_each(|line| {
            let s: Vec<&str> = line.split(')').collect();
            children.entry(s[0]).or_default().push(s[1]);
            parents.entry(s[1]).or_default().push(s[0]);
            pairs.insert((s[0], s[1]), 1);
        });

    loop {
        let mut changed = false;
        for (mid, par1) in parents.clone() {
            for parent in par1 {
                if let Some(children_of_mid) = children.get(mid) {
                    for child in children_of_mid.clone() {
                        if pairs.contains_key(&(parent, child)) {
                            continue;
                        }
                        changed = true;
                        children.entry(parent).or_default().push(child);
                        parents.entry(child).or_default().push(parent);
                        pairs.insert(
                            (parent, child),
                            pairs[&(parent, mid)] + pairs[&(mid, child)],
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    (pairs, parents)
}

pub fn part_a(input: &str) -> usize {
    get_map(input).0.len()
}

pub fn part_b(input: &str) -> usize {
    let (pairs, parents) = get_map(input);
    for common in &parents["SAN"] {
        if pairs.contains_key(&(common, "YOU")) {
            eprintln!("{} {}", pairs[&(*common, "SAN")], pairs[&(*common, "YOU")]);
            return (pairs[&(*common, "SAN")] + pairs[&(*common, "YOU")] - 2).max(0);
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
            42
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 140608);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
            4
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 337);
    }
}
