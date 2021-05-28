pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let (mut res, mut s) = (0, x ^ y);

        while s > 0 {
            res += 1;
            s = s & (s - 1);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }
}
