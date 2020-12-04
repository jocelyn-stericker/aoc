use std::collections::HashSet;

pub fn part_b(input: &str) -> i64 {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut pp = Vec::new();
    let mut set = HashSet::new();
    for line in input.trim().split('\n') {
        if line == "" {
            pp.push(set);
            set = HashSet::new();
            continue;
        }
        let mut parts = line.split(':');
        let field = parts.next().unwrap();
        let val = parts.next().unwrap();
        if field == "byr" {
            let val = val.parse::<i64>().unwrap();
            if val < 1920 || val > 2002 {
                continue;
            }
        }
        if field == "iyr" {
            let val = val.parse::<i64>().unwrap();
            if val < 2010 || val > 2020 {
                continue;
            }
        }
        if field == "eyr" {
            let val = val.parse::<i64>().unwrap();
            if val < 2020 || val > 2030 {
                continue;
            }
        }
        if field == "hgt_cm" {
            let val = val.parse::<i64>().unwrap();
            if val < 150 || val > 193 {
                continue;
            }
            set.insert("hgt");
            continue;
        }
        if field == "hgt_in" {
            let val = val.parse::<i64>().unwrap();
            if val < 59 || val > 76 {
                continue;
            }
            set.insert("hgt");
            continue;
        }
        if field == "ecl"
            && val != "amb"
            && val != "blu"
            && val != "brn"
            && val != "gry"
            && val != "grn"
            && val != "hzl"
            && val != "oth"
        {
            continue;
        }
        set.insert(field);
    }
    pp.push(set);

    let mut valids = 0;
    'a: for pp in &pp {
        for field in &fields {
            if !pp.contains(field) {
                continue 'a;
            }
        }
        valids += 1;
    }

    valids
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 145);
    }
}
