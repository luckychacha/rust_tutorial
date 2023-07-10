use num_traits::Num;

// calculate the sum of numbers.
#[allow(dead_code)]
fn calculate_sum<T: Num + Copy>(numbers: &[T]) -> T {
    let mut sum = T::zero();
    for num in numbers {
        sum = sum + *num;
    }
    sum
}

// calculate the add of two numbers.
#[allow(dead_code)]
fn calculate_add<T>(a: T, b: T) -> T
where
    T: Num + Copy,
{
    a + b
}

// calculate the subtract of two numbers.
#[allow(dead_code)]
fn calculate_subtract<T: Num + Copy>(a: T, b: T) -> T {
    a - b
}

// calculate the multiply of two numbers.
#[allow(dead_code)]
fn calculate_multiply<T: Num + Copy>(a: T, b: T) -> T {
    a * b
}

// calculate the divide of two numbers.
#[allow(dead_code)]
fn calculate_divide<T: Num + Copy>(a: T, b: T) -> T {
    a / b
}

#[cfg(test)]
mod tests {
    use num_traits::{Float, NumCast};

    use super::*;

    #[test]
    fn test_calculate_sum() {
        // i32 type
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(calculate_sum(&numbers), 15);

        // f64 type
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(calculate_sum(&numbers), 15.0);

        // f32 type
        let numbers = [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32];
        assert_eq!(calculate_sum(&numbers), 15.0f32);
    }

    #[test]
    fn test_calculate_divide() {
        // i32 type
        assert_eq!(calculate_divide(10, 2), 5);

        // f64 type
        assert_eq!(calculate_divide(10.0, 2.0), 5.0);

        // f32 type
        assert_eq!(calculate_divide(10.0f32, 2.0f32), 5.0f32);
    }

    #[test]
    fn test_calculate_add_subtract_multiply_divide() {
        // i32 type
        assert_eq!(calculate_add(10, 2), 12);
        assert_eq!(calculate_subtract(10, 2), 8);
        assert_eq!(calculate_multiply(10, 2), 20);
        assert_eq!(calculate_divide(10, 2), 5);

        // f64 type
        assert_eq!(calculate_add(10.0, 2.0), 12.0);
        assert_eq!(calculate_subtract(10.0, 2.0), 8.0);
        assert_eq!(calculate_multiply(10.0, 2.0), 20.0);
        assert_eq!(calculate_divide(10.0, 2.0), 5.0);

        // f32 type
        assert_eq!(calculate_add(10.0f32, 2.0f32), 12.0f32);
        assert_eq!(calculate_subtract(10.0f32, 2.0f32), 8.0f32);
        assert_eq!(calculate_multiply(10.0f32, 2.0f32), 20.0f32);
        assert_eq!(calculate_divide(10.0f32, 2.0f32), 5.0f32);

        let a: i32 = 5;
        let b: f32 = 3.14;

        let result: f32 = a as f32 + b;

        println!("Result: {}", result);
    }
}
