/// https://leetcode.com/problems/longest-palindromic-substring/
pub struct Solution;

// TODO: document original solution and the new one + start a notes repo
// tldr:
// - original brute forced and was too slow O(n^2)
// - new: Manacher's algorithm O(n)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        const SEP: &str = "|";
        // TODO: I think I can remove the edge sep
        let prime = format!(
            "{SEP}{}{SEP}",
            s.chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(SEP)
        );

        let ss = s
            .chars()
            .fold(String::new(), |acc, cur| acc + "|" + &cur.to_string());
        println!("{ss}");

        let chars = prime.chars().collect::<Vec<char>>();

        let mut center = 0;
        // palindrome length
        let mut p_len = vec![0; prime.len() * 2 + 1];
        let mut r_bound = 0;

        // crawl forward
        for i in 0..prime.len() {
            if i < r_bound {
                let mirror = 2 * center - i;
                p_len[i] = (r_bound - i).min(p_len[mirror]);
            }

            // explore
            while {
                let left = i as isize - p_len[i] as isize - 1;
                let right = i + p_len[i] + 1;
                left >= 0 && right < prime.len() && chars[left as usize] == chars[right]
            } {
                p_len[i] += 1;
            }

            // expand
            if p_len[i] + 1 > r_bound {
                center = i;
                r_bound = i + p_len[i];
            }
        }

        let longest = p_len.iter().max().unwrap_or(&0);
        let center_idx = p_len.iter().position(|n| n == longest).unwrap();
        let left = center_idx - longest;
        let right = center_idx + longest;

        prime.get(left..right).unwrap().to_string().replace("|", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab"); // aba is also valid
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn case_single_char() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }
    #[test]
    fn case_empty() {
        assert_eq!(Solution::longest_palindrome("".to_string()), "");
    }

    #[test]
    // #[ignore = "reason"]
    fn case_long() {
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        let expected = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }
}
