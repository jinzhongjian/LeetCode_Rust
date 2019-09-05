# 题目 [1. 两数之和](https://leetcode-cn.com/problems/two-sum/)
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素

示例:

```
给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
```

#### 方法一：暴力法

便利每个元素，找到元素相加等于target，返回索引数组
```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}
```
#### 方法二：哈希表

把数组中元素放到插入到表中的同时  hashmap<值, 索引>，反过来查看是否存在当前元素对应的目标元素 ，存在立即返回
```
pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let y = target - num;
        if map.contains_key(&y){
            let i = map.get(&y);
            return vec![i as i32, index as i32];
        }
        map.insert(num, index);
    }
    return vec![];
}
```
#### 问题

 返回数组是存在问题

i as i32

note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
