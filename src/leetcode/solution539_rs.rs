pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() > 1440 {
            return 0;
        }

        let mut res = i32::MAX;

        let mut times: Vec<i32> = time_points
            .iter()
            .map(|time_point| {
                time_point[0..2].parse::<i32>().unwrap() * 60
                    + time_point[3..].parse::<i32>().unwrap()
            })
            .collect();

        times.sort_unstable();

        (1..times.len()).for_each(|i| res = res.min(times[i] - times[i - 1]));
        res.min(times[0] + 1440 - times[times.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let time_points: Vec<String> = vec!["23:59".to_string(), "00:00".to_string()];
        assert_eq!(Solution::find_min_difference(time_points), 1);
    }

    #[test]
    fn test_2() {
        let time_points: Vec<String> = vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string(),
        ];
        assert_eq!(Solution::find_min_difference(time_points), 0);
    }
}
