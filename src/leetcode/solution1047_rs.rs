pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if let Some(ch) = stack.last() {
                if ch == &c {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }

        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_duplicates(String::from("abbaca")),
            String::from("ca")
        );
    }
}
