pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        matches!(num, 6 | 28 | 496 | 8128 | 33550336)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_perfect_number(28), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::check_perfect_number(6), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::check_perfect_number(496), true);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::check_perfect_number(8128), true);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::check_perfect_number(2), false);
    }
}
