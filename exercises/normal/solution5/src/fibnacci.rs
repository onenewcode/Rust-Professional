use std::iter;

fn fib_iter() -> impl Iterator<Item = u32> {
    let mut a = 0;
    let mut b = 1;
    iter::from_fn(move || {
        let c = a;
        a = b;
        b = a + c;
        Some(c)
    })
}

pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    fib_iter()
        .take_while(|&x| x < threshold)
        .filter(|&x| x % 2 == 1)
        .sum()
}