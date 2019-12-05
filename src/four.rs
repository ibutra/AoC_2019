

#[derive(Debug)]
struct DigitIter {
    digits: Vec<u64>
}

impl DigitIter {
    fn new(mut num: u64) -> Self {
        let mut digits = Vec::new();
        while num != 0 {
            digits.push(num % 10);
            num /= 10;
        }
        DigitIter {digits}
    }
}

impl Iterator for DigitIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.digits.pop()
    }
}

fn two_digits(a: u64) -> bool {
    let mut previous = 0;
    for digit in DigitIter::new(a) {
        if digit == previous {
            return true;
        }
        previous = digit;
    }
    false
}

fn exactly_two_digits(a: u64) -> bool {
    let mut previous = 0;
    let mut count = 1;
    for digit in DigitIter::new(a) {
        if digit == previous {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        previous = digit;
    }
    count == 2
}


fn increasing(a: u64) -> bool {
    let mut previous = 0;
    for digit in DigitIter::new(a) {
        if digit < previous {
            return false;
        }
        previous = digit;
    }
    true
}

pub fn run(a: u64, b: u64) {
    let codes: Vec<u64> = (a..=b).filter(|x| two_digits(*x)).filter(|x| increasing(*x)).collect();
    // print!("Possible Codes: ");
    // for code in codes {
    //     print!("{}, ", code);
    // }
    // println!("");
    let code1_len = codes.len();
    let codes_2: Vec<u64> = codes.into_iter().filter(|x| exactly_two_digits(*x)).collect();
    println!("Day4 Task1: Num Codes: {} Task2: {}", code1_len, codes_2.len());
}
