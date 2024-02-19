pub fn roman_to_integer(s: String) -> i32 {
    let mut total = 0;
    // Use something that'll never be less than the given values
    let mut prev_val = 9999;
    for c in s.chars() {
        let val = match c {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            'I' => 1,
            _ => panic!("Invalid input!"),
        };
        if prev_val < val {
            // IV, IX, etc
            total -= 2 * prev_val;
        }
        total += val;
        prev_val = val;
    }
    total
}
