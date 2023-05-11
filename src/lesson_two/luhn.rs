pub fn luhn(cc_number: &str) -> bool {
    println!("raw: {:?}", cc_number);
    let cc_number = cc_number.trim();
    if cc_number.len() < 2 {
        return false;
    }
    if cc_number
        .chars()
        .find(|&c| !c.is_digit(10) && !c.is_whitespace())
        .is_some()
    {
        return false;
    }
    if let Some(validate_number) = cc_number.get(0..cc_number.len() - 1) {
        let mut acc = 0;
        for (idx, item) in validate_number
            .chars()
            .filter(|&item| item.is_numeric())
            .rev()
            .enumerate()
        {
            match item.to_digit(10) {
                Some(d) => acc += deal_luhn((idx, d as i32)),
                None => return false,
            }
        }
        let digit = ((10 - acc % 10) % 10) as u32;
        let true_tail_char: char = char::from_digit(digit, 10).unwrap();

        return cc_number.ends_with(true_tail_char);
    }
    false
}

fn deal_luhn(input: (usize, i32)) -> i32 {
    let mut sum = 0;
    let mut res = if input.0 % 2 != 0 {
        println!("{}", input.1);
        input.1
    } else {
        println!("2 * {}", input.1);
        2 * input.1
    };
    while res % 10 != 0 {
        sum += res % 10;
        res = res / 10;
    }
    println!("sum: {sum:?}");
    sum
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("7992 7398 713"));
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
