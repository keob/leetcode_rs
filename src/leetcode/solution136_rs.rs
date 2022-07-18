pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![2, 2, 1];
        assert_eq!(Solution::single_number(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![4, 1, 2, 1, 2];
        assert_eq!(Solution::single_number(nums), 4);
    }
}
