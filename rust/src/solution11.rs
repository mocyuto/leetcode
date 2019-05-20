use std::cmp::{min, max};

struct Solution11;
impl Solution11 {
    pub fn max_area(height: Vec<i32>) -> i32 {
        (0..=height.len()-1)
            .skip(1)
            .fold(0,|ma, i| get_max(&height, ma, i ))
    }
}

// 後ろがyのときの面積最大値を求める
fn get_max(height: &Vec<i32>, ma:i32, y: usize) -> i32{
    (0..=height.len()-1)
        .take(y )
        .fold(ma, |m, x| max(m, calc_area(height,x ,y )))
}

// x < y
fn calc_area(height: &Vec<i32>, x: usize, y: usize) -> i32 {
    (y-x) as i32 * min(height[x], height[y])
}

pub fn main(){
    assert_eq!(Solution11::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
}