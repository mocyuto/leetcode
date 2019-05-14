struct Solution13;
impl Solution13 {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total: i32 = 0;
        let mut pre: i32 = 0;
        let mut stock: i32 = 0;
        for (i,c) in s.as_str().chars().enumerate() {
            let now = to_int(c);
            if i == 0 {
                stock += now;
            } else if pre < now  {
                total += now - stock;
                stock = 0;
            } else if pre > now {
                total += stock;
                stock = now;
            } else {
                stock += now;
            }
            pre = now;
        }
        total += stock;
        total
    }
}
fn to_int(c: char) ->i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    }
}


fn main() {
    println!("{}", Solution13::roman_to_int("III".to_string())); // 3
    println!("{}", Solution13::roman_to_int("IX".to_string())); // 9
    println!("{}", Solution13::roman_to_int("LVIII".to_string())); //58
    println!("{}", Solution13::roman_to_int("MCMXCIV".to_string())); //1994
}
