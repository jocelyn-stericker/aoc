use std::collections::{BTreeMap, VecDeque};

fn best<'a>(
    ingredients: &'a BTreeMap<&'a str, BTreeMap<&'a str, i64>>,
    rem: i64,
    mut rem_ingredients: VecDeque<&'a str>,
    mut values: BTreeMap<&'a str, i64>,
) -> i64 {
    if rem == 0 {
        let mut score = 1;
        for property in &["capacity", "durability", "flavor", "texture"] {
            let mut partial = 0;
            for (ingredient, qty) in &values {
                partial += qty * ingredients[ingredient][property];
            }
            score *= partial.max(0);
        }
        return score;
    }

    let mut best_so_far = 0;

    while let Some(ingredient) = rem_ingredients.front() {
        *values.entry(ingredient).or_insert(0) += 1;
        let b = best(
            ingredients,
            rem - 1,
            rem_ingredients.clone(),
            values.clone(),
        );
        best_so_far = best_so_far.max(b);
        *values.entry(ingredient).or_insert(0) -= 1;

        rem_ingredients.pop_front();
    }

    best_so_far
}

pub fn part_a(input: &str) -> i64 {
    let mut ingredients = BTreeMap::new();

    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(": ").collect();
        let ingredient = parts[0];
        let mut properties = BTreeMap::new();

        for part in parts[1].split(", ") {
            let mut parts = part.split(' ');
            let name = parts.next().unwrap();
            let val = parts.next().unwrap().parse().unwrap();

            properties.insert(name, val);
        }

        ingredients.insert(ingredient, properties);
    }

    best(
        &ingredients,
        100,
        ingredients.keys().copied().collect(),
        BTreeMap::new(),
    )
}

fn best_b<'a>(
    ingredients: &'a BTreeMap<&'a str, BTreeMap<&'a str, i64>>,
    rem: i64,
    mut rem_ingredients: VecDeque<&'a str>,
    mut values: BTreeMap<&'a str, i64>,
) -> i64 {
    if rem == 0 {
        let mut score = 1;
        let mut calories = 0;
        for (ingredient, qty) in &values {
            calories += qty * ingredients[ingredient]["calories"];
        }

        if calories != 500 {
            return 0;
        }

        for property in &["capacity", "durability", "flavor", "texture"] {
            let mut partial = 0;
            for (ingredient, qty) in &values {
                partial += qty * ingredients[ingredient][property];
            }
            score *= partial.max(0);
        }
        return score;
    }

    let mut best_so_far = 0;

    while let Some(ingredient) = rem_ingredients.front() {
        *values.entry(ingredient).or_insert(0) += 1;
        let b = best_b(
            ingredients,
            rem - 1,
            rem_ingredients.clone(),
            values.clone(),
        );
        best_so_far = best_so_far.max(b);
        *values.entry(ingredient).or_insert(0) -= 1;

        rem_ingredients.pop_front();
    }

    best_so_far
}

pub fn part_b(input: &str) -> i64 {
    let mut ingredients = BTreeMap::new();

    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(": ").collect();
        let ingredient = parts[0];
        let mut properties = BTreeMap::new();

        for part in parts[1].split(", ") {
            let mut parts = part.split(' ');
            let name = parts.next().unwrap();
            let val = parts.next().unwrap().parse().unwrap();

            properties.insert(name, val);
        }

        ingredients.insert(ingredient, properties);
    }

    best_b(
        &ingredients,
        100,
        ingredients.keys().copied().collect(),
        BTreeMap::new(),
    )
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 222870);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 117936);
    }
}
