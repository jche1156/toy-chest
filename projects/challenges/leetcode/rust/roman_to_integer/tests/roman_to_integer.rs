use roman_to_integer::roman_to_integer;

#[test]
fn easy() {
    let roman_numerals: String = "III".to_string();
    let ans: i32 = 3;
    assert_eq!(roman_to_integer(roman_numerals), ans);
}

#[test]
fn medium() {
    let roman_numerals: String = "LVIII".to_string();
    let ans: i32 = 58;
    assert_eq!(roman_to_integer(roman_numerals), ans);
}

#[test]
fn prefix_subtract() {
    let roman_numerals: String = "IV".to_string();
    let ans: i32 = 4;
    assert_eq!(roman_to_integer(roman_numerals), ans);
}

#[test]
fn hard() {
    let roman_numerals: String = "MCMXCIV".to_string();
    let ans: i32 = 1994;
    assert_eq!(roman_to_integer(roman_numerals), ans);
}
