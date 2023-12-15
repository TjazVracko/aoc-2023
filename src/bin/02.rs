use std::cmp::Ordering;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn parse_hand(input: &str) -> Hand {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for c in input.split(',') {
        let c = c.trim();

        /* <Number><Space><Color> */
        let (num, rest) = c.split_once(' ').unwrap();
        let num: u32 = num.parse().unwrap();

        if rest.cmp("red") == Ordering::Equal {
            red += num;
        } else if rest.cmp("green") == Ordering::Equal {
            green += num;
        } else if rest.cmp("blue") == Ordering::Equal {
            blue += num;
        }
    }

    Hand { red, green, blue }
}

fn parse_line(input: &str) -> Game {
    /* Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green */

    /* Characters starting at 6 are game number */
    let (_game, rest) = input.split_once(' ').unwrap();
    let (id, rest) = rest.split_once(':').unwrap();

    let id: u32 = id.trim().parse().unwrap();

    Game {
        id,
        hands: rest.split(';').map(parse_hand).collect(),
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);

    /* Max cubes */
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    /* Count possible games */
    let mut game_id_sum = 0;
    for game in games {
        let invalid_hands = game
            .hands
            .iter()
            .filter(|h| h.red > max_red || h.green > max_green || h.blue > max_blue)
            .count();

        if invalid_hands == 0 {
            game_id_sum += game.id;
        }
    }

    Some(game_id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);

    /* Find max of each colour in each game */
    let mut total_power = 0;
    for game in games {
        /* Minimum number of cubes for each game */
        let mc = Hand {
            red: game
                .hands
                .iter()
                .max_by(|a, b| a.red.cmp(&b.red))
                .unwrap()
                .red,
            green: game
                .hands
                .iter()
                .max_by(|a, b| a.green.cmp(&b.green))
                .unwrap()
                .green,
            blue: game
                .hands
                .iter()
                .max_by(|a, b| a.blue.cmp(&b.blue))
                .unwrap()
                .blue,
        };

        total_power += mc.red * mc.green * mc.blue;
    }

    Some(total_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
