// 冒泡排序
fn bubble_sort(s: &mut Vec<i32>) {
    if s.len() < 2 {
        return;
    }
    let length = s.len();
    for i in 0..length {
        for j in 0..length - i - 1 {
            if s[j] > s[j + 1] {
                swap(s, j, j + 1);
            }
        }
    }
}
//选择排序 选择最小的
fn select_sort(s: &mut Vec<i32>) {
    if s.len() < 2 {
        return;
    }
    let length = s.len();
    for i in 0..length {
        let mut min_index = i;
        for j in i + 1..length {
            if s[min_index] > s[j] {
                min_index = j;
            }
        }
        if min_index != i {
            swap(s, min_index, i);
        }
    }
}
//插入排序
fn insert_sort(s: &mut Vec<i32>) {
    if s.len() < 2 {
        return;
    }
    for mut i in 1..s.len() {
        for j in (0..i).rev() {
            if s[i] > s[j] {
                break;
            }
            swap(s, i, j);
            i = j;
        }
    }
}

// 归并排序 时间复杂度O（n*logn)
fn merge_sort(s: &mut Vec<i32>, l: usize, r: usize) {
    if l == r {
        return;
    }

    let mid = l + ((r - l) >> 1);
    merge_sort(s, l, mid);
    merge_sort(s, mid + 1, r);
    merge(s, l, mid, r)
}

fn merge(s: &mut Vec<i32>, l: usize, mid: usize, r: usize) {
    let mut help = Vec::new();
    let mut p1 = l;
    let mut p2 = mid + 1;
    while p1 <= mid && p2 <= r {
        if s[p1] < s[p2] {
            help.push(s[p1]);
            p1 += 1;
        } else {
            help.push(s[p2]);
            p2 += 1;
        }
    }
    while p1 <= mid {
        help.push(s[p1]);
        p1 += 1;
    }
    while p2 <= r {
        help.push(s[p2]);
        p2 += 1;
    }
    for i in 0..help.len() {
        s[l + i] = help[i];
    }
}

//异或运算 c= a^b a= c^b, b=c^a  0=c^c
fn swap(s: &mut Vec<i32>, i: usize, j: usize) {
    s[i] = s[i] ^ s[j];
    s[j] = s[i] ^ s[j];
    s[i] = s[i] ^ s[j];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut s = vec![3, 2, 1, 5, 9, 7];
        bubble_sort(&mut s);
        assert_eq!(s, vec![1, 2, 3, 5, 7, 9]);
        let mut s = vec![3, 2, 1, 5, 9, 7];
        select_sort(&mut s);
        assert_eq!(s, vec![1, 2, 3, 5, 7, 9]);
        let mut s = vec![3, 2, 1, 5, 9, 7];
        insert_sort(&mut s);
        assert_eq!(s, vec![1, 2, 3, 5, 7, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut s = vec![3, 2, 1, 5, 9, 7];
        let l = 0;
        let r = s.len() - 1;
        merge_sort(&mut s, l, r);
        assert_eq!(s, vec![1, 2, 3, 5, 7, 9]);
    }
}
