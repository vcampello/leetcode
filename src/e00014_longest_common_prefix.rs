/// https://leetcode.com/problems/longest-common-prefix/
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: Vec<char> = Vec::new();

        // loop through the all chars of the first word
        for (char_idx, c) in strs.first().unwrap_or(&String::new()).chars().enumerate() {
            let is_same = strs
                .iter()
                // all: test if every element matches the predicate
                // matches!: condensed version of `match bool`
                // - see: https://rust-lang.github.io/rust-clippy/master/index.html#match_like_matches_macro
                .all(|word| matches!(word.chars().nth(char_idx), Some(other_c) if  other_c ==c));

            // store the prefix or stop processing
            match is_same {
                true => prefix.push(c),
                false => break,
            }
        }

        prefix.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(result, "fl");
    }

    #[test]
    fn case_2() {
        let result = Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]);
        assert_eq!(result, "");
    }

    #[test]
    fn case_single_char() {
        let result = Solution::longest_common_prefix(vec!["a".to_string()]);
        assert_eq!(result, "a");
    }
}
