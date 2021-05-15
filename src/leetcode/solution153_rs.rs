pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while nums[left] > nums[right] {
            let mid = (right - left) / 2 + left;

            if nums[right] < nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 1];
        assert_eq!(Solution::find_min(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![11, 13, 15, 17];
        assert_eq!(Solution::find_min(nums), 11);
    }
}
