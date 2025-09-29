/// https://leetcode.com/problems/reverse-integer/
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };

        let reversed = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_default(); // the spec says to return 0 in case of overflows

        reversed * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }
    #[test]
    fn case_3() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
