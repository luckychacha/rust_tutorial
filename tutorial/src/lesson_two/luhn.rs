pub fn luhn(cc_number: &str) -> bool {
    println!("raw: {:?}", cc_number);
    let cc_number = cc_number.trim();
    if cc_number.len() < 2 {
        return false;
    }
    if cc_number
        .chars()
        .any(|c| !c.is_ascii_digit() && !c.is_whitespace())
    {
        return false;
    }
    if let Some(validate_number) = cc_number.get(0..cc_number.len() - 1) {
        let mut luhn_sum = 0;
        for (idx, item) in validate_number
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .rev()
            .enumerate()
        {
            if let Some(d) = item.to_digit(10) {
                luhn_sum += calculate_luhn_digit((idx, d as i32));
            } else {
                return false;
            }
        }
        let check_digit = ((10 - luhn_sum % 10) % 10) as u32;
        let true_tail_char = char::from_digit(check_digit, 10).unwrap();

        return cc_number.ends_with(true_tail_char);
    }
    false
}

fn calculate_luhn_digit((idx, digit): (usize, i32)) -> i32 {
    let mut sum = 0;
    let mut res = if idx % 2 != 0 { digit } else { 2 * digit };
    while res > 0 {
        sum += res % 10;
        res /= 10;
    }
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
