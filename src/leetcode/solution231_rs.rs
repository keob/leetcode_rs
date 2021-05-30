pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_power_of_two(16), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_power_of_two(3), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_power_of_two(4), true);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::is_power_of_two(5), false);
    }
}
