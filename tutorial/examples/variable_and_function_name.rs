fn largest<T: std::cmp::PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut res: T = list[0];
    for &item in list.iter() {
        if item > res {
            res = item;
        }
    }
    res
}

fn main() {
    let a = vec![1, 2, 3];
    let largest = largest(a);
    println!("{}", largest);

    let b = vec!['a', 'b', 'c'];
    let largest_2 = crate::largest(b);
    println!("{}", largest);
}
