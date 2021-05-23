pub struct Solution;

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.len() % 2 == 0 || nums.iter().fold(0, |pre, cur| pre ^ cur) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 1, 2];
        assert_eq!(Solution::xor_game(nums), false);
    }
}
