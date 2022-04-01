pub struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&x| {
                let mut tmp = x;
                while tmp > 0 {
                    let digit = tmp % 10;
                    if digit == 0 || x % digit != 0 {
                        return false;
                    }
                    tmp /= 10;
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let want: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
        assert_eq!(Solution::self_dividing_numbers(1, 22), want);
    }

    #[test]
    fn test_2() {
        let want: Vec<i32> = vec![48, 55, 66, 77];
        assert_eq!(Solution::self_dividing_numbers(47, 85), want);
    }
}
