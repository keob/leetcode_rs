pub struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64 + 0.5).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::bulb_switch(3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::bulb_switch(0), 0);
    }

    #[test]
    fn tset_3() {
        assert_eq!(Solution::bulb_switch(1), 1);
    }
}
