fn duplicate(
    a: impl Clone + core::fmt::Debug,
) -> (impl Clone + core::fmt::Debug, impl Clone + core::fmt::Debug) {
    (a.clone(), a.clone())
}

// fn duplicate<T: Clone>(a: T) -> (T, T) {
//     (a.clone(), a.clone())
// }

fn main() {
    let foo = String::from("foo");
    let pair: (String, String) = duplicate(foo);
    println!("{:?}", pair);

    let fo: i32 = 10;
    let pair_fo = duplicate(fo);
    println!("{:?}", pair_fo);
}
