pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = Vec::new();

        for s in preorder.split_terminator(',') {
            if s != "#" {
                stack.push(s);
                continue;
            }
            while !stack.is_empty() && stack[stack.len() - 1] == "#" {
                stack.pop();
                if let Some(x) = stack.pop() {
                    if x == "#" {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            stack.push("#");
        }

        stack.len() == 1 && stack[0] == "#"
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let preorder = String::from("9,3,4,#,#,1,#,#,2,#,6,#,#");
        assert_eq!(Solution::is_valid_serialization(preorder), true);
    }

    #[test]
    fn test_2() {
        let preorder = String::from("1,#");
        assert_eq!(Solution::is_valid_serialization(preorder), false);
    }

    #[test]
    fn test_3() {
        let preorder = String::from("9,#,#,1");
        assert_eq!(Solution::is_valid_serialization(preorder), false);
    }
}
