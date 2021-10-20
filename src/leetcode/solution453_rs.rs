pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        nums.iter().fold(0, |acc, n| n - min + acc)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3];
        assert_eq!(Solution::min_moves(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 1, 1];
        assert_eq!(Solution::min_moves(nums), 0);
    }
}
