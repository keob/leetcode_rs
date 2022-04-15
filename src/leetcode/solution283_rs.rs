pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
            right += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        let want: Vec<i32> = vec![1, 3, 12, 0, 0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn test_2() {
        let mut nums: Vec<i32> = vec![0, 0, 0, 1, 0, 1, 1];
        let want: Vec<i32> = vec![1, 1, 1, 0, 0, 0, 0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn test_3() {
        let mut nums: Vec<i32> = vec![0; 10];
        let want: Vec<i32> = vec![0; 10];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, want);
    }
}
