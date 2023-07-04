use num_traits::Num;

#[allow(dead_code)]
fn calculate_sum<T: Num + Copy>(numbers: &[T]) -> T {
    let mut sum = T::zero();
    for num in numbers {
        sum = sum + *num;
    }
    sum
}

#[cfg(test)]
mod tests {
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
}
