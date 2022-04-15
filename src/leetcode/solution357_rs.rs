pub struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 10;
        }
        let (mut res, mut cur) = (10, 9);
        for i in 0..n - 1 {
            cur *= 9 - i;
            res += cur
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    }
}
