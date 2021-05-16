use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Trie<T> {
    root: Rc<RefCell<TrieNode<T>>>,
}

#[derive(Default)]
struct TrieNode<T> {
    left: Option<Rc<RefCell<TrieNode<T>>>>,
    right: Option<Rc<RefCell<TrieNode<T>>>>,
}

impl Trie<i32> {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(TrieNode::<i32>::default())),
        }
    }

    pub fn add(&mut self, x: i32) {
        let f = || Rc::new(RefCell::new(TrieNode::<i32>::default()));
        let mut p = self.root.clone();
        (0..31).rev().for_each(|k| {
            p = {
                let mut bor = p.borrow_mut();
                if x & (1 << k) == 0 {
                    bor.left.get_or_insert_with(f)
                } else {
                    bor.right.get_or_insert_with(f)
                }
                .clone()
            };
        });
    }

    fn get_max(&self, x: i32) -> i32 {
        let mut p = self.root.clone();
        let mut m = 0;

        (0..31).rev().for_each(|k| {
            p = {
                let bor = p.borrow();
                let (a, b) = if x & (1 << k) == 0 {
                    (bor.right.as_ref(), bor.left.as_ref())
                } else {
                    (bor.left.as_ref(), bor.right.as_ref())
                };
                if let Some(a) = a {
                    m |= 1 << k;
                    a.clone()
                } else if let Some(b) = b {
                    b.clone()
                } else {
                    unreachable!()
                }
            };
        });
        m
    }
}

pub struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        nums.into_iter().fold(0, |ans, x| {
            trie.add(x);
            max(ans, trie.get_max(x))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![3, 10, 5, 25, 2, 8];
        assert_eq!(Solution::find_maximum_xor(nums), 28);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![0];
        assert_eq!(Solution::find_maximum_xor(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![2, 4];
        assert_eq!(Solution::find_maximum_xor(nums), 6);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![8, 10, 2];
        assert_eq!(Solution::find_maximum_xor(nums), 10);
    }

    #[test]
    fn test_5() {
        let nums: Vec<i32> = vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70];
        assert_eq!(Solution::find_maximum_xor(nums), 127);
    }
}
