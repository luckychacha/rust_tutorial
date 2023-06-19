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

pub fn times5(left: u64) -> u64 {
    let world = S!("World");
    println!("Hello, {}!", world);
    times5!(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times5_should_work() {
        assert_eq!(25, times5!(5));
        assert_eq!(50, times5!(2 * 5));
    }

    #[test]
    fn multiply_add_should_work() {
        assert_eq!(10, multiply_add!(2, 2, 3));
    }

    #[test]
    fn vec_strs_should_work() {
        let v = vec_strs!["a", "b", "c"];
        assert_eq!(v, &["a", "b", "c"]);
        // assert_eq!(&["a", "b", "c"], v);
    }
}
