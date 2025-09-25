pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: Vec<char> = Vec::new();

        // loop through the all chars of the first word
        for (char_idx, c) in strs[0].chars().enumerate() {
            let mut is_same = false;

            for word in strs.iter() {
                match word.chars().nth(char_idx) {
                    Some(c_to_cmp) if c_to_cmp == c => is_same = true,
                    _ => {
                        is_same = false;
                        break;
                    }
                }
            }

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
