pub mod homework_1;
pub mod types;

fn main() {
    // println!("Hello, world!");
    // println!("{:?}", std::mem::size_of::<String>());
    // println!("{:?}", std::mem::size_of::<usize>());
    // println!("{:?}", std::mem::size_of::<u64>());
    // println!("{:?}", std::mem::size_of::<u64>());
    let x = 10;
    take_u16(x);

    let m = MyType { a: vec![1] };
    let mut n = m.clone();
    println!("m.a:{:?} {:?}", m.a, m.a.as_ptr());
    println!("n.a:{:?} {:?}", n.a, n.a.as_ptr());
    n.a.push(2);
    println!("m:{:?}", m.a);

    println!("n:{:?}", n.a);
    println!("res: {}", life(&1, &2));
    println!("res: {}", life(&2, &1));
}

fn take_u16(x: u16) {
    println!("{:?}", x);
}

fn life<'a, 'b: 'a>(a: &'a i32, b: &'b i32) -> &'a i32 {
    println!("life a: {}", a);
    println!("life b: {}", b);
    if a > b {
        return a;
    }
    b
}

#[derive(Clone, Debug)]
struct MyType {
    pub a: Vec<i32>,
}
