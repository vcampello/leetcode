/// https://leetcode.com/problems/zigzag-conversion/description/
pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        // Calculate how many elements are required to complete a `link` spanning from 0-3 - e.g:
        // 0   4   8
        // 1 3 5 7
        // 2   6
        let link_len = (num_rows * 2) - 2;

        // A link needs to have at least 1 element
        if link_len < 1 {
            return s;
        }

        let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows];

        for (ch_idx, ch) in s.char_indices() {
            // Calculate how far an element is outside the initial column
            let link_remainder = ch_idx % link_len;

            // if the remainder < num_rows, then it  first row. Otherwise we use the remainder to
            // calculate how far it's supposed to be
            let target_idx = if link_remainder < num_rows {
                link_remainder
            } else {
                link_len - link_remainder
            };

            rows[target_idx].push(ch);
        }

        rows.iter().flatten().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }
}
