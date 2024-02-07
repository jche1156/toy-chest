use two_sum::two_sum;

#[test]
fn easy() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let ans: Vec<i32> = vec![0, 1];
    assert_eq!(two_sum(nums, target), ans);
}

#[test]
fn medium() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6;
    let ans: Vec<i32> = vec![1, 2];
    assert_eq!(two_sum(nums, target), ans);
}

#[test]
fn hard() {
    let nums: Vec<i32> = vec![3, 3];
    let target: i32 = 6;
    let ans: Vec<i32> = vec![0, 1];
    assert_eq!(two_sum(nums, target), ans);
}
