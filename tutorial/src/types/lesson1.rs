use std::{fmt::Display, ops::Deref};

fn fromZtoa() {
    let _a = 0..5;
    let _b: std::ops::Range<char> = 'a'..'Z';

    let char_lower_a = 'a';
    let char_upper_z = 'Z';
    if char_lower_a < char_upper_z {
        println!("a > Z");
    } else {
        println!("a <= Z");
    }

    let bytes_a = char_lower_a.to_string().into_bytes();
    let bytes_emoji = "👍🏻".to_string().into_bytes();
    println!("bytes_a: {bytes_a:?}");
    println!("bytes_emoji: {bytes_emoji:?}");
}
