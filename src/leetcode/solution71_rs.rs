pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut queue = Vec::new();
        path.split('/').for_each(|level| match level {
            "" | "." => (),
            ".." => {
                queue.pop();
            }
            _ => queue.push(level),
        });
        "/".to_string() + &queue.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let path = String::from("/home/");
        let want = String::from("/home");
        assert_eq!(Solution::simplify_path(path), want);
    }

    #[test]
    fn test_2() {
        let path = String::from("/../");
        let want = String::from("/");
        assert_eq!(Solution::simplify_path(path), want);
    }

    #[test]
    fn test_3() {
        let path = String::from("/home//foo");
        let want = String::from("/home/foo");
        assert_eq!(Solution::simplify_path(path), want);
    }

    #[test]
    fn test_4() {
        let path = String::from("/a/./b/../../c/");
        let want = String::from("/c");
        assert_eq!(Solution::simplify_path(path), want);
    }
}
