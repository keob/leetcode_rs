pub struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let max_row = grid
            .iter()
            .map(|rows| rows.iter().max().unwrap())
            .sum::<i32>();

        let max_col = (0..grid.len())
            .map(|i| grid.iter().map(|col| col[i]).max().unwrap())
            .sum::<i32>();

        grid.iter()
            .map(|r| r.iter().filter(|c| **c != 0).count() as i32)
            .sum::<i32>()
            + max_row
            + max_col
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::projection_area(grid), 17);
    }

    #[test]
    fn test_2() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 2]];
        assert_eq!(Solution::projection_area(grid), 8);
    }
}
