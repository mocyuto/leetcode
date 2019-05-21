struct Solution20;
impl Solution20 {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '('|'{'|'[' => stack.push(c),
                ')'|'}'|']' => if !validate(&mut stack, c) {return false},
                _ => return false,
            };
        }
        stack.is_empty()
    }
}

fn validate(stack: &mut Vec<char>, c: char) -> bool{
    let top_op = stack.pop();
    if top_op.is_none() {
        return false;
    }
    match top_op.unwrap() {
        '(' => ')' == c,
        '{' => '}' == c,
        '[' => ']' == c,
        _ => false,
    }
}

pub fn main(){
    assert_eq!(Solution20::is_valid("[".to_string()),false);
    assert_eq!(Solution20::is_valid("()".to_string()),true);
    assert_eq!(Solution20::is_valid("()[]{}".to_string()),true);
    assert_eq!(Solution20::is_valid("(]".to_string()),false);
    assert_eq!(Solution20::is_valid("([)]".to_string()),false);
}