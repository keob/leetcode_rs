pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut num = n;
        if num <= 0 {
            return false;
        }
        while num > 1 {
            if num % 2 == 0 {
                num /= 2;
            } else if num % 3 == 0 {
                num /= 3;
            } else if num % 5 == 0 {
                num /= 5;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_ugly(6), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_ugly(8), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::is_ugly(14), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::is_ugly(1), true);
    }
}
