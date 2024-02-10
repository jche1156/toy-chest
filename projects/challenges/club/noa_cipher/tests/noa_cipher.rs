use noa_cipher::noa_cipher;

#[test]
fn easy() {
    let nums: Vec<&str> = vec![
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"
    ];
    let ans: 142;
    assert_eq!(noa_cipher(nums, target), ans);
}
