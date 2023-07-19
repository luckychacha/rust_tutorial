fn main() {
    let input: &[u8] = b"0123456789abcdef0123456789abcdef";
    let (_selector, input_stripped) = input.split_at(4);

    println!("_selector: {_selector:?} input_stripped: {input_stripped:?}");
    let mut cursor = 0;
    let mut next = || {
        let start = cursor;
        cursor += 2;
        // println!("start: {}, cursor: {}", start, cursor);
        &input_stripped[start..cursor]
    };

    let a = (next(), next());
    let b = (next(), next(), next(), next());

    println!("a: {a:?} b: {b:?}");

    let test = (0..2).map(|_| (next(), next())).collect::<Vec<_>>();

    println!("test: {test:?}");

    let next_test = (cursor..input_stripped.len())
        .step_by(8)
        .map(|i| {
            println!("---i:{} {:?}", i, &input_stripped[i..i + 8]);
            &input_stripped[i..i + 8]
        })
        .collect::<Vec<_>>();

    println!("next_test: {next_test:?}");
}
