use reverse_linked_list::reverse_list;

#[test]
fn easy() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let ans: Vec<i32> = vec![0, 1];
    assert_eq!(two_sum(nums, target), ans);
}
