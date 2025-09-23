pub struct Solution {}

impl Solution {
    const LOOKUP: [(i32, &'static str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    pub fn int_to_roman(num: i32) -> String {
        let mut remaining: i32 = num;
        let mut converted = String::new();

        while remaining > 0 {
            for roman in Solution::LOOKUP.iter() {
                if remaining >= roman.0 {
                    remaining -= roman.0;
                    converted += roman.1;
                    break;
                }
            }
        }

        converted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX")
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII")
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
