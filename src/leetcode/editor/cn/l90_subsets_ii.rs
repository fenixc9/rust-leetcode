//给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。 
//
// 解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。 
//
// 
// 
// 
// 
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,2,2]
//输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
// 
//
// 示例 2： 
//
// 
//输入：nums = [0]
//输出：[[],[0]]
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 10 
// -10 <= nums[i] <= 10 
// 
//
// Related Topics 位运算 数组 回溯 👍 1155 👎 0


#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut path = vec![];
        let mut vis = vec![false; nums.len()];
        let mut n = nums;
        n.sort();
        Solution::bt(&mut res, &mut path, 0, &n, &mut vis);
        res.into_iter().collect::<Vec<Vec<i32>>>()
    }

    fn bt(res: &mut HashSet<Vec<i32>>, path: &mut Vec<i32>,
          n: usize, nums: &Vec<i32>,
          vis: &mut Vec<bool>) {
        if n == nums.len() {
            res.insert(path.clone());
            return;
        }
        path.push(nums[n]);
        Self::bt(res, path, n + 1, nums, vis);
        path.remove(path.len() - 1);
        Self::bt(res, path, n + 1, nums, vis);
    }
}

//leetcode submit region end(Prohibit modification and deletion)
#[test]
fn f1() {
    let vec1 = Solution::subsets_with_dup(vec![1, 2, 2]);
    dbg!(vec1);
}