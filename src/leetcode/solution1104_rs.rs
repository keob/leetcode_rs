pub struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut max = 2;

        while max <= label {
            max <<= 1;
        }

        let mut res: Vec<i32> = Vec::new();
        let mut now = label;

        while now > 0 {
            res.push(now);
            now = (max >> 2) + (max >> 1) - 1 - (now >> 1);
            max >>= 1;
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let want: Vec<i32> = vec![1, 3, 4, 14];
        assert_eq!(Solution::path_in_zig_zag_tree(14), want);
    }

    #[test]
    fn test_2() {
        let want: Vec<i32> = vec![1, 2, 6, 10, 26];
        assert_eq!(Solution::path_in_zig_zag_tree(26), want);
    }
}
