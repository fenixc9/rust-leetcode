//给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持
//一致 。然后返回 nums 中唯一元素的个数。 
//
// 考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过： 
//
// 
// 更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不
//重要。 
// 返回 k 。 
// 
//
// 判题标准: 
//
// 系统会用下面的代码来测试你的题解: 
//
// 
//int[] nums = [...]; // 输入数组
//int[] expectedNums = [...]; // 长度正确的期望答案
//
//int k = removeDuplicates(nums); // 调用
//
//assert k == expectedNums.length;
//for (int i = 0; i < k; i++) {
//    assert nums[i] == expectedNums[i];
//} 
//
// 如果所有断言都通过，那么您的题解将被 通过。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,1,2]
//输出：2, nums = [1,2,_]
//解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
// 
//
// 示例 2： 
//
// 
//输入：nums = [0,0,1,1,1,2,2,3,3,4]
//输出：5, nums = [0,1,2,3,4]
//解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 3 * 10⁴ 
// -10⁴ <= nums[i] <= 10⁴ 
// nums 已按 非严格递增 排列 
// 
//
// Related Topics 数组 双指针 👍 3331 👎 0
/**
      s   f
1 2 3 4 5 4 5


 */
#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn remove_duplicates0(nums: &mut Vec<i32>) -> i32 {
        let mut j = 0;
        // for (i, v) in nums.iter_mut().enumerate() {
        //     if nums[i] != nums[j] {
        //         j += 1;
        //         *v = nums[i];
        //     }
        // }
        for i in 0..nums.len() {
            if nums[i] != nums[j] {
                j += 1;
                nums[j] = nums[i];
            }
        }
        j as i32 + 1
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        for x in 0..nums.len() {
            if idx < 1 || nums[idx - 1] != nums[x] {
                nums[idx] = nums[x];
                idx += 1;
            }
        }
        idx as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn f1() {
    let mut vec1 = vec![1, 2, 3, 3, 4, 5];
    println!("{}", Solution::remove_duplicates(&mut vec1));
}