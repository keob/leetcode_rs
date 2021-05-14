pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        if num_rows < 1 {
            return res;
        }

        let mut cur = vec![1];
        for _ in 0..num_rows {
            let mut next = vec![1; cur.len() + 1];
            for i in 1..cur.len() {
                next[i] = cur[i - 1] + cur[i];
            }
            res.push(cur);
            cur = next;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        );
    }
}
