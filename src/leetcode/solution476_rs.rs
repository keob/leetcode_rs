pub struct Solution;

impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut result = 0;
        let mut tmp = 1;

        while num > 0 {
            if num & 1 == 0 {
                result |= tmp;
            }
            tmp <<= 1;
            num >>= 1;
        }
        result
    }
}
