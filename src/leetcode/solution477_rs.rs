pub struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let (mut res, mut count, len) = (0, 0, nums.len() as i32);

        for offset in 0..30 {
            for n in nums.iter() {
                count += (n >> offset) & 1;
            }
            res += count * (len - count);
            count = 0;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![4, 14, 2];
        assert_eq!(Solution::total_hamming_distance(nums), 6);
    }
}
