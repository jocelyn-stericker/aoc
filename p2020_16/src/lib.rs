use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut segments = input.trim().split("\n\n");
    let ranges = segments.next().unwrap();
    let _your = segments.next().unwrap();
    let nearby = segments.next().unwrap();

    let mut checks = Vec::new();

    for range in ranges.split('\n') {
        let mut parts = range.split(": ");
        let _name = parts.next().unwrap();
        let options = parts.next().unwrap();

        let mut checks_ = Vec::new();
        for option in options.split(" or ") {
            let mut parts = option.split('-');
            checks_.push((
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
        checks.push(checks_);
    }

    let mut rate = 0;

    for ticket in nearby.trim().split('\n').skip(1) {
        for num in ticket.split(',').map(|n| n.parse::<i64>().unwrap()) {
            let mut ok = false;
            'a: for check in &checks {
                for subcheck in check {
                    if num >= subcheck.0 && num <= subcheck.1 {
                        ok = true;
                        break 'a;
                    }
                }
            }

            if !ok {
                rate += num;
            }
        }
    }

    rate
}

pub fn part_b(input: &str) -> i64 {
    let mut segments = input.trim().split("\n\n");
    let ranges = segments.next().unwrap();
    let your = segments.next().unwrap();
    let nearby = segments.next().unwrap();

    let mut checks = Vec::new();

    for range in ranges.split('\n') {
        let mut parts = range.split(": ");
        let name = parts.next().unwrap();
        let options = parts.next().unwrap();

        let mut checks_ = Vec::new();
        for option in options.split(" or ") {
            let mut parts = option.split('-');
            checks_.push((
                name.to_owned(),
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
        checks.push(checks_);
    }

    let mut valid_tickets = Vec::new();

    'o: for ticket in nearby.trim().split('\n').skip(1) {
        for num in ticket.split(',').map(|n| n.parse::<i64>().unwrap()) {
            let mut ok = false;
            'a: for check in &checks {
                for subcheck in check {
                    if num >= subcheck.1 && num <= subcheck.2 {
                        ok = true;
                        break 'a;
                    }
                }
            }
            if !ok {
                continue 'o;
            }
        }
        valid_tickets.push(
            ticket
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut constraints = HashMap::new();

    for check in &checks {
        let mut c = Vec::new();
        for i in 0..checks.len() {
            let mut ok = true;
            for ticket in &valid_tickets {
                let num = ticket[i];
                let mut any = false;
                for subcheck in check {
                    if num >= subcheck.1 && num <= subcheck.2 {
                        any = true;
                    }
                }
                if !any {
                    ok = false;
                    break;
                }
            }

            if ok {
                c.push(i);
            }
        }
        constraints.insert(check[0].0.clone(), c);
    }

    let mut solved = HashMap::new();

    loop {
        let mut did_something = false;
        for (k, v) in constraints.iter() {
            let items: Vec<_> = v.iter().filter(|i| !solved.contains_key(i)).collect();
            if items.len() == 1 {
                solved.insert(items[0], k.clone());
                did_something = true;
            }
            //
        }
        if !did_something {
            break;
        }
    }

    let your: Vec<_> = your
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|k| k.parse::<i64>().unwrap())
        .collect();
    let mut mul = 1;
    for (k, v) in solved.iter() {
        if v.starts_with("departure") {
            mul *= your[**k];
        }
    }
    mul
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 25916);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2564529489989);
    }
}
