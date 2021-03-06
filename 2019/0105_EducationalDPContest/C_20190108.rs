#![allow(non_snake_case, unused_imports)]

// Input
macro_rules! input { (source = $s:expr, $($r:tt)*) => { let mut iter = $s.split_whitespace(); let mut next = || { iter.next().unwrap() }; input_inner!{next, $($r)*} }; ($($r:tt)*) => { let stdin = std::io::stdin(); let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock())); let mut next = move || -> String{ bytes.by_ref().map(|r|r.unwrap() as char).skip_while(|c|c.is_whitespace()).take_while(|c|!c.is_whitespace()).collect() }; input_inner!{next, $($r)*} }; }
macro_rules! input_inner { ($next:expr) => {}; ($next:expr, ) => {}; ($next:expr, $var:ident : $t:tt $($r:tt)*) => { let $var = read_value!($next, $t); input_inner!{$next $($r)*} };}
macro_rules! read_value { ($next:expr, ( $($t:tt),* )) => { ( $(read_value!($next, $t)),* ) }; ($next:expr, [ $t:tt ; $len:expr ]) => { (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>() }; ($next:expr, chars) => { read_value!($next, String).chars().collect::<Vec<char>>() }; ($next:expr, bytes) => { read_value!($next, String).into_bytes() }; ($next:expr, usize1) => { read_value!($next, usize) - 1 }; ($next:expr, $t:ty) => { $next().parse::<$t>().expect("Parse error") }; }

// Module
use std::cmp::{min,max};
use std::collections::{HashSet,HashMap,BTreeMap,VecDeque};

// Functions

// Main
fn main(){
    input!{
        N: usize,
        ABC: [(i64,i64,i64); N],
    }
    // dp[i][j] : i日目にjを選んだ場合の最大幸福度
    let mut dp = vec![vec![0; 3]; N];
    // dp table init
    dp[0][0] = ABC[0].0;
    dp[0][1] = ABC[0].1;
    dp[0][2] = ABC[0].2;
    // dp
    for i in 1..N{
        let (a,b,c) = ABC[i];
        dp[i][0] = max(dp[i-1][1] + a, dp[i-1][2] + a);
        dp[i][1] = max(dp[i-1][0] + b, dp[i-1][2] + b);
        dp[i][2] = max(dp[i-1][0] + c, dp[i-1][1] + c);
    }
    let mut ans:i64 = 0;
    for i in 0..3{
        ans = max(ans, dp[N-1][i]);
    }
    println!("{}", ans);
}