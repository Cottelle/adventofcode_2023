use core::num;
use std::fs;

#[derive(Debug, Clone)]
struct Number {
    value: u64,
    line: usize,
    begin: usize,
    end: usize,
}

impl Number {
    pub fn new(number: u64, line: usize, end: usize) -> Self {
        let begin = end
            - (if number > 1000 {
                4
            } else if number > 100 {
                3
            } else if number > 10 {
                2
            } else {
                1
            });

        Self {
            value: number,
            line: line,
            begin: begin,
            end: end,
        }
    }

    fn found(
        c: char,
        line: &mut std::iter::Enumerate<std::str::Chars<'_>>,
        n_line: usize,
        begin: usize,
    ) -> (Self,Option<(usize,char)> ) {
        let mut number = Self {
            value: 0,
            line: n_line,
            begin: begin,
            end: begin,
        };
        number.value = c.to_digit(10).unwrap() as u64;

        let mut res = None;
        loop {
            match line.next() {
                Some((_, c)) if c.is_digit(10) => {
                    number.end += 1;
                    number.value = number.value * 10 + c.to_digit(10).unwrap() as u64;
                }
                Some(a) => {
                    res = Some(a);
                    break;
                }
                _ => break,
            }
        }
        (number,res)
    }
}
#[derive(Debug, Clone)]
struct Symbole {
    line: usize,
    index: usize,
}

impl Symbole {
    pub fn new(line: usize, index: usize) -> Self {
        Self {
            line: line,
            index: index,
        }
    }

    pub fn hit(&self, number: &Number) -> bool {
        if number.value == 617 {
            dbg!(number, self.line as i64);
        }
        if i64::abs((self.line as i64) - (number.line as i64)) <= 1 && neer(self.index, number) {
            return true;
        }
        false
    }
}

fn neer(index: usize, number: &Number) -> bool {
    for i in number.begin..number.end + 1 {
        if i64::abs((index as i64) - (i as i64)) <= 1 {
            return true;
        }
    }
    false
}
#[derive(Debug, Clone)]
struct NeedName {
    sum: u64,
    numbers: Vec<Number>,
    symoble: Vec<Symbole>,
}

impl NeedName {
    pub fn add(&mut self, num: Number) {
        for s in &self.symoble {
            if s.hit(&num) {
                self.sum += num.value;
                println!("{} has hit", num.value);
                return;
            }
        }
        self.numbers.push(num);
    }

    pub fn found_symbole(&mut self, line: usize, index: usize) {
        let s = Symbole::new(line, index);
        self.symoble.push(s.clone());
        let mut new_numbers = vec![];
        for n in &self.numbers {
            if s.hit(&n) {
                self.sum += n.value;
                println!("{} has hit2", n.value);
            } else {
                new_numbers.push(n.clone());
            }
        }
        self.numbers = new_numbers;
    }

    fn new() -> Self {
        Self {
            sum: 0,
            numbers: vec![],
            symoble: vec![],
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut needname = NeedName::new();

    for (n_line, line) in contents.lines().enumerate() {
        let mut line = line.chars().enumerate();
        loop {
            match line.next() {
                Some((i, c)) => {
                    if c.is_digit(10) {
                        let (number,res) = Number::found(c, &mut line, n_line, i);
                        needname.add(number);
                        if let Some((i,c)) =  res{
                            if c.is_ascii() && c != '.'{
                                needname.found_symbole(n_line, i);
                            }
                        }
                    } else if c.is_ascii() && c != '.' {
                        println!("-->{}",c);
                        needname.found_symbole(n_line, i);
                    }
                }
                None => break,
            }
        }
    }
    println!("The sum is {}", needname.sum);
}
