pub struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let (mut left, mut right) = (answer_key.chars(), answer_key.chars());
        let (mut t_cnt, mut f_cnt) = (0, 0);
        let mut res = 0;
        loop {
            while t_cnt.min(f_cnt) <= k {
                res = res.max(t_cnt + f_cnt);
                match right.next() {
                    Some('T') => {
                        t_cnt += 1;
                    }
                    Some('F') => {
                        f_cnt += 1;
                    }
                    _ => break,
                }
            }
            match left.next() {
                Some('T') => {
                    t_cnt -= 1;
                }
                Some('F') => {
                    f_cnt -= 1;
                }
                _ => break,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let answer_key = String::from("TTFF");
        assert_eq!(Solution::max_consecutive_answers(answer_key, 2), 4);
    }

    #[test]
    fn test_2() {
        let answer_key = String::from("TFFT");
        assert_eq!(Solution::max_consecutive_answers(answer_key, 1), 3);
    }

    #[test]
    fn test_3() {
        let answer_key = String::from("TTFTTFTT");
        assert_eq!(Solution::max_consecutive_answers(answer_key, 1), 5);
    }
}
