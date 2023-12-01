use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let mut first_seen = false;
        let mut numbers: (char, char) = ('0', '0');
        for c in line.chars() {
            if c.is_numeric() {
                if !first_seen {
                    numbers.0 = c;
                    numbers.1 = c;
                    first_seen = true;
                } else {
                    numbers.1 = c;
                }
            }
        }
        sum += numbers.0.to_digit(10).unwrap() * 10 + numbers.1.to_digit(10).unwrap();
    }

    println!("The result is {}",sum);
}
