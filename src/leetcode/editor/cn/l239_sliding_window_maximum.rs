//给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位
//。 
//
// 返回 滑动窗口中的最大值 。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
//输出：[3,3,5,5,6,7]
//解释：
//滑动窗口的位置                最大值
//---------------               -----
//[1  3  -1] -3  5  3  6  7       3
// 1 [3  -1  -3] 5  3  6  7       3
// 1  3 [-1  -3  5] 3  6  7       5
// 1  3  -1 [-3  5  3] 6  7       5
// 1  3  -1  -3 [5  3  6] 7       6
// 1  3  -1  -3  5 [3  6  7]      7
// 
//
// 示例 2： 
//
// 
//输入：nums = [1], k = 1
//输出：[1]
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 10⁵ 
// -10⁴ <= nums[i] <= 10⁴ 
// 1 <= k <= nums.length 
// 
//
// Related Topics 队列 数组 滑动窗口 单调队列 堆（优先队列） 👍 2529 👎 0


#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

// #![feature(let_chains)]
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 { return nums; }
        let mut q = VecDeque::new();
        let mut result = vec![0; nums.len() + 1 - k as usize];
        let mut l = 0;
        for r in 0..nums.len() {
            while !q.is_empty() && nums[*q.front().unwrap()]<=nums[r] { q.pop_front(); }
            q.push_front(r);
            if r as i32 - k == *q.back().unwrap() as i32 { q.pop_back(); }
            if r >= (k - 1) as usize {
                result[l] = nums[*q.back().unwrap()];
                l += 1;
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn f1() {
    let vec1 = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    dbg!(vec1);
}