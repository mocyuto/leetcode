use std::collections::HashSet;

struct Solution15;
impl Solution15 {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut v = nums.clone();
        v.sort();
        let l = v.len()-1;
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut before1 = 0;
        let mut before2 = 0;
        println!("{:?}", v);
        for i in 0..=l {
            let x = v[i];
            if x == before1 && x == before2 {
                if i >= 2 && x == 0 {
                    set.insert(vec![0,0,0]);
                }
                continue
            }
            let mut start = i+1;
            let mut end = l;

            while start < end {
                let total = v[start] + v[end] + x;
                if total < 0 {
                    start += 1;
                } else if total > 0 {
                    end -= 1;
                } else {
                    set.insert(vec![x,v[start],v[end]]);
                    start += 1;
                    end -= 1;
                }
            }
            before2 = before1.clone();
            before1 = x.clone();
        }
        set.into_iter().collect()
    }
}

pub fn main(){
    let v2: Vec<Vec<i32>> = Vec::new();
    assert_eq!(
        Solution15::three_sum(vec![0, 0, 0]),
        vec![vec![0, 0, 0]]
    );
    assert_eq!(
        Solution15::three_sum(vec![0]),
        v2
    );
    assert_eq!(
        Solution15::three_sum(vec![0,1,1]),
        v2
    );
    assert_eq!(
        Solution15::three_sum(vec![0,0,1,1]),
        v2
    );
    assert_eq!(
        Solution15::three_sum(vec![3,0,-2,-1,1,2]),
        vec![vec![-1, 0, 1], vec![-2, -1, 3], vec![-2, 0, 2]]
    );
    assert_eq!(
        Solution15::three_sum(vec![0,0,0,1,1]),
        vec![vec![0,0,0]]
    );
    assert_eq!(
        Solution15::three_sum(vec![-2,0,1,1,2]),
        vec![vec![-2,0,2],vec![-2,1,1]]
    );
    assert_eq!(
        Solution15::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 0, 1],vec![-1, -1, 2]]
    );
    assert_eq!(Solution15::three_sum(vec![]), v2);
}