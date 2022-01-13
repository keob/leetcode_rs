pub struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut first, mut second, mut res) = (0, 0, 0);

        for (idx, &num) in nums.iter().enumerate() {
            if num > first {
                res = idx;
                second = first;
                first = num;
            } else if num > second {
                second = num;
            }
        }
        if first >= second * 2 {
            return res as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![3, 6, 1, 0];
        assert_eq!(Solution::dominant_index(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(Solution::dominant_index(nums), -1);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1];
        assert_eq!(Solution::dominant_index(nums), 0);
    }
}
