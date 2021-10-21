pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
            }
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let digits: Vec<i32> = vec![1, 2, 3];
        let want: Vec<i32> = vec![1, 2, 4];
        assert_eq!(Solution::plus_one(digits), want);
    }

    #[test]
    fn test_2() {
        let digits: Vec<i32> = vec![4, 3, 2, 1];
        let want: Vec<i32> = vec![4, 3, 2, 2];
        assert_eq!(Solution::plus_one(digits), want);
    }

    #[test]
    fn test_3() {
        let digits: Vec<i32> = vec![0];
        let want: Vec<i32> = vec![1];
        assert_eq!(Solution::plus_one(digits), want);
    }
}
