pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.as_bytes()
            .iter()
            .chain(t.as_bytes().iter())
            .fold(0, |acc, x| acc ^ x)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = String::from("abcd");
        let t = String::from("abcde");

        assert_eq!(Solution::find_the_difference(s, t), 'e');
    }

    #[test]
    fn test_2() {
        let s = String::from("");
        let t = String::from("y");

        assert_eq!(Solution::find_the_difference(s, t), 'y');
    }

    #[test]
    fn test_3() {
        let s = String::from("a");
        let t = String::from("aa");

        assert_eq!(Solution::find_the_difference(s, t), 'a');
    }
}
