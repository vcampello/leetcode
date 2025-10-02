/// https://leetcode.com/problems/valid-parentheses/description/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // An odd length string will always be false
        if !s.len().is_multiple_of(2) {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            // opening: push to stack and continue
            if matches!(ch, '(' | '[' | '{') {
                stack.push(ch);
                continue;
            };

            // closing: get the equivalent opening for the current closing character
            match (stack.pop(), ch) {
                (Some('('), ')') => (),
                (Some('['), ']') => (),
                (Some('{'), '}') => (),
                _ => return false,
            };
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn case_4() {
        assert!(Solution::is_valid("([])".to_string()));
    }

    #[test]
    fn case_5() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }

    #[test]
    fn case_6() {
        assert!(!Solution::is_valid("((".to_string()));
    }
}
