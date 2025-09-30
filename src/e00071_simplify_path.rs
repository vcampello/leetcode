/// https://leetcode.com/problems/simplify-path/description/
pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let out: Vec<_> = path
            // clean boundaries
            .split("/")
            // parse components
            .fold(Vec::new(), |mut acc: Vec<&str>, cur| {
                match cur {
                    // remove the previous component
                    ".." => {
                        acc.pop();
                    }
                    // ignore current dir or empty string (caused by 'split')
                    "." | "" => (),
                    // add to path
                    _ => acc.push(cur),
                }
                acc
            });

        // prefix path and join
        match out.len() {
            0 => "/".to_string(),
            _ => out.iter().map(|comp| "/".to_string() + comp).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::simplify_path("/home/user/Documents/../Pictures".to_string()),
            "/home/user/Pictures"
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    }

    #[test]
    fn case_5() {
        assert_eq!(
            Solution::simplify_path("/.../a/../b/c/../d/./".to_string()),
            "/.../b/d"
        );
    }
}
