pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut sum, mut carry) = (a, b);
        while carry != 0 {
            let tmp = sum;
            sum ^= carry;
            carry = (tmp & carry) << 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_sum(1, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_sum(2, 3), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_sum(-1, 1), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::get_sum(-1, -1), -2);
    }
}
