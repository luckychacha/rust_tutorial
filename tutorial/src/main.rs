use the_little_book_of_rust_macros::chapter_2::times5;
use tutorial::lesson_one::funcs::{foo, longest, multiply_elements, string_concat};
use tutorial::struct_generator::generate;
use tutorial::{the_little_book_of_rust_macros, types};

fn main() {
    println!("{:?}", multiply_elements([1, 2, 3], 2));

    // 2
    // let s1 = String::from("short");
    let s2 = String::from("long");
    let s3 = String::from("longest");

    let result;
    {
        // let s3 = String::from("longest");
        result = longest(s2.as_str(), s3.as_str());
    }

    println!("longest string is: {}", result);

    // 3
    let s = String::from("hello");
    let length = foo(&s);
    println!("the length of '{}' is {}", s, length);

    // 4
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let result = string_concat(s1.clone(), s2.clone());
    println!("s1:{} concat s2:{} is: {}", s1, s2, result);

    // 5
    types::library::book_func();
    // println!("s1 concat s2 is: {}", result);

    types::user::print_info();

    times5(5);

    generate();
}

// use chrono to calculate days between dates
fn calculate_days_between_dates(start: &str, end: &str) -> i32 {
    let start_date = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap();
    let end_date = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap();
    let days = end_date.signed_duration_since(start_date).num_days();
    // need to think about if days is larger than i32::MAX
    match i32::try_from(days).ok() {
        Some(days) => days,
        None => panic!("days is larger than i32::MAX"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_days_between_dates() {
        let start = "2021-01-01";
        let end = "2021-01-31";
        let days = calculate_days_between_dates(start, end);
        assert_eq!(days, 30);

        let start = "2021-01-01";
        let end = "2021-02-01";
        let days = calculate_days_between_dates(start, end);
        assert_eq!(days, 31);

        // panic test case
    }
}
