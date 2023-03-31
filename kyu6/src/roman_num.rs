pub fn num_as_roman(num: i32) -> String {
    const SYMBOLS: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    const VALUES: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    if num <= 0 {
        return "i can't do it".to_string();
    }
    let mut num = num;
    SYMBOLS.iter().zip(VALUES.iter()).fold(String::new(), |mut res, (&symbol, &value)| {
        res.push_str(&symbol.repeat((num / value) as usize));
        num %= value;
        res
    })
}