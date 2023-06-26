pub fn multiply_elements_with_iter(arr: [i32; 3], n: i32) -> [i32; 3] {
    arr.iter()
        .map(|item| item * n)
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

pub fn multiply_elements(arr: [i32; 3], n: i32) -> [i32; 3] {
    let mut res = [0; 3];

    for (idx, item) in arr.iter().enumerate() {
        res[idx] = item * n;
    }

    res
}

pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn foo(s: &str) -> usize {
    s.len()
}

pub fn string_concat(s1: String, s2: String) -> String {
    s1 + &s2
}

pub fn print_chars() {
    for c in ('a'..='z').chain('A'..='Z').rev() {
        print!("{}, ", c);
    }
}
