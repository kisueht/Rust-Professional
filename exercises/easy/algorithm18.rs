/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    let row_len = intervals.len();
    let col_len = intervals[0].len();

    intervals.sort_unstable();
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut index_m = 0;
    let mut index_n = 1;
    while index_n < row_len {
        let mut item_temp = Vec::new();
        let row_m = intervals[index_m].clone();
        let row_n = intervals[index_n].clone();
        if row_n[0] >= row_m[0] && row_n[0] <= row_m[col_len - 1] {
            item_temp.push(row_m[0]);
            if row_n[col_len - 1] <= row_m[col_len - 1] {
                item_temp.push(row_m[col_len - 1]);
                index_n += 1;
            } else {
                item_temp.push(row_n[col_len - 1]);
                index_m += 2;
                index_n += 2;
            }

            if !result.contains(&item_temp) {
                result.push(item_temp);
            }

            if index_m == row_len - 1 {
                result.push(intervals[row_len - 1].clone());
            }
        } else {
            item_temp = row_m;
            index_m += 1;
            index_n += 1;

            result.push(item_temp);
    
            if index_n == row_len {
                result.push(intervals[row_len - 1].clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
