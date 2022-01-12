pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for item in nums.into_iter() {
            if item < first {
                first = item;
            } else if item <= second {
                second = item;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert!(Solution::increasing_triplet(nums));
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![5, 4, 3, 2, 1];
        assert_eq!(Solution::increasing_triplet(nums), false);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![2, 1, 5, 0, 4, 6];
        assert!(Solution::increasing_triplet(nums));
    }
}
