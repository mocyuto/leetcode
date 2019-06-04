use std::collections::HashMap;

struct Solution200;
struct Coord {
    x: i32,
    y: i32,
}
impl Solution200 {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islandMap: HashMap<Coord,i32> = HashMap::new();
        let mut result = 0;
        for x in grid {
            for y in x {
                if y == '1' {

                    // 自分の更新
                    // if 自分のIDがある then 自分のグループIDは更新しない
                    // if 隣がグループに所属しているか then そのグループID
                    // else 新しいグループID as result + 1

                    // 下のIDの更新
                    // if 今の位置の下が1 then 下も同じグループID
                }
            }
        }
        result
    }
}

pub fn main(){
    assert_eq!(Solution200::num_islands(
        vec![vec!['1','0','0'],vec!['0','1','0'],vec!['0','0','1']]), 3);
    assert_eq!(Solution200::num_islands(
        vec![vec!['1','1','0'],vec!['1','0','1'],vec!['0','1','1']]), 2);
    assert_eq!(Solution200::num_islands(
        vec![vec!['1','1','0'],vec!['1','0','0'],vec!['0','0','1']]), 2);
}