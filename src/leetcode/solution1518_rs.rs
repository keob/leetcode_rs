pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        if num_bottles < num_exchange {
            return num_bottles;
        }
        (num_bottles - num_exchange) / (num_exchange - 1) + 1 + num_bottles
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_water_bottles(5, 5), 6);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::num_water_bottles(2, 3), 2);
    }
}
