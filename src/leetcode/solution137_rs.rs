pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut once, mut more) = (0, 0);

        for num in nums.iter() {
            once = !more & (once ^ num);
            more = !once & (more ^ num);
        }

        once
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![2, 2, 3, 2];
        assert_eq!(Solution::single_number(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![0, 1, 0, 1, 0, 1, 99];
        assert_eq!(Solution::single_number(nums), 99);
    }
}
