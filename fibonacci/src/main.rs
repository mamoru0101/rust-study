fn main() {
    let n = 21;
    for i in 0..n + 1 {
        println!("第{}項 {}", i, fibonacci(i));
    }
}

fn fibonacci(n: u64) -> u64 {
    let mut f: u64 = 0;
    let mut prev_value: u64 = 1;

    for _ in 0..n {
        let _prev = f;
        f = f + prev_value;
        prev_value = _prev;
    }

    return f
}
