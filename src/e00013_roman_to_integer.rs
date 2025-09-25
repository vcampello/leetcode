/// https://leetcode.com/problems/roman-to-integer/
pub struct Solution;

impl Solution {
    fn to_int(s: &char) -> i32 {
        match s {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Could not find value for {s}"),
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut total: i32 = 0;
        let mut it = s.chars().peekable();

        while let Some(curr) = it.next() {
            let curr_int = Solution::to_int(&curr);

            match it.peek() {
                Some(next) if curr_int < Solution::to_int(next) => {
                    total = total - curr_int + Solution::to_int(next);
                    it.next();
                }
                // `Some(next) if` is not exhaustive
                _ => total += curr_int,
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3)
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58)
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994)
    }
}
