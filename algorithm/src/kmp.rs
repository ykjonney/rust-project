// KMP (Knuth-Morris-Pratt) 字符串匹配实现
// 提供两个函数：
// - `build_next(p: &str) -> Vec<i32>`: 计算模式串 p 的部分匹配表（又称 failure function 或 next 数组）
// - `kmp_search(text: &str, p: &str) -> Option<usize>`: 在 text 中查找模式 p 的第一次出现位置（以字符索引返回），不存在则返回 `None`。
// 算法复杂度为 O(n + m)（n = text 长度，m = p 长度）。
// 性能注意：本实现中使用了 `chars().nth(i)` 来按索引访问字符，Rust 的 `chars().nth` 是 O(i) 的（需要从头迭代到第 i 个字符），
// 因此对于较长字符串会产生 O(n^2) 的表现。建议的优化是先把字符串转换为 `Vec<char>` 或按 bytes 处理以获得 O(1) 的索引访问。

/// 生成模式串 `p` 的 next 数组（部分匹配表）
/// 返回长度为 `p.len()` 的 `Vec<i32>`，其中 `next[i]` 表示 `p[0..=i]` 的最长相等的前缀后缀的长度。
fn build_next(p: &[char]) -> Vec<i32> {
    // next 数组初始化为全 0，长度为模式串长度
    let mut next = vec![0; p.len()];

    // k: 当前匹配的前缀长度（也是 p 的索引，用于比较 p[i] 与 p[k]）
    // i: 当前正在计算 next[i] 的位置，从 1 开始（next[0] 总是 0）
    let mut k = 0;
    let mut i = 1;

    while i < p.len() {
        // 直接按索引比较，O(1)
        if p[i] == p[k] {
            k += 1;
            next[i] = k as i32;
            i += 1;
        } else {
            if k > 0 {
                // 回退到上一个可能的前缀长度
                k = next[k - 1] as usize;
            } else {
                next[i] = 0;
                i += 1;
            }
        }
    }

    next
}

/// 在 `text` 中查找模式 `p`，返回第一次匹配的起始字符索引（若存在）
fn kmp_search(text: &str, p: &str) -> Option<usize> {
    // 先将字符串转换为 Vec<char>，以便进行 O(1) 的索引访问
    let t: Vec<char> = text.chars().collect();
    let pat: Vec<char> = p.chars().collect();

    if pat.is_empty() {
        return Some(0);
    }

    let next = build_next(&pat);
    print!("next array: {:?}\n", next);
    // i: text 的当前索引，j: p 的当前索引
    let mut i = 0;
    let mut j = 0;

    while i < t.len() && j < pat.len() {
        if t[i] == pat[j] {
            i += 1;
            j += 1;
        } else {
            if j != 0 {
                j = next[j - 1] as usize;
            } else {
                i += 1;
            }
        }
    }

    if j == pat.len() {
        Some(i - j)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp() {
        let text = "BBC ABCDAB ABCDABCDABDE";
        let p = "ABCDABD";
        // 已知模式在 text 中第一次出现的起始位置为 15（以字符计）
        assert_eq!(kmp_search(text, p), Some(15));
    }
}