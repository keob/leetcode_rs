pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .as_bytes()
            .iter()
            .fold(0, |ans, c| ans * 26 + (c - (b'A' - 1)) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::title_to_number(String::from("A")), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::title_to_number(String::from("AB")), 28);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::title_to_number(String::from("FXSHRXW")),
            2147483647
        );
    }
}
