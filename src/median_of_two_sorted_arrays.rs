use std::cmp::{max, min};

/// Solution for https://leetcode.com/problems/median-of-two-sorted-arrays/
struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // find the shorter
        let (shorter, longer) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let m = shorter.len();
        let n = longer.len();
        let total_cnt_is_even = (m + n) % 2 == 0;
        let (mut start, mut end) = (0, m);

        loop {
            let i = (end + start) / 2;
            let j = (m + n + 1) / 2 - i;

            let max_l_x = if i == 0 { i32::MIN } else { shorter[i - 1] };
            let min_r_x = if i == m { i32::MAX } else { shorter[i] };

            let max_l_y = if j == 0 { i32::MIN } else { longer[j - 1] };
            let min_r_y = if j == n { i32::MAX } else { longer[j] };

            if max_l_x <= min_r_y && max_l_y <= min_r_x {
                // found the place
                if total_cnt_is_even {
                    return f64::from(max(max_l_x, max_l_y) + min(min_r_y, min_r_x)) / 2_f64;
                }
                return max(max_l_x, max_l_y).into();
            }
            if max_l_x > min_r_y {
                // move towards left
                end = i - 1;
            } else {
                start = i + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2_f64);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }
    #[test]
    fn test5() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
    #[test]
    fn test6() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![23, 26, 31, 35], vec![3, 5, 7, 9, 11, 16]),
            13.5
        );
    }
}
