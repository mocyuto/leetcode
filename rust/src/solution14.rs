
struct Solution14;
impl Solution14 {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new()
        }
        let size = strs[0].len();
        let chars_list: Vec<Vec<char>> = strs
            .iter()
            .cloned()
            .map(|x| x.chars().collect())
            .collect();

        let mut res: String = String::from("");
        for i in 0..=size {
            if  size  < i + 1 {
                break
            }
            let a = chars_list[0].iter().nth(i);
            if chars_list.iter()
                .cloned()
                .any(|ref x| x.len() < i + 1 || x[i] != a.unwrap().clone()) {
                break
            }
            res.push(a.unwrap().clone());
        }
        res
    }
}

pub fn main() {
    println!("{}", Solution14::longest_common_prefix(change(vec!["flower","flow","flight"]))); // fl
    println!("{}", Solution14::longest_common_prefix(change(vec!["dog","racecar","car"]))); //
    println!("{}", Solution14::longest_common_prefix(change(vec![]))); //
    println!("{}", Solution14::longest_common_prefix(change(vec![""]))); //
    println!("{}", Solution14::longest_common_prefix(change(vec!["a", ""]))); //
}

fn change(ss: Vec<&str>) -> Vec<String> {
    ss.iter()
        .map(|&x| String::from(x))
        .collect()
}