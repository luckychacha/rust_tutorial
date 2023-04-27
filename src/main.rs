use rust_tutorial::homework_1::funcs::{foo, longest, multiply_elements, string_concat};
use rust_tutorial::types;
fn main() {
    // 1
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
    types::library::book_func();
    // println!("s1 concat s2 is: {}", result);
}
