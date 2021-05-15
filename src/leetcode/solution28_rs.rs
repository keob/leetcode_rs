pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        let mut next = vec![0];

        for i in 1..n {
            let mut j = next[i - 1];
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }

            next.push(if needle[i] == needle[j] { j + 1 } else { j });
        }

        let mut j = 0;

        for (i, &c) in haystack.iter().enumerate() {
            while j > 0 && c != needle[j] {
                j = next[j - 1];
            }

            if needle[j] == c {
                j += 1;
            }

            if j == n {
                return (i + 1 - j) as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let haystack = String::from("hello");
        let needle = String::from("ll");
        assert_eq!(Solution::str_str(haystack, needle), 2);
    }

    #[test]
    fn test_2() {
        let haystack = String::from("aaaaa");
        let needle = String::from("bba");
        assert_eq!(Solution::str_str(haystack, needle), -1);
    }

    #[test]
    fn test_3() {
        let haystack = String::from("");
        let needle = String::from("");
        assert_eq!(Solution::str_str(haystack, needle), 0);
    }
}
