fn alt_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    let mut state = -1;
    while n > 0 {
        state *= -1;
        sum += (n % 10) * state;
        n /= 10;
    }
    sum * state
}

fn main() {
    assert!(alt_sum(521) == 4, "alt_sum(521) != 4");
    assert!(alt_sum(111) == 1, "alt_sum(111) != 1");
    assert!(alt_sum(886996) == 0, "alt_sum(886996) != 0");
}
