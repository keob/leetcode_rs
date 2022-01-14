use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(nums1[0] + nums2[0]), 0, 0));
        let mut res = vec![];

        for _ in 0..k {
            if let Some((_, i, j)) = heap.pop() {
                res.push(vec![nums1[i], nums2[j]]);
                if i + 1 < nums1.len() && j == 0 {
                    heap.push((Reverse(nums1[i + 1] + nums2[j]), i + 1, j));
                }
                if j + 1 < nums2.len() {
                    heap.push((Reverse(nums1[i] + nums2[j]), i, j + 1));
                }
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
        let nums1: Vec<i32> = vec![1, 7, 11];
        let nums2: Vec<i32> = vec![2, 4, 6];
        let k = 3;
        let want: Vec<Vec<i32>> = vec![vec![1, 2], vec![1, 4], vec![1, 6]];
        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), want);
    }

    #[test]
    fn test_2() {
        let nums1: Vec<i32> = vec![1, 1, 2];
        let nums2: Vec<i32> = vec![1, 2, 3];
        let k = 2;
        let want: Vec<Vec<i32>> = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), want);
    }

    #[test]
    fn test_3() {
        let nums1: Vec<i32> = vec![1, 2];
        let nums2: Vec<i32> = vec![3];
        let k = 3;
        let want: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), want);
    }
}
