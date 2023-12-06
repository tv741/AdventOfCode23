use common_lib::get_input_cached;

static STRING_DIGITS: [(char, &str); 9] = [
    ('1', "one"),
    ('2', "two"),
    ('3', "three"),
    ('4', "four"),
    ('5', "five"),
    ('6', "six"),
    ('7', "seven"),
    ('8', "eight"),
    ('9', "nine"),
];

fn get_digits(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>()
}

fn sum_coordinates(digits: Vec<Vec<char>>) -> u32 {
    digits
        .iter()
        .map(|slice| {
            let mut s = String::new();
            s.push(*slice.first().unwrap());
            s.push(*slice.last().unwrap());
            s.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

fn replace_digit_words(input: &str) -> String {
    let mut start = 0;
    let mut end = 5;
    let mut new_input = String::new();

    while start != end {
        let input_window = &input[start..end];

        let first = input.chars().nth(start).unwrap();
        if first.is_ascii_digit() || (first == '\n') {
            new_input.push(first);
        } else {
            for (d, s) in STRING_DIGITS {
                if input_window.starts_with(s) {
                    new_input.push(d);
                    break;
                }
            }
        }

        start += 1;

        if end < input.len() {
            end += 1;
        };
    }

    new_input
}

fn main() {
    let input = get_input_cached(1, true).unwrap();

    let digits1 = get_digits(&input);
    let sum1 = sum_coordinates(digits1);
    println!("Part One: {sum1}");

    let new_input = replace_digit_words(&input);
    let digits2 = get_digits(&new_input);
    let sum2 = sum_coordinates(digits2);
    println!("Part Two: {sum2}");
}
