macro_rules! times5 {
    ($left: expr) => {
        $left * 5
    };
}

macro_rules! multiply_add {
    ($a: expr, $b: expr, $c: expr) => {
        $a * ($b + $c)
    };
}

macro_rules! S {
    ($e:expr) => {
        String::from($e)
    };
}

macro_rules! vec_strs {
    (
        // 开始反复捕获
        $(
            // 每个反复必须包含一个表达式
            $element:expr
        )
        // 由逗号分隔
        ,
        // 0 或多次
        *
    ) => {
        // 在这个块内用大括号括起来，然后在里面写多条语句
        {
            let mut v = Vec::new();

            // 开始反复捕获
            $(
                // 每个反复会展开成下面表达式，其中 $element 被换成相应被捕获的表达式
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}

macro_rules! repeat_two {
    ($($i: ident)*, $($i2: ident)*) => {
        // $(let $i:(); let $i2:();)*
        $(
            let $i:();
            let $i2:();
        )*
    };
}

pub fn times5(left: u64) -> u64 {
    let world = S!("World");
    println!("Hello, {}!", world);
    times5!(left)
}

macro_rules! literals {
    ($($literal:literal) *) => {
        $(
            println!("{}", $literal);
        )*
    };
}

macro_rules! idents {
    ($($ident:ident)*) => {
        $(
            println!("{:?}", stringify!($ident));
        )*
    };
}

macro_rules! my_vec {
    () => {
        std::vec::Vec::new()
    };
    ($($el: expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });

    ($default: expr; $n: expr) => ({
        std::vec::from_elem($default, $n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times5_should_work() {
        assert_eq!(25, times5!(5));
        assert_eq!(50, times5!(2 * 5));
        assert_eq!(35, times5!(2 + 5));
    }

    #[test]
    fn multiply_add_should_work() {
        assert_eq!(10, multiply_add!(2, 2, 3));
    }

    #[test]
    fn vec_strs_should_work() {
        let v = vec_strs!("a", "b", "c");
        // error[E0277]: can't compare `&[&str; 3]` with `Vec<String>`
        // 因为 assert_eq! 源码中是 *left == *right 进行比较 所以左侧需要实现对右侧的 PartialEq Trait
        // 切片的 PartialEq 要求的是：
        // impl<A, B> PartialEq<[B]> for [A]
        // where A: PartialEq<B>
        // 所以首先需要切片和切片才可以比较
        // 其次，左侧需要实现针对 B 类型的PartialEq Trait
        // &str 实现了 PartialEq<String>，因为 String 实现了 Deref Trait，可以转换成 &str
        // 于是这个实现可以让二者进行比较：https://doc.rust-lang.org/stable/src/core/str/traits.rs.html#27

        // 1.将 Vec<String> 转换为切片(assert_eq!(&["a", "b", "c"], v.as_slice());)可以比较，是因为 &str 实现了 PartialEq<String>。这个地方和 String 实现了 Deref Trait 有关 ，可以将 &String 变成 &str。
        // 2.Vec<String> 可以和 &[&str] 比较，我理解也和 String 实现了 Deref Trait 有关，String解引用成&str，符合Vec<T> 实现 PartialEq 的要求，所以也不报错。
        // assert_eq!(&["a", "b", "c"], v);
        assert_eq!(&["a", "b", "c"], v.as_slice());
        assert_eq!(v, &["a", "b", "c"]);
    }

    #[test]
    fn literals_should_work() {
        literals!(-1  "hello world"  2.3  b'b'  true);
    }

    #[test]
    fn ident_should_work() {
        idents! {
            // _ <- `_` 不是标识符，而是一种模式
            foo
            async
            O_________O
            _____O_____
        }
    }

    #[test]
    fn my_vec_should_work() {
        let mut v: Vec<usize> = my_vec![];
        v.push(1);
        v.push(2);

        let v1 = my_vec!(1, 2, 3);
        println!("v1: {:?}", v1);

        let v2 = my_vec![1;3];
        println!("v2: {:?}", v2);
    }
}
