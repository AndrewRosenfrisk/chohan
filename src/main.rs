use core::fmt;
use rand::Rng;
use std::io::stdin;

fn main() {
    println!(
        "In this traditional Japanese dice game, two dice are rolled in a bamboo
cup by the dealer sitting on the floor. The player must guess if the
dice total to an even (cho) or odd (han) number."
    );

    let mut purse = Purse(5000);
    let mut bet: u64;

    'game: loop {
        'input: loop {
            println!(
                "You have {} mon. How much do you bet? (or Q to quit)",
                purse.0
            );
            let input = get_player_input();

            let command = parse_command_from_input(input, &purse);

            match command {
                Command::QUIT => {
                    println!("Thanks for playing!");
                    break 'game;
                }
                Command::PROMPT => {
                    println!(
                        "Enter a positive whole number, up to your total mon ({}).",
                        purse.0
                    );
                    continue;
                }
                Command::BET(x) => {
                    bet = x;
                    break 'input;
                }
            }
        }

        let dice = roll_dice();

        println!("The dealer swirls the cup and you hear the rattle of the dice.");
        println!(
        "The dealer slams the cup on the floor, still covering the dice and asks for your bet.\n"
    );
        println!("CHO (even) or HAN (odd)?");

        let mut parity: Option<Parity>;
        'input: loop {
            let input = get_player_input();
            parity = parse_parity_from_input(input);

            if parity.is_none() {
                println!("Please enter either \"CHO\" or \"HAN\".");
                continue;
            } else {
                break 'input;
            }
        }

        println!("The dealer lifts the cup to reveal: ");
        println!(
            "    {}-{}",
            get_japanese_number(dice.0).unwrap(),
            get_japanese_number(dice.1).unwrap()
        );
        println!("      {}-{}", dice.0, dice.1);

        let win = is_bet_correct(dice, parity.unwrap());

        if win {
            let fee = bet / 10;
            purse.adjust(bet - fee, win);
            println!("You won! You take {} mon.", bet);
            println!("The house collects a {} mon fee.", fee);
        } else {
            purse.adjust(bet, win);
            println!("You lost!");

            if is_game_over(&purse) {
                println!("You have run out of money!\nThanks for playing!");
                break 'game;
            }
        }
    }
}

fn roll_dice() -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

fn sum_dice(dice: (u8, u8)) -> u8 {
    dice.0 + dice.1
}

fn mod_dice(dice: (u8, u8)) -> Parity {
    if sum_dice(dice) % 2 == 0 {
        Parity::CHO
    } else {
        Parity::HAN
    }
}

fn is_bet_correct(dice: (u8, u8), bet: Parity) -> bool {
    if bet == mod_dice(dice) {
        true
    } else {
        false
    }
}

fn get_player_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_uppercase()
}

fn parse_command_from_input(input: String, purse: &Purse) -> Command {
    if input == "Q" {
        Command::QUIT
    } else {
        let bet = input.parse::<u64>().unwrap_or(0);
        if bet > 0 && bet <= purse.0 {
            Command::BET(bet)
        } else {
            Command::PROMPT
        }
    }
}

fn parse_parity_from_input(input: String) -> Option<Parity> {
    if input == "CHO" {
        Some(Parity::CHO)
    } else if input == "HAN" {
        Some(Parity::HAN)
    } else {
        None
    }
}

fn is_game_over(purse: &Purse) -> bool {
    if purse.0 == 0 {
        true
    } else {
        false
    }
}

fn get_japanese_number(number: u8) -> Option<Number> {
    match number {
        1 => Some(Number::ICHI),
        2 => Some(Number::NI),
        3 => Some(Number::SAN),
        4 => Some(Number::SHI),
        5 => Some(Number::GO),
        6 => Some(Number::ROKU),
        _ => None,
    }
}

#[derive(PartialEq, Debug)]
enum Parity {
    CHO,
    HAN,
}
#[derive(PartialEq, Debug)]
enum Command {
    QUIT,
    BET(u64),
    PROMPT,
}
#[derive(PartialEq, Debug)]
enum Number {
    ICHI,
    NI,
    SAN,
    SHI,
    GO,
    ROKU,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::ICHI => write!(f, "ICHI"),
            Number::NI => write!(f, "NI"),
            Number::SAN => write!(f, "SAN"),
            Number::SHI => write!(f, "SHI"),
            Number::GO => write!(f, "GO"),
            Number::ROKU => write!(f, "ROKU"),
        }
    }
}

struct Purse(u64);

impl Purse {
    fn adjust(&mut self, bet: u64, is_positive: bool) {
        if is_positive {
            self.0 += bet
        } else {
            self.0 -= bet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ODD_DICE: (u8, u8) = (1, 4);
    const EVEN_DICE: (u8, u8) = (2, 4);

    #[test]
    fn dice_sum_correctly() {
        assert_eq!(sum_dice(EVEN_DICE), 6);
    }
    #[test]
    fn dice_mod_correctly() {
        assert_eq!(mod_dice(EVEN_DICE), Parity::CHO);
        assert_eq!(mod_dice(ODD_DICE), Parity::HAN);
    }
    #[test]
    fn bet_validates() {
        assert!(is_bet_correct(EVEN_DICE, Parity::CHO));
        assert!(is_bet_correct(ODD_DICE, Parity::HAN));
    }
    #[test]
    fn command_parses() {
        let purse = Purse(500);
        assert_eq!(
            parse_command_from_input("Q".to_string(), &purse),
            Command::QUIT
        );
        assert_eq!(
            parse_command_from_input("123".to_string(), &purse),
            Command::BET(123)
        );
        assert_eq!(
            parse_command_from_input("501".to_string(), &purse),
            Command::PROMPT
        );
        assert_eq!(
            parse_command_from_input("-82937".to_string(), &purse),
            Command::PROMPT
        );
        assert_eq!(
            parse_command_from_input("1.054".to_string(), &purse),
            Command::PROMPT
        );
        assert_eq!(
            parse_command_from_input("daf12198jkshd[]1/.#$".to_string(), &purse),
            Command::PROMPT
        );
    }

    #[test]
    fn parity_parses() {
        assert_eq!(
            parse_parity_from_input("CHO".to_string()),
            Some(Parity::CHO)
        );
        assert_eq!(
            parse_parity_from_input("HAN".to_string()),
            Some(Parity::HAN)
        );
        assert_eq!(parse_parity_from_input("ladfjio123".to_string()), None)
    }

    #[test]
    fn game_ends() {
        assert!(!is_game_over(&Purse(1)));
        assert!(is_game_over(&Purse(0)));
    }

    #[test]
    fn translates_numbers() {
        assert_eq!(get_japanese_number(1), Some(Number::ICHI));
        assert_eq!(get_japanese_number(2), Some(Number::NI));
        assert_eq!(get_japanese_number(3), Some(Number::SAN));
        assert_eq!(get_japanese_number(4), Some(Number::SHI));
        assert_eq!(get_japanese_number(5), Some(Number::GO));
        assert_eq!(get_japanese_number(6), Some(Number::ROKU));
        assert_eq!(get_japanese_number(0), None);
        assert_eq!(get_japanese_number(7), None);
    }

    #[test]
    fn purse_adjusts() {
        let mut purse = Purse(100);

        purse.adjust(100, true);
        assert_eq!(purse.0, 200);

        purse.adjust(200, false);
        assert_eq!(purse.0, 0)
    }
}
