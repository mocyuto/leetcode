struct Solution12;
impl Solution12 {
    pub fn int_to_roman(num: i32) -> String {
        recurse(num, "".to_string())
    }
}

fn recurse(n: i32, s: String) -> String {
    if n == 0 {
        return s
    } else {
        let (n2,s2) = to_char(n, s);
        recurse(n2,s2)
    }
}

fn to_char(num: i32, s: String) -> (i32, String) {
    let mut res = s.clone();
    if num / 1000 > 0 {
        res += &to_char2(num / 1000, "M".to_string(), "".to_string(), "".to_string());
        (num - (num / 1000 * 1000 ), res)
    } else if num / 100 > 0 {
        res += &to_char2(num/100, "C".to_string(), "D".to_string(), "M".to_string());
        (num - (num / 100 * 100), res)
    } else if num / 10 > 0 {
        res += &to_char2(num/10, "X".to_string(), "L".to_string(), "C".to_string());
        (num - (num / 10 * 10), res)
    } else {
        res += &to_char2(num, "I".to_string(), "V".to_string(), "X".to_string());
        (0, res)
    }
}
// num: 一番上の桁
// roman1
// roman5
// roman10
fn to_char2(num: i32, roman1: String, roman5 :String, roman10: String) -> String {
    match num {
        1|2|3 => roman1.repeat(num as usize),
        4 => roman1 + &roman5,
        5 => roman5,
        6|7|8=> roman5 + &roman1.repeat((num-5) as usize),
        9 => roman1 + &roman10,
        _ => "".to_string()
    }
}
pub fn main(){
    println!("{}", Solution12::int_to_roman(3)); // III
    println!("{}", Solution12::int_to_roman(4)); // IV
    println!("{}", Solution12::int_to_roman(9)); // IX
    println!("{}", Solution12::int_to_roman(58)); // LVIII
    println!("{}", Solution12::int_to_roman(1994)); // MCMXCIV
}