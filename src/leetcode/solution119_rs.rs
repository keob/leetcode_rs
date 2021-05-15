pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![];

        for i in 0..=row_index as usize {
            res.push(1);
            for l in (1..i as usize).rev() {
                res[l] += res[l - 1];
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
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }
}
