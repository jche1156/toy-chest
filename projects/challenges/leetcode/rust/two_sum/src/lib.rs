use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // leetcode.com/problems/two-sum
    use std::collections::HashMap;

    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        if let Some(j) = m.get(&(target - *v)) {
            return vec![*j, i as i32];
        }
        m.insert(*v, i as i32);
    }
    vec![]
}
