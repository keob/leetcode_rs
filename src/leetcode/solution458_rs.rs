pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        if buckets == 1 {
            return 0;
        }
        let mut buckets = buckets;
        let mut res = 1;
        let base: i32 = minutes_to_test / minutes_to_die + 1;
        while buckets > base {
            buckets /= base;
            res += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
}
