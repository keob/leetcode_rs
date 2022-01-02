pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n < 4 {
            return 2;
        }

        let k = n / 2;
        if k % 2 == 0 {
            2 * (2 * Self::last_remaining(k / 2) - 1)
        } else {
            2 * 2 * Self::last_remaining(k / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::last_remaining(9), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::last_remaining(1), 1);
    }
}
