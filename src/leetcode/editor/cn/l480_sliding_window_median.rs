//中位数是有序序列最中间的那个数。如果序列的长度是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。 
//
// 例如： 
//
// 
// [2,3,4]，中位数是 3 
// [2,3]，中位数是 (2 + 3) / 2 = 2.5 
// 
//
// 给你一个数组 nums，有一个长度为 k 的窗口从最左端滑动到最右端。窗口中有 k 个数，每次窗口向右移动 1 位。你的任务是找出每次窗口移动后得到的新窗
//口中元素的中位数，并输出由它们组成的数组。 
//
// 
//
// 示例： 
//
// 给出 nums = [1,3,-1,-3,5,3,6,7]，以及 k = 3。 
//
// 
//窗口位置                      中位数
//---------------               -----
//[1  3  -1] -3  5  3  6  7       1
// 1 [3  -1  -3] 5  3  6  7      -1
// 1  3 [-1  -3  5] 3  6  7      -1
// 1  3  -1 [-3  5  3] 6  7       3
// 1  3  -1  -3 [5  3  6] 7       5
// 1  3  -1  -3  5 [3  6  7]      6
// 
//
// 因此，返回该滑动窗口的中位数数组 [1,-1,-1,3,5,6]。 
//
// 
//
// 提示： 
//
// 
// 你可以假设 k 始终有效，即：k 始终小于等于输入的非空数组的元素个数。 
// 与真实值误差在 10 ^ -5 以内的答案将被视作正确答案。 
// 
//
// Related Topics 数组 哈希表 滑动窗口 堆（优先队列） 👍 432 👎 0


#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BinaryHeap;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let n = nums.len() as i32;
        let mut ans = vec![0f64; (n - k + 1) as usize];
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        for i in 0..k { right.push(nums[i as usize]); }
        for _i in 0..k / 2 { left.push(right.pop().unwrap()); }
        ans[0] = Self::get_mid(&left, &right);
        for i in k..n {
            let add = nums[i as usize];
            let del = nums[(i - k) as usize];
            if &add >= right.peek().unwrap() {
                right.push(add);
            } else {
                left.push(add);
            }
            if &del >= right.peek().unwrap() {
                right.pop();
            } else {
                left.pop();
            }
            while left.len() > right.len() { right.push(left.pop().unwrap()); }
            while right.len() - left.len() > 1 { left.push(right.pop().unwrap()) }
            ans[(i - k + 1) as usize] = Self::get_mid(&left, &right);
        }
        ans
    }

    fn get_mid(left: &BinaryHeap<i32>, right: &BinaryHeap<i32>) -> f64 {
        return if left.len() == right.len() {
            f64::from(*left.peek().unwrap()) / 2f64 + f64::from(*right.peek().unwrap()) / 2f64
        } else {
            f64::from(*right.peek().unwrap())
        };
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn f1() {
    let v = vec![1, 3, -1, -3, 5, 3, 6, 7];
    dbg!(&Solution::median_sliding_window(v, 3));
}