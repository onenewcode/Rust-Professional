pub fn dp_rec_mc(mut amount: u32) -> u32 {
    [1, 2, 5, 10, 20, 30, 50, 100]
        .iter()
        .rev()
        .fold(0, |acc, coin| {
            let num = amount / coin;
            amount %= coin;
            acc + num
        })
}