/// https://leetcode.com/problems/generate-parentheses/description/
pub struct Solution;

impl Solution {
    fn r(target: i32, opened: i32, closed: i32, s: &str, cache: &mut Vec<String>) {
        if closed == target {
            // println!("[{opened}, {closed}] solved\t= {s} ");
            cache.push(s.to_string());
            return;
        }

        if opened < target {
            // println!("[{opened}, {closed}] open\t= {s} ");
            Self::r(target, opened + 1, closed, &format!("{s}("), cache);
        }

        if closed < opened {
            // println!("[{opened}, {closed}] close\t= {s} ");
            Self::r(target, opened, closed + 1, &format!("{s})"), cache);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache: Vec<String> = Vec::new();
        if n < 1 {
            return cache;
        }

        Solution::r(n, 0, 0, "", &mut cache);

        cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(3), expected);
    }

    #[test]
    fn case_2() {
        let expected = vec!["()"];
        assert_eq!(Solution::generate_parenthesis(1), expected);
    }
}
