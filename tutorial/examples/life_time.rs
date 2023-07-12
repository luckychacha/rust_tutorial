fn test<'a>(a: &'a i32, b: &'a i32, c: &'a i32) -> (&'a i32, &'a i32) {
    let left = if a % 2 == 0 { a } else { b };

    let right = if b % 2 == 0 { b } else { c };

    (left, right)
}

fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    let res = test(&a, &b, &c);
    println!("res: {res:?}");
}
