pub fn goldbach_conjecture() -> u64 {
    let mut count = 0;
    let mut sum = 0;
    let mut number = 9;

    while count < 2 {
        if is_odd_composite(number) && !can_be_expressed(number) {
            sum += number;
            count += 1;
        }
        number += 2; // 只考虑奇数,减少循环次数
    }
    sum
}
// 判断是否为素数。
fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n == 1 || n & 1 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        // 跳过2和3的倍数，减少循环次数
        i += 6;
    }
    true
}
//
fn is_odd_composite(n: u64) -> bool {
    n & 1 == 1 && !is_prime(n)
}
// 判断是否能被组合
fn can_be_expressed(n: u64) -> bool {
    for i in 1.. {
        let square_two = 2 * i * i;
        if square_two > n {
            break;
        }
        if is_prime(n - square_two) {
            return true;
        }
    }
    false
}