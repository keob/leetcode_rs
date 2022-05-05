pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        let (mut res, mut mul) = (0, 1);

        while right < nums.len() {
            mul *= nums[right];
            while left <= right && mul >= k {
                mul /= nums[left];
                left += 1;
            }
            res += (right as i32 - left as i32) + 1;
            right += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![10, 5, 2, 6];
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, 100), 8);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3];
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, 0), 0);
    }
}
