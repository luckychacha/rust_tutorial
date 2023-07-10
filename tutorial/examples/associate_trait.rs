use std::fmt::{Error, Formatter};

#[derive(Clone)]
struct MyTuple<T>((T, T, T));

#[derive(Clone)]
struct MyStruct<T> {
    inner: T,
}

trait AssociateTypeTrait {
    type A;
    type B;
    fn get_inner(&self) -> Option<Self::A>;

    fn get_inner_2(&self) -> Option<Self::B>;
}

impl<T> AssociateTypeTrait for MyTuple<T>
where
    T: Clone,
{
    type A = (T, T, T);
    type B = T;
    fn get_inner(&self) -> Option<Self::A> {
        // println!("{}", self.0);
        Some(self.0.clone())
    }

    fn get_inner_2(&self) -> Option<Self::B> {
        // println!("{}", self.0);
        Some(self.0 .0.clone())
    }
}

impl<T> AssociateTypeTrait for MyStruct<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display,
{
    type A = T;
    type B = (T, T, T, T);
    fn get_inner(&self) -> Option<Self::A> {
        Some(self.inner.clone())
    }
    fn get_inner_2(&self) -> Option<Self::B> {
        // println!("{}", self.0);
        Some((
            self.inner.clone(),
            self.inner.clone(),
            self.inner.clone(),
            self.inner.clone(),
        ))
    }
}

fn main() {
    let a = MyTuple((1, 2, 3));

    if let Some(value) = a.get_inner() {
        println!("MyTuple: {:?}", value);
    }

    if let Some(value) = a.get_inner_2() {
        println!("MyTuple2: {:?}", value);
    }

    let b = MyStruct {
        inner: String::from("abc"),
    };

    if let Some(value) = b.get_inner() {
        println!("MyStruct: {:?}", value);
    }
}
