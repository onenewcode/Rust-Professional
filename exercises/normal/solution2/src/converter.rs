const DIGITS_TABLE: &[u8] = b"0123456789abcdef";

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析得到输入的数字
    let (digits, base) = num_str.split_once('(').unwrap();
    let base = base[..base.len() - 1].parse::<u32>().unwrap();
    let mut num = u32::from_str_radix(digits, base).unwrap();

    let mut out_digits = vec![];
    loop {
        out_digits.push(DIGITS_TABLE[(num % to_base) as usize] as char);
        num /= to_base;
        if num == 0 {
            break;
        }
    }

    out_digits.into_iter().rev().collect()
}
