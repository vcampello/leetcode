// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// https://leetcode.com/problems/add-two-numbers/
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut n1 = l1;
        let mut n2 = l2;
        let mut carry_over = 0;
        let mut result: Vec<i32> = Vec::new();

        loop {
            let (v1, v2) = match (n1, n2) {
                (Some(v1), Some(v2)) => {
                    n1 = v1.next;
                    n2 = v2.next;
                    (v1.val, v2.val)
                }
                (Some(v1), None) => {
                    n1 = v1.next;
                    n2 = None;
                    (v1.val, 0)
                }
                (None, Some(v2)) => {
                    n1 = None;
                    n2 = v2.next;
                    (0, v2.val)
                }
                (None, None) => {
                    n1 = None;
                    n2 = None;
                    (0, 0)
                }
            };

            let sum = v1 + v2 + carry_over;
            let val = sum % 10;
            carry_over = sum / 10;
            result.push(val);

            if n1.is_none() && n2.is_none() {
                if carry_over > 0 {
                    result.push(carry_over);
                }

                break;
            }
        }

        Solution::from_vec(result)
    }

    fn from_vec(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        // Build the linked list in reverse because it's too much
        // of a pain to track the head and current nodes.
        for &val in list.iter().rev() {
            let mut new_head = Box::new(ListNode::new(val));
            head = match head {
                // Change head
                Some(_) => {
                    new_head.next = head;
                    Some(new_head)
                }
                None => Some(new_head),
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = Solution::from_vec(vec![2, 4, 3]);
        let l2 = Solution::from_vec(vec![5, 6, 4]);
        let expected = Solution::from_vec(vec![7, 0, 8]);

        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        let l1 = Solution::from_vec(vec![0]);
        let l2 = Solution::from_vec(vec![0]);
        let expected = Solution::from_vec(vec![0]);

        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        let l1 = Solution::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = Solution::from_vec(vec![9, 9, 9, 9]);
        let expected = Solution::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);

        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }
}
