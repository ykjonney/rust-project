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
}
