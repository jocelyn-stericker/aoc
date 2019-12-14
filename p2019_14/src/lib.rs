use std::collections::HashMap;

pub fn parse_item(item: &str) -> (String, usize) {
    let p: Vec<_> = item.split(" ").collect();
    (p[1].to_owned(), p[0].parse().unwrap())
}

pub fn cook(
    kind: &str,
    quantity: usize,
    recipies: &HashMap<String, (usize, HashMap<String, usize>)>,
    pantry: &mut HashMap<String, usize>,
) -> usize {
    if kind == "ORE" {
        return quantity;
    }

    // Insight: there is only ever one receipe for each output kind.
    let (batch, ingredients) = recipies.get(kind).unwrap();

    let num_batches = (quantity + batch - 1) / batch;
    let remainder = batch * num_batches - quantity;

    let mut sum = 0;
    for (ingredient, ingredient_units) in ingredients {
        let mut total_units = ingredient_units * num_batches;
        let pantry_units = pantry.entry(ingredient.to_string()).or_default();
        let take_from_pantry = total_units.min(*pantry_units);
        total_units -= take_from_pantry;
        *pantry_units -= take_from_pantry;

        sum += cook(ingredient, total_units, recipies, pantry);
    }

    *pantry.entry(kind.to_string()).or_default() += remainder;

    sum
}

pub fn part_a(input: &str) -> usize {
    let recipies: HashMap<String, (usize, HashMap<String, usize>)> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let x: Vec<_> = line.split(" => ").collect();
            let input: HashMap<String, usize> =
                x[0].split(", ").map(|item| parse_item(item)).collect();
            let output = parse_item(x[1]);

            (output.0, (output.1, input))
        })
        .collect();

    let mut pantry = HashMap::default();
    cook("FUEL", 1, &recipies, &mut pantry)
}

pub fn part_b(input: &str) -> usize {
    let recipies: HashMap<String, (usize, HashMap<String, usize>)> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let x: Vec<_> = line.split(" => ").collect();
            let input: HashMap<String, usize> =
                x[0].split(", ").map(|item| parse_item(item)).collect();
            let output = parse_item(x[1]);

            (output.0, (output.1, input))
        })
        .collect();

    // ceil is > the answer
    let mut ceil: usize = 1000000000;
    // floor is <= the answer
    let mut floor: usize = 1;
    loop {
        let chk = (ceil + floor) / 2;
        let mut pantry = HashMap::default();
        let score = cook("FUEL", chk, &recipies, &mut pantry);
        if score > 1000000000000 {
            ceil = chk;
        } else {
            floor = chk;
        }

        if ceil - 1 == floor {
            return floor;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("exa.txt")), 165);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 443537);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2910558);
    }
}
