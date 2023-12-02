use std::fs;

const BLUE: u8 = 14;
const RED: u8 = 12;
const GREEN: u8 = 13;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut sum: u64 = 0;

    for game in contents.lines() {
        if let Some(value) = anylise_game(game) {
            sum+=value as u64;
        }
    }
    println!("The sum is {}", sum);
}

fn anylise_game(game: &str) -> Option<u8> {
    let split = game.split(" ").collect::<Vec<&str>>();
    let game_id = split[1].replace(":", "").parse::<u8>().unwrap();
    let mut iter = split[2..].iter();
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    loop {
        match iter.next() {
            None => break,
            Some(value) => {
                let value: u8 = value
                    .parse()
                    .expect(&format!("Can not convert into a int ({})", value));
                match iter.next() {
                    Some(color) => {
                        match color.replace(",", "").replace(";", "").as_str() {
                            "red" => {
                                red += value;
                            }

                            "blue" => {
                                blue += value;
                            }

                            "green" => {
                                green += value;
                            }

                            _ => {
                                panic!("Unknow color {}", color);
                            }
                        }
                        if color.contains(";") {
                            if !(GREEN >= green && BLUE >= blue && RED >= red) {
                                return None;
                            } else {
                                blue = 0;
                                red = 0;
                                green = 0;
                            }
                        }
                    }
                    None => panic!("No color link to the value"),
                }
            }
        }
    }

    if GREEN >= green && BLUE >= blue && RED >= red {
        Some(game_id)
    } else {
        None
    }
}
