pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let (mut n, mut res) = (n, 0);
        while n > 0 {
            n /= 5;
            res += n
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
