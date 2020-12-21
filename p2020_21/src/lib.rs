use std::collections::{HashMap, HashSet};

pub fn part_a(input: &str) -> usize {
    let mut maybe = HashSet::new();
    let mut all_ingredients = HashSet::new();
    let mut all_alergens = HashSet::new();
    let mut rules = Vec::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(" (contains ");
        let ingredients: Vec<_> = parts.next().unwrap().split(' ').collect();
        let alergens: Vec<_> = parts
            .next()
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split(", ")
            .collect();
        for ing in &ingredients {
            all_ingredients.insert(*ing);
        }
        for al in &alergens {
            all_alergens.insert(*al);
        }
        for ing in &ingredients {
            for al in &alergens {
                maybe.insert((*ing, *al));
            }
        }
        rules.push((ingredients, alergens));
    }
    for (ingredients, alergens) in &rules {
        maybe = maybe
            .into_iter()
            .filter(|(ing, al)| !alergens.contains(al) || ingredients.contains(ing))
            .collect();
    }
    let maybe: HashSet<_> = maybe.iter().map(|(ing, _)| *ing).collect();
    let not: HashSet<_> = all_ingredients.difference(&maybe).cloned().collect();

    let mut sum = 0;
    for (ingredients, _alergens) in &rules {
        sum += ingredients.iter().filter(|ing| not.contains(*ing)).count()
    }

    sum
}

pub fn part_b(input: &str) -> String {
    let mut maybe = HashSet::new();
    let mut all_ingredients = HashSet::new();
    let mut all_alergens = HashSet::new();
    let mut rules = Vec::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(" (contains ");
        let ingredients: Vec<_> = parts.next().unwrap().split(' ').collect();
        let alergens: Vec<_> = parts
            .next()
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split(", ")
            .collect();
        for ing in &ingredients {
            all_ingredients.insert(*ing);
        }
        for al in &alergens {
            all_alergens.insert(*al);
        }
        for ing in &ingredients {
            for al in &alergens {
                maybe.insert((*ing, *al));
            }
        }
        rules.push((ingredients, alergens));
    }
    for (ingredients, alergens) in &rules {
        maybe = maybe
            .into_iter()
            .filter(|(ing, al)| !alergens.contains(al) || ingredients.contains(ing))
            .collect();
    }

    loop {
        let all_maybe = maybe.clone();
        let def: HashMap<&str, &str> = maybe
            .iter()
            .filter(|(ing, al)| {
                let maybe: HashSet<_> = all_maybe
                    .iter()
                    .filter_map(|(ing2, al2)| {
                        if ing2 == ing && al2 == al {
                            None
                        } else {
                            Some(al2)
                        }
                    })
                    .collect();
                maybe.len() < all_alergens.len()
            })
            .cloned()
            .collect();
        let old_len = maybe.len();
        maybe = maybe
            .into_iter()
            .filter(|(ing, al)| def.get(ing).is_none() || def.get(ing).unwrap() == al)
            .collect();
        if old_len == maybe.len() {
            break;
        }
    }
    let mut maybe: Vec<_> = maybe.into_iter().collect();
    maybe.sort_unstable_by_key(|a| a.1);
    let maybe: Vec<_> = maybe.into_iter().map(|m| m.0).collect();
    maybe.join(",")
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)\n"), 5);
    }
    #[test]
    fn example2() {
        assert_eq!(super::part_b("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)\n"), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2461);
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("input.txt")),
            "ltbj,nrfmm,pvhcsn,jxbnb,chpdjkf,jtqt,zzkq,jqnhd"
        );
    }
}
