pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 1;
        let mut right = n - 1;
        let mut res = -1;

        while left <= right {
            let mid = (left + right) >> 1;
            let mut cnt = 0;

            for item in nums.iter() {
                if *item <= mid as i32 {
                    cnt += 1;
                }
            }

            if cnt <= mid {
                left = mid + 1;
            } else {
                right = mid - 1;
                res = mid as i32;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 1];
        assert_eq!(Solution::find_duplicate(nums), 1);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![1, 1, 2];
        assert_eq!(Solution::find_duplicate(nums), 1);
    }
}
