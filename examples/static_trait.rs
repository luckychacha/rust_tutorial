fn duplicate(
    a: impl Clone + core::fmt::Debug,
) -> (impl Clone + core::fmt::Debug, impl Clone + core::fmt::Debug) {
    (a.clone(), a.clone())
}

// fn duplicate<T: Clone>(a: T) -> (T, T) {
//     (a.clone(), a.clone())
// }

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let foo = String::from("foo");
    // let pair: (String, String) = duplicate(foo);
    let pair = duplicate(foo);
    println!("{:?}", pair);

    let fo: i32 = 10;
    let pair_fo = duplicate(fo);
    println!("{:?}", pair_fo);

    let op: fn(i32, i32) -> i32 = sum;

    let op1 = op;
    let op2 = op;

    println!("op = {:p}", op);
    println!("op1 = {:p}", op1);
    println!("op2 = {:p}", op2);

    assert_eq!(op1, op2);

    // let op3 = sum;
    // let op4 = sum;

    // assert!(op3 == op4);
}
