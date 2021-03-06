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
        M: usize,
        ABC: [(usize,usize,u64); M],
    }
    let mut A = vec![vec![9999; N]; N];
    for i in 0..M{
        let (a,b,c) = ABC[i];
        A[a-1][b-1] = c;
        A[b-1][a-1] = c;
    }
    for k in 0..N{
        for i in 0..N{
            for j in 0..N{
                if i==j{
                    A[i][j] = 0;
                }else{
                    A[i][j] = min(A[i][j], A[i][k]+A[k][j]);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..M{
        let (a,b,c) = ABC[i];
        let mut flg = true;
        for j in 0..N{
            if A[a-1][j] == A[b-1][j] + c{
                flg = false;
                break
            }
        }
        if flg{
            ans += 1;
        }
    }
    println!("{}",ans);
}