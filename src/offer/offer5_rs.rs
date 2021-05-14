pub struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut ans = String::from("");

        for c in s.chars() {
            match c {
                ' ' => ans.push_str("%20"),
                _ => ans.push(c),
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = String::from("We are happy.");
        let want = String::from("We%20are%20happy.");
        assert_eq!(Solution::replace_space(s), want);
    }
}
