//给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。 
//
// 不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。 
//
// 
//
// 说明： 
//
// 为什么返回数值是整数，但输出的答案是数组呢？ 
//
// 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。 
//
// 你可以想象内部操作如下: 
//
// 
//// nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
//int len = removeDuplicates(nums);
//
//// 在函数里修改输入数组对于调用者是可见的。
//// 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
//for (int i = 0; i < len; i++) {
//    print(nums[i]);
//}
// 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,1,1,2,2,3]
//输出：5, nums = [1,1,2,2,3]
//解释：函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3。 不需要考虑数组中超出新长度后面的元素。
// 
//
// 示例 2： 
//
// 
//输入：nums = [0,0,1,1,1,1,2,3,3]
//输出：7, nums = [0,0,1,1,2,3,3]
//解释：函数应返回新长度 length = 7, 并且原数组的前五个元素被修改为 0, 0, 1, 1, 2, 3, 3。不需要考虑数组中超出新长度后面的元素
//。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 3 * 10⁴ 
// -10⁴ <= nums[i] <= 10⁴ 
// nums 已按升序排列 
// 
//
// Related Topics 数组 双指针 👍 894 👎 0


#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
/**

1 2 3 3 3 4 5 6
      s   f
1 2 3 4 3 3 5 6


1 2 3 4 3 3 5 6


 */
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::process(nums, 2)
    }

    fn process(nums: &mut [i32], k: i32) -> i32 {
        let mut idx = 0;
        for x in 0..nums.len() {
            if idx < k || &nums[(idx - k) as usize] != &nums[x] {
                nums[idx as usize] = nums[x];
                idx += 1;
            }
        }
        return idx;
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn f1() {
    let mut vec1 = vec![1, 2, 3, 3, 3, 4, 5];
    let i = Solution::remove_duplicates(&mut vec1);
    println!("{}", i);
}