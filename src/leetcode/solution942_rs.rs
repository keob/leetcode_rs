pub struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = vec![0; s.len() + 1];
        let (mut left, mut right) = (0, s.len() as i32);

        s.chars().enumerate().for_each(|(i, c)| {
            if c == 'I' {
                res[i] = left;
                left += 1;
            } else {
                res[i] = right;
                right -= 1;
            }
        });
        res[s.len()] = left;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = String::from("IDID");
        assert_eq!(Solution::di_string_match(s), vec![0, 4, 1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let s = String::from("III");
        assert_eq!(Solution::di_string_match(s), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let s = String::from("DDI");
        assert_eq!(Solution::di_string_match(s), vec![3, 2, 0, 1]);
    }
}
