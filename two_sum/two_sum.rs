use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let result = two_sum(nums, 9);
    println!("{}", result.len());
    if result.len() == 2{
        println!("{} , {}", result[0], result[1]);
    }
}

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
