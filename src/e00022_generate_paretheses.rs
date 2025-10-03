/// https://leetcode.com/problems/generate-parentheses/description/
pub struct Solution;

impl Solution {
    fn permute(to_open: i32, to_close: i32, s: &str, cache: &mut Vec<String>) {
        // leaf node
        if to_close == 0 {
            // println!("[{to_open}, {to_close}] solved\t= {s} ");
            cache.push(s.to_string());
            return;
        }

        // explore opening
        if to_open > 0 {
            // println!("[{to_open}, {to_close}] open\t= {s} ");
            Self::permute(to_open - 1, to_close, &format!("{s}("), cache);
        }

        // explore closing
        if to_close > to_open {
            // println!("[{to_open}, {to_close}] close\t= {s} ");
            Self::permute(to_open, to_close - 1, &format!("{s})"), cache);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache: Vec<String> = Vec::new();

        Solution::permute(n, n, "", &mut cache);

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
