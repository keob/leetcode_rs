pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|x| x.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(Solution::maximum_wealth(accounts), 6);
    }

    #[test]
    fn test_2() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(Solution::maximum_wealth(accounts), 10);
    }

    #[test]
    fn test_3() {
        let accounts: Vec<Vec<i32>> = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(Solution::maximum_wealth(accounts), 17);
    }
}
