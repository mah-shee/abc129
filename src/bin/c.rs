#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let MOD = 1_000_000_007;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    let mut step = vec![true; n + 1];
    for i in 0..m {
        step[a[i]] = false;
    }
    for now in 0..n {
        for next in now + 1..=(min(n, now + 2)) {
            if step[next] {
                dp[next] += dp[now];
                dp[next] %= MOD;
            }
        }
    }
    println!("{}", dp[n]);
}
