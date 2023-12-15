advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        /* Iterate over characters from front until digit is found */
        let c1 = line
            .chars()
            .find(|c| c.is_numeric())
            .expect("We expect there to always be a first digit");

        let c2 = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .expect("We expect there to always be a last digit");

        /* Covert chars to int */
        let d1 = c1.to_digit(10).unwrap();
        let d2 = c2.to_digit(10).unwrap();
        sum += d1 * 10 + d2;
    }

    Some(sum)
}

struct DigitStr {
    text: &'static str,
    value: u32,
}

static DIGIT_STRS: [DigitStr; 18] = [
    DigitStr {
        text: "1",
        value: 1,
    },
    DigitStr {
        text: "2",
        value: 2,
    },
    DigitStr {
        text: "3",
        value: 3,
    },
    DigitStr {
        text: "4",
        value: 4,
    },
    DigitStr {
        text: "5",
        value: 5,
    },
    DigitStr {
        text: "6",
        value: 6,
    },
    DigitStr {
        text: "7",
        value: 7,
    },
    DigitStr {
        text: "8",
        value: 8,
    },
    DigitStr {
        text: "9",
        value: 9,
    },
    DigitStr {
        text: "one",
        value: 1,
    },
    DigitStr {
        text: "two",
        value: 2,
    },
    DigitStr {
        text: "three",
        value: 3,
    },
    DigitStr {
        text: "four",
        value: 4,
    },
    DigitStr {
        text: "five",
        value: 5,
    },
    DigitStr {
        text: "six",
        value: 6,
    },
    DigitStr {
        text: "seven",
        value: 7,
    },
    DigitStr {
        text: "eight",
        value: 8,
    },
    DigitStr {
        text: "nine",
        value: 9,
    },
];

pub fn find_first_digit(input: &str) -> u32 {
    DIGIT_STRS
        .iter()
        .filter_map(|ds| input.find(ds.text).map(|i| (i, ds)))
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .1
        .value
}

pub fn find_last_digit(input: &str) -> u32 {
    DIGIT_STRS
        .iter()
        .filter_map(|ds| input.rfind(ds.text).map(|i| (i, ds)))
        .max_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .1
        .value
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let d1 = find_first_digit(line);
        let d2 = find_last_digit(line);

        let n = d1 * 10 + d2;
        sum += n;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(379));
    }
}
