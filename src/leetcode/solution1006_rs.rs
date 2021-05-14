pub struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            3 => 6,
            4 => 7,
            _ => match n % 4 {
                0 => n + 1,
                1 | 2 => n + 2,
                _ => n - 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::clumsy(4), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::clumsy(10), 12);
    }
}
