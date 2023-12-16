advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    instances: u32,
    hits: u32,
}

fn count_hits(winning: &[u32], numbers: &[u32]) -> u32 {
    /* For each number in winning, check if it is present in numbers. If yes, increase score */
    let mut hits = 0;
    for wn in winning {
        if numbers.contains(wn) {
            hits += 1;
        }
    }

    hits
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            /* Card <id>: w w w w w | n n n n n n n n */
            let (_, rest) = l.split_once(':').unwrap();
            let (winning, numbers) = rest.split_once('|').unwrap();

            let winning: Vec<u32> = winning
                .trim()
                .split(' ')
                .filter(|w| !w.is_empty())
                .map(|w| w.trim().parse().unwrap())
                .collect();
            let numbers: Vec<u32> = numbers
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect();

            let hits = count_hits(&winning, &numbers);

            Card { hits, instances: 1 }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    /* Calculate score of each card */
    Some(
        cards
            .iter()
            .map(|c| if c.hits > 0 { 2u32.pow(c.hits - 1) } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = parse_input(input);

    /* Figure out how many instances of each card there actually are */
    for i in 0..=cards.len() - 1 {
        /* The hits of the card tells us how many subsequent cards have been won */
        /* The number of instances of the card tells how many instaces of the won cards have to be added */
        let hits = cards[i].hits as usize;
        if hits == 0 {
            continue;
        }
        let instances = cards[i].instances;
        for c in cards.iter_mut().skip(i + 1).take(hits) {
            c.instances += instances;
        }
    }

    /* Count total number of cards */
    Some(cards.iter().map(|c| c.instances).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(31));
        assert_eq!(result, Some(30));
    }
}
