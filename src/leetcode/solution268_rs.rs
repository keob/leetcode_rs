pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut sum = (len * (len + 1) / 2) as i32;

        for num in nums.iter().take(len) {
            sum -= num;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![0];
        assert_eq!(Solution::missing_number(nums), 1);
    }
}
