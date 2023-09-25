//给你一个 无重叠的 ，按照区间起始端点排序的区间列表。 
//
// 在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。 
//
// 
//
// 示例 1： 
//
// 
//输入：intervals = [[1,3],[6,9]], newInterval = [2,5]
//输出：[[1,5],[6,9]]
// 
//
// 示例 2： 
//
// 
//输入：intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//输出：[[1,2],[3,10],[12,16]]
//解释：这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。 
//
// 示例 3： 
//
// 
//输入：intervals = [], newInterval = [5,7]
//输出：[[5,7]]
// 
//
// 示例 4： 
//
// 
//输入：intervals = [[1,5]], newInterval = [2,3]
//输出：[[1,5]]
// 
//
// 示例 5： 
//
// 
//输入：intervals = [[1,5]], newInterval = [2,7]
//输出：[[1,7]]
// 
//
// 
//
// 提示： 
//
// 
// 0 <= intervals.length <= 10⁴ 
// intervals[i].length == 2 
// 0 <= intervals[i][0] <= intervals[i][1] <= 10⁵ 
// intervals 根据 intervals[i][0] 按 升序 排列 
// newInterval.length == 2 
// 0 <= newInterval[0] <= newInterval[1] <= 10⁵ 
// 
//
// Related Topics 数组 👍 822 👎 0


#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::{max, min};

impl Solution {
    pub fn insert0(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 { return vec![new_interval]; }
        let mut r = vec![];
        let mut new_left = -1;
        let mut new_right = -1;
        let mut finish = false;
        let mut last = vec![];
        for (i, v) in intervals.into_iter().enumerate() {
            if finish {
                r.push(v.clone());
            } else if v[1] < new_interval[0] {
                r.push(v.clone());
            } else if new_left == -1 {
                new_left = min(v[0], new_interval[0]);
                new_right = max(v[1], new_interval[1]);
            } else if v[0] > new_right {
                r.push(vec![new_left, new_right]);
                r.push(vec![v[0], v[1]]);
                finish = true;
            } else {
                new_right = max(new_right, v[1]);
            }
            last = v.clone();
        }
        if !finish && new_left != -1 {
            r.push(vec![new_left, new_right])
        }
        if !finish && new_left == -1 {
            r.push(new_interval);
        }
        r
    }

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut index = 0;
        let mut new = new_interval.clone();
        while index < intervals.len() && intervals[index][1] < new_interval[0] {
            res.push(intervals[index].clone());
            index += 1;
        }

        while index < intervals.len() && intervals[index][0] <= new_interval[1] {
            new[0] = min(intervals[index][0], new[0]);
            new[1] = max(intervals[index][1], new[1]);
            index += 1;
        }

        res.push(new);
        while index < intervals.len() {
            res.push(intervals[index].clone());
            index+=1;
        }
        return res;
        // unimplemented!()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn f1() {
    let f = Solution::insert(vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ], vec![4, 8]);
    println!("{:?}", f);
    let f = Solution::insert(vec![
        vec![1, 5],
    ], vec![6, 8]);
    println!("{:?}", f);
}