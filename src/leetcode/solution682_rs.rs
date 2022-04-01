pub struct Solution;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut res: Vec<i32> = Vec::new();

        ops.iter().for_each(|x| {
            match x.as_str() {
                "+" => {
                    res.push(res[res.len() - 1] + res[res.len() - 2]);
                }
                "C" => {
                    res.pop();
                }
                "D" => {
                    res.push(res[res.len() - 1] * 2);
                }
                _ => {
                    res.push(x.parse().unwrap());
                }
            };
        });

        res.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ops = vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string(),
        ];
        assert_eq!(Solution::cal_points(ops), 30);
    }

    #[test]
    fn test_2() {
        let ops = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        assert_eq!(Solution::cal_points(ops), 27);
    }

    #[test]
    fn test_3() {
        let ops = vec!["1".to_string()];
        assert_eq!(Solution::cal_points(ops), 1);
    }
}
