/// https://leetcode.com/problems/valid-sudoku/
pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 1. I'll only handle 9x9 for this problem
        // 2. Anything lager will cause it to panic
        // 3. Fixed size stack allocated multi-dimensional arrays for readability
        // - a 1d array won't make any difference here

        // [row][num] = lock row and number
        let mut row_num = [[false; 9]; 9];
        // [col][num] = lock column and number
        let mut col_num = [[false; 9]; 9];
        // [box_row][box_col][num] = lock box number
        let mut area_x_y_num = [[[false; 9]; 3]; 3];

        for (row, row_chars) in board.iter().enumerate() {
            for (col, &ch) in row_chars.iter().enumerate() {
                // Parse as a digit or skip
                let num_idx = match ch.to_digit(10) {
                    Some(num) => (num - 1) as usize, // subtract one to create an index
                    None => continue,
                };

                // calculate which sub area
                let area_row = row / 3;
                let area_col = col / 3;

                // check if it's been seen already
                if row_num[row][num_idx]
                    || col_num[col][num_idx]
                    || area_x_y_num[area_row][area_col][num_idx]
                {
                    return false;
                }

                // mark as seen
                row_num[row][num_idx] = true;
                col_num[col][num_idx] = true;
                area_x_y_num[area_row][area_col][num_idx] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn case_2() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!Solution::is_valid_sudoku(board));
    }

    #[test]
    fn case_3() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];

        assert!(!Solution::is_valid_sudoku(board));
    }
}
