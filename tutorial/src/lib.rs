pub mod the_little_book_of_rust_macros;

pub mod effective_rust;
pub mod lesson_one;
pub mod lesson_two;
pub mod struct_generator;
pub mod types;

// fn test_func() {
//     // println!("Hello, world!");
//     // println!("{:?}", std::mem::size_of::<String>());
//     // println!("{:?}", std::mem::size_of::<usize>());
//     // println!("{:?}", std::mem::size_of::<u64>());
//     // println!("{:?}", std::mem::size_of::<u64>());
//     let x = 10;
//     take_u16(x);

//     let m = MyType { a: vec![1] };
//     let mut n = m.clone();
//     println!("m.a:{:?} {:?}", m.a, m.a.as_ptr());
//     println!("n.a:{:?} {:?}", n.a, n.a.as_ptr());
//     n.a.push(2);
//     println!("m:{:?}", m.a);

//     println!("n:{:?}", n.a);
//     println!("res: {}", life(&1, &2));
//     println!("res: {}", life(&2, &1));
// }

// fn take_u16(x: u16) {
//     println!("{:?}", x);
// }

// fn life<'a, 'b: 'a>(a: &'a i32, b: &'b i32) -> &'a i32 {
//     println!("life a: {}", a);
//     println!("life b: {}", b);
//     if a > b {
//         return a;
//     }
//     b
// }

// #[derive(Clone, Debug)]
// struct MyType {
//     pub a: Vec<i32>,
// }

// // pub const fn map<U, F>(self, f: F) -> Option<U>
// // where
// //     F: ~const FnOnce(T) -> U,
// //     F: ~const Destruct,
// // {
// //     match self {
// //         Some(x) => Some(f(x)),
// //         None => None,
// //     }
// // }

// // pub const fn and_then<U, F>(self, f: F) -> Option<U>
// // where
// //     F: ~const FnOnce(T) -> Option<U>,
// //     F: ~const Destruct,
// // {
// //     match self {
// //         Some(x) => f(x),
// //         None => None,
// //     }
// // }
