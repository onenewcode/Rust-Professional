
// 生成素数表，用于查询信息，同时用于最后一个素数进行平方，方便跳过大量的值
fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as u64) {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(index, _)| index as u64)
        .collect()
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number = number;
    let mut max_prime = 0;
    while number % 2 == 0 {
        max_prime = 2;
        number /= 2;
    }
    while number % 3 == 0 {
        max_prime = 3;
        number /= 3;
    }

    // 确定我们生成表的范围
    // let limit = (number.isqrt() as u64).min(1_000_000);
    let limit = ((number as f64).sqrt() as u64).min(1_000_000);
    let small_primes = sieve_of_eratosthenes(limit);

    // 检查素数因子
    for &prime in &small_primes {
        while number % prime as u128 == 0 {
            max_prime = prime as u128;
            number /= prime as u128;
        }
    }
    let mut factor = 1_000_000;
    // 超出表范围，使用更复杂的方法查询素数因子
    // 判断是否超表的平方，超出时，直接让新的值为最后一个素数的平方，否则为最后一个素数
    // if number.isqrt() > *small_primes.last().unwrap() as u128 {
    if (number as f64).sqrt() as u128 > *small_primes.last().unwrap() as u128 {
        factor = small_primes.last().unwrap().pow(2) as u128;
    } else {
        factor = *small_primes.last().unwrap() as u128;
    }

    while factor * factor <= number {
        while number % factor == 0 {
            max_prime = factor;
            number /= factor;
        }
        while number % (factor + 2) == 0 {
            max_prime = factor + 2;
            number /= factor + 2;
        }
        factor += 6;
    }
    if number > 2 {
        max_prime = number;
    }
    max_prime
}