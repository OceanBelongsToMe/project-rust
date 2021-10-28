/// 斐波拉序列
pub mod fibonacci {
    /// 暴力递归
    pub fn fib(n: usize) -> i128 {
        if n == 1 || n == 2 {
            return 1;
        }
        fib(n - 1) + fib(n - 2)
    }
    /// 备忘录递归
    pub mod memo {
        pub fn fib_memo(n: usize) -> i128 {
            if n < 1 {
                return n as i128;
            }
            let mut memo: Vec<i128> = vec![0; n + 1];
            helper(&mut memo, n)
        }
        fn helper(memo: &mut Vec<i128>, n: usize) -> i128 {
            if n == 1 || n == 2 {
                return 1;
            }
            if memo[n] != 0 {
                return memo[n];
            }
            helper(memo, n - 1) + helper(memo, n - 2)
        }
    }

    /// DP数组的递归解法
    pub fn fib_dp(n: usize) -> i128 {
        let mut dp: Vec<i128> = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..n + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
    /// DP Smaller Space递归算法
    /// ```
    /// let number = fib_small_space_dp(n);
    /// ```
    pub fn fib_small_space_dp(n: usize) -> i128 {
        if n == 1 || n == 2 {
            return 1;
        }
        let mut prev: i128 = 1;
        let mut curr = prev;
        for _ in 3..n + 1 {
            let sum = prev + curr;
            prev = curr;
            curr = sum;
        }
        curr
    }
}
