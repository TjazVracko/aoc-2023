advent_of_code::solution!(3);

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for (x, line) in lines.iter().enumerate() {
        // println!("Checking line: {line}");

        let mut iter = line.chars().enumerate();
        while let Some((y, c)) = iter.next() {
            if c.is_ascii_digit() {
                /* Find all subsequent digits so that we form the whole number */
                let num: String = line
                    .chars()
                    .skip(y)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let num_len = num.len();
                // println!("{num}, {num_len}");
                let num: u32 = num.parse().unwrap();

                /* Determine neighbourhood that we must check for symbols */
                let xl = if x == 0 { 0 } else { x - 1 };
                let yl = if y == 0 { 0 } else { y - 1 };
                // println!(
                //     "Neighbour check around {num}. x[{xl} {}] , y[{yl} {}]",
                //     x + 1,
                //     y + num_len
                // );
                'outer: for xn in xl..=x + 1 {
                    for yn in yl..=y + num_len {
                        if let Some(ln) = lines.get(xn) {
                            if let Some(cn) = ln.chars().nth(yn) {
                                if is_symbol(cn) {
                                    // println!("{num} has symbol '{cn}'. x: {xn}, y: {yn}");
                                    sum += num;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                /* Skip digits we parsed above */
                for _ in 1..=num_len {
                    iter.next();
                }
            }
        }
    }

    Some(sum)
}

#[derive(Debug, Copy, Clone)]
struct PotentialGear {
    x: usize,
    y: usize,
    number: u32,
    used: bool,
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut potential_gears: Vec<PotentialGear> = Vec::new();

    for (x, line) in lines.iter().enumerate() {
        // println!("Checking line: {line}");

        let mut iter = line.chars().enumerate();
        while let Some((y, c)) = iter.next() {
            if c.is_ascii_digit() {
                /* Find all subsequent digits so that we form the whole number */
                let num: String = line
                    .chars()
                    .skip(y)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let num_len = num.len();
                // println!("{num}, {num_len}");
                let num: u32 = num.parse().unwrap();

                /* Determine neighbourhood that we must check for symbols */
                let xl = if x == 0 { 0 } else { x - 1 };
                let yl = if y == 0 { 0 } else { y - 1 };
                // println!(
                //     "Neighbour check around {num}. x[{xl} {}] , y[{yl} {}]",
                //     x + 1,
                //     y + num_len
                // );
                'outer: for xn in xl..=x + 1 {
                    for yn in yl..=y + num_len {
                        if let Some(ln) = lines.get(xn) {
                            if let Some(cn) = ln.chars().nth(yn) {
                                if cn == '*' {
                                    // println!("{num} has gear at x: {xn}, y: {yn}");
                                    potential_gears.push(PotentialGear {
                                        x: xn,
                                        y: yn,
                                        number: num,
                                        used: false,
                                    });
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                /* Skip digits we parsed above */
                for _ in 1..=num_len {
                    iter.next();
                }
            }
        }
    }

    let mut sum = 0;
    /* Go over all potential gears and select only those that appear exactly 2 times */

    for i in 0..potential_gears.len() {
        /* How many PG's are the same as the current PG? */
        let pg = potential_gears[i];
        if pg.used {
            continue;
        }

        let mut c = 0;
        let mut ratio = 1;
        for pgi in potential_gears.iter_mut() {
            if pgi.x == pg.x && pgi.y == pg.y {
                pgi.used = true;
                ratio *= pgi.number;
                c += 1;
            }
        }

        if c == 2 {
            sum += ratio;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
