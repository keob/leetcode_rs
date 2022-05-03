pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&x| x & 1 == 0)
            .chain(nums.iter().filter(|&x| x & 1 == 1))
            .copied()
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![]);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![3, 1, 2, 4];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![2, 4, 3, 1]);
    }
}
