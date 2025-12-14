// 动态规划解题思路：
// 问题判断：
// 1. 是否存在重叠子问题？
// 2. 是否存在最优子结构？
// 3. 是否可以用状态转移方程描述问题？
// 4. 是否可以用表格（二维或一维）存储子问题的解？
// 总的来说，如果一个问题包含重叠子问题、最优子结构，并满足无后效性，那么它通常适合用动态规划求解。然而，我们很难从问题描述中直接提取出这些特性。因此我们通常会放宽条件，先观察问题是否适合使用回溯（穷举）解决。

// 适合用回溯解决的问题通常满足“决策树模型”，这种问题可以使用树形结构来描述，其中每一个节点代表一个决策，每一条路径代表一个决策序列。

// 换句话说，如果问题包含明确的决策概念，并且解是通过一系列决策产生的，那么它就满足决策树模型，通常可以使用回溯来解决。

// 在此基础上，动态规划问题还有一些判断的“加分项”。

// 问题包含最大（小）或最多（少）等最优化描述。
// 问题的状态能够使用一个列表、多维矩阵或树来表示，并且一个状态与其周围的状态存在递推关系。
// 相应地，也存在一些“减分项”。

// 问题的目标是找出所有可能的解决方案，而不是找出最优解。
// 问题描述中有明显的排列组合的特征，需要返回具体的多个方案。
// 如果一个问题满足决策树模型，并具有较为明显的“加分项”，我们就可以假设它是一个动态规划问题，并在求解过程中验证它。

// 问题求解步骤：
// 1. 定义状态：明确用哪些变量来表示问题的状态。
// 2. 状态转移方程：找出状态之间的递推关系
// 3. 初始化：确定初始状态的值。
// 4. 计算顺序：确定计算状态的顺序，通常是自
//    底向上或自顶向下（递归加记忆化）。
// 5. 返回结果：根据问题的要求，返回最终的结果。


// 给定一个共有n级台阶的楼梯，每次可以上1级或2级，求有多少种不同的方法可以爬到楼顶。

// 使用回溯实现

use std::{cmp, vec};

fn backtrack(choices:&[i32],n:i32,state:i32,res:&mut[i32]){
    if state == n{
        res[0] += 1;
        return;
    }
    for choice in choices{
        if choice + state > n{
            continue;
        }
        backtrack(choices,n,choice + state,res);
    }
}
pub fn climb_stairs_backtrack(n:i32)->i32{
    let choices=[1,2];
    let mut res = vec![0];
    backtrack(&choices,n,0,&mut res);
    res[0]
}

// 使用动态规划实现
pub fn climb_stairs_dp(n:usize)->i32{
    if n<=2{return n as i32;}
    let mut dp = vec![0; n+1];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n{
        dp[i] = dp[i-1] + dp[i-2];
    }
    dp[n]
}
// 使用滚动数组优化空间复杂度
pub fn climb_stairs_optimized(n:usize)->i32{
    if n<=2{return n as i32;}
    let (mut a,mut b) = (1,2);
    for _ in 3..=n{
        let tmp = b;
        b = a + b;
        a  = tmp;
    }
    b
}

// 爬楼梯最小代价问题
// 给定一个楼梯，你每步可以上 1级或2级，每个台阶有一个对应的代价cost，求达到楼顶的最小代价。
// dp[i] = min(dp[i-1],dp[i-2]) + cost[i]
pub fn min_claimb_cost(cost:&[i32]) -> i32{
    let n = cost.len()-1;
    if n <=2 {return cost[n];}
    let mut dp = vec![0; n+1];
    dp[1] = cost[1];
    dp[2] = cost[2];
    for i in 3..=n{
        dp[i] = std::cmp::min(dp[i-1],dp[i-2]) + cost[i-1];
    }
    dp[n]
}
// 带约束爬楼梯
// 给定一个共有n级台阶的楼梯，每次可以上1级、2级或3级，但不能连续两次上相同的级数，求有多少种不同的方法可以爬到楼顶。
// dp[i][1]表示到达第i级台阶，最后一步上1级台阶的方法数 dp[i][1] = dp[i-1][2]
// dp[i][2]表示到达第i级台阶，最后一步上2级台阶的方法数 dp[i][2] = dp[i-2][1] + dp[i-2][2]
// 到达第i级台阶的方法数 = dp[i][1] + dp[i][2]
pub fn climb_stairs_with_constraint(n:usize) -> i32{
    if n==1|| n==2{
        return 1;
    }
    let mut dp = vec![vec![-1;3];n+1];
    dp[1][1] = 1;
    dp[1][2] = 0;
    dp[2][2] = 1;
    dp[2][1] = 0;
    for i in 3..=n{
        dp[i][1] = dp[i-1][2];
        dp[i][2] = dp[i-2][1] + dp[i-2][2];
    }
     dp[n][1] + dp[n][2]
    }
   
// 背包问题
// 给定一个最大承重为 W 的背包和 n 个物品，每个物品有重量 wt[i] 和价值 val[i]，求在不超过背包承重的前提下，能够获得的最大价值。
// dp[i][w]表示前i个物品放入容量为w的背包所能获得的最大价值
// dp[i][w] = max(dp[i-1][w], dp[i-1][w-wt[i]] + val[i])

// 暴力搜索解法（回溯）
pub fn knapsack_dfs(wt:&[i32],val:&[i32],n:usize,w:usize)->i32{
    if n==0 || w==0{
        return 0;
    }
    if wt[n-1] as usize > w{
        return knapsack_dfs(wt,val,n-1,w);
    }
    let no = knapsack_dfs(wt,val,n-1,w);
    let yes = knapsack_dfs(wt, val, n-1, w-wt[n-1] as usize) + val[n-1];
    std::cmp::max(no,yes)
}
// 暴力搜索解法（回溯）带记忆化
pub fn knapsack_memo(wt:&[i32],val:&[i32],n:usize,w:usize,memo:&mut Vec<Vec<i32>>)->i32{
    if n==0 || w==0{return 0;}
    if memo[n][w] != -1{return memo[n][w];}
    if wt[n-1] as usize > w{
        memo[n][w] = knapsack_memo(wt, val, n-1, w, memo);
        return memo[n][w];
    }
    let no = knapsack_memo(wt,val,n-1,w,memo);
    let yes = knapsack_memo(wt, val, n-1, w-wt[n-1] as usize, memo) + val[n-1];
    memo[n][w] = std::cmp::max(no,yes);
    memo[n][w]
}
// 动态规划解法
pub fn knapsack(wt:&[i32],val:&[i32],w:usize)->i32{
    let n = wt.len();
    let mut dp =  vec![vec![0;w+1];n+1];
    for i in 1..=n{
        for j in 1..=w{
            if wt[i-1] as usize <= j{
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i-1][j-wt[i-1]as usize] + val[i-1]);
            }else{
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    dp[n][w]
}
// 01背包问题空间优化
pub fn knapsack_optimized(wt:&[i32],val:&[i32],w:usize)->i32{
    let n = wt.len();
    let mut dp = vec![0;w+1];
    for i in 1..=n{
        for j in (1..=w).rev(){
            if wt[i-1] as usize <=j{
                dp[j] = std::cmp::max(dp[j],dp[j-wt[i-1]as usize]+val[i-1]);
            }
        }
    }
    dp[w]
}
// 完全背包问题
// 与01背包的区别在于物品可以重复使用
pub fn complete_knapsack(wt:&[i32],val:&[i32],w:usize)->i32{
    let n = wt.len();
    let mut dp = vec![vec![0;w+1];n+1];
    for i in 1..=n{
        for j in 1..=w{
            if wt[i-1] as usize <= j{
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i][j-wt[i-1]as usize]+val[i-1]);
            }else{
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    dp[n][w]
}
// 完全背包问题空间优化
pub fn complete_knapsack_optimized(wt:&[i32],val:&[i32],w:usize)->i32{
    let n = wt.len();
    let mut dp = vec![0;w+1];
    for i in 1..=n{
        for j in 1..=w{
            if wt[i-1] as usize <= j{
                dp[j] = std::cmp::max(dp[j],dp[j-wt[i-1]as usize]+val[i-1]);
            }
        }
    }
    dp[w]
}

// 零钱兑换问题
// 给定不同面额的硬币 coins 和一个总金额 amount，计算组成该金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成该金额，返回 -1。
pub fn coin_change(coins:&[i32],amount:usize)->i32{
    let n = coins.len();
    let mut dp = vec![vec![i32::MAX;amount+1];n+1];
    for i in 0..=n{
        dp[i][0] =0;
    }
    for i in 1..=n{
        for j in 1..=amount{
            if coins[i-1] as usize <=j{
                dp[i][j] = std::cmp::min(dp[i-1][j],dp[i][j-coins[i-1]as usize]+1);
            }else{
                dp[i][j]= dp[i-1][j];
            }
        }
    }
    if dp[n][amount]==i32::MAX{-1}else{dp[n][amount]}
}
// 零钱兑换问题空间优化
pub fn coin_change_optimized(coins:&[i32],amount:usize)->i32{
    let n = coins.len();
    let mut dp = vec![i32::MAX;amount+1];
    dp[0] =0;
    for i in 1..=n{
        for j in 1..=amount{
            if coins[i-1] as usize <= j{
                dp[j] = std::cmp::min(dp[j],dp[j - coins[i-1] as usize]+1);
            }else{
                dp[j] = dp[j];
            }
        }
    }
    if dp[amount]==i32::MAX{-1}else{dp[amount]}
}
// 零钱兑换问题 II
// 给定不同面额的硬币 coins 和一个总金额 amount，计算组成该金额的不同组合的数量。
pub fn coin_change_ways(coins:&[i32],amount:usize)->i32{
    let n = coins.len();
    let mut dp = vec![vec![0;amount+1];n+1];
    for i in 0..=n{
        dp[i][0] =1;
    }
    for i in 1..=n{
        for a in 1..=amount{
            if coins[i-1] as usize <=a{
                dp[i][a] = dp[i-1][a] + dp[i][a-coins[i-1] as usize];
            }else{
                dp[i][a] = dp[i-1][a];
            }
        }
    }
    dp[n][amount]
}
    