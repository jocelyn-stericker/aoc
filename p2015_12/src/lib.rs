// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut neg = false;
    let mut sum = 0;
    let mut curr_num: Option<i64> = None;

    for c in input.trim().chars() {
        match (c, curr_num) {
            ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', None) => {
                curr_num = Some(c.to_string().parse().unwrap());
            }
            ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', Some(x)) => {
                curr_num = Some(x * 10 + c.to_string().parse::<i64>().unwrap());
            }
            ('-', None) => {
                neg = true;
            }
            ('-', Some(x)) => {
                if neg {
                    sum -= x;
                } else {
                    sum += x;
                }
                neg = true;
            }
            (_, None) => {
                neg = false;
                curr_num = None;
            }
            (_, Some(x)) => {
                if neg {
                    sum -= x;
                } else {
                    sum += x;
                }
                neg = false;
                curr_num = None;
            }
        }
    }

    if let Some(x) = curr_num {
        if neg {
            sum -= x;
        } else {
            sum += x;
        }
    }

    sum
}

/*
 * part_b was done in JavaScript because the question is stupid
 *
 * count = 0;
 * JSON.stringify(x, (key, value) => {
 *   if (!(value instanceof Array)) {
 *     for (const v of Object.values(value)) {
 *       if (v === 'red') {
 *         return null;
 *       }
 *     }
 *   }
 *   if (typeof value === 'number') {
 *     count += value;
 *   }
 *   return value;
 * });
 * console.log(count);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(r#"{"a":{"b":4},"c":-1}\n"#), 3);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 111754);
    }
}
