// fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
//     let mut res = &list[0];
//     for item in list {
//         if *item > *res {
//             res = item;
//         }
//     }
//     res
// }
struct MT {
    inner: i32,
}

impl PartialEq for MT {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd for MT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut res = &list[0];
    for item in list {
        if item > res {
            res = item;
        }
    }
    res
}

fn main() {
    let a = vec![1, 2, 3];
    let a_l = largest(&a);
    println!("{}", a_l);

    let c = vec![MT { inner: 1 }, MT { inner: 2 }];
    let l = largest(&c);
    println!("{}", l.inner);

    let b = vec!['a', 'b', 'c'];
    let largest_2 = crate::largest(&b);
    println!("{}", largest_2);
}
