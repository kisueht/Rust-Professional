/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // TODO: Implement the logic to find the intersection of two arrays
    // intersection_hash(nums1, nums2)
    intersection_2_pointer(nums1, nums2)
}

fn intersection_hash(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let hash_set: HashSet<_> = nums1.into_iter().collect();
    let mut result_set = HashSet::new();

    for item in nums2 {
        if hash_set.contains(&item) && !result_set.contains(&item) {
            result_set.insert(item);
        }
    }
    result_set.into_iter().collect() 
}

fn intersection_2_pointer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort_unstable();
    nums2.sort_unstable();

    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();

    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                let temp_v = nums1[i];
                result.push(temp_v);

                while i < nums1.len() && nums1[i] == temp_v {
                    i += 1;
                }

                while j < nums2.len() && nums2[j] == temp_v {
                    j += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
