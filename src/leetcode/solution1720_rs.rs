pub struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        let mut res = vec![first];

        for n in encoded {
            first ^= n;
            res.push(first);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let encoded: Vec<i32> = vec![1, 2, 3];
        let want: Vec<i32> = vec![1, 0, 2, 1];
        assert_eq!(Solution::decode(encoded, 1), want);
    }

    #[test]
    fn test_2() {
        let encoded: Vec<i32> = vec![6, 2, 7, 3];
        let want: Vec<i32> = vec![4, 2, 0, 7, 4];
        assert_eq!(Solution::decode(encoded, 4), want);
    }
}
