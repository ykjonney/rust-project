
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

    let mid = l + ((r - l) >> 1);//防止l+r溢出
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


fn quick_sort(arr: &mut Vec<i32>){
    let len = arr.len();
    if len >1{
        quick_sort_range(arr, 0, len-1)
    }

}
fn quick_sort_range(arr:&mut Vec<i32>,low:usize,high:usize){
    if low<high{
        let partition = partition(arr, low, high); 
        quick_sort_range(arr, low, partition.0);
        quick_sort_range(arr, partition.1+1, high);
    }
}

//
fn partition(arr:&mut Vec<i32>,low:usize,high:usize)->(usize,usize){
    use rand::Rng;
    // 随机数增加效率
    let r = rand::thread_rng().gen_range(low..=high);
    arr.swap(high,r);
    let (pivot,mut cur,mut left,mut right) = (high,low,(low as isize-1) as i32,high);
    while cur<right {
        //如何当前值小于标准值，将当前值和左边界前一个值交换，当前值前移，左边界前移
        if arr[cur]<arr[pivot]{
            left+=1;
            // swap(arr, cur, left as usize);
            arr.swap(cur,left as usize);
            cur +=1;
        }
        else if arr[cur]>arr[pivot] {
            right -=1;
            // swap(arr, cur, right);
            arr.swap(cur,right);
        }else{
            cur+=1;
        }
    }
    arr.swap(right,high);
    left = if left<0{0}else{left};
    return (left as usize,right);



}

fn heap_sort(arr:&mut Vec<i32>){
    let mut heap_size = arr.len()-1;
    if arr.len()<2{return;}
    for i in 1..arr.len()  { //O(n)
        heap_insert(arr, i); // o(logn)
    }
    
    arr.swap(0,heap_size);
    println!("{:?}",arr);
    while heap_size>0 {
        heapify(arr, 0, heap_size);
        heap_size -=1;
        arr.swap(0,heap_size);
        println!("{:?},{heap_size}",arr);
    }
    // println!("{heap_size}");
}

//某个数处在i位置，往上继续移动
fn heap_insert(arr: &mut Vec<i32>,i: usize){
    let mut index = i;
    while arr[index]>arr[((index as isize-1)/2)as usize] {
        arr.swap(index,(index-1)/2);
        index = (index-1)/2
    }
}

fn heapify(arr: &mut Vec<i32>,i:usize,heap_size:usize){
    let mut left = i*2+1; //左孩子下标
    let mut parent = i; //当前下标
    while left < heap_size {
        //比较左右孩子求出最大者下标
        let mut largest = if left +1 < heap_size&&arr[left] < arr[left+1] {left+1} else{left} ;
        println!("{left},{largest},{heap_size}");
        // 将最大者和父亲节点比较求出最大者下标
        largest = if arr[parent]> arr[largest]{parent} else{largest};
        //如果是父亲节点自己就结束
        if largest == parent{
            break;
        }
        // 反之，交换两者
        arr.swap(parent,largest);
        println!("heapify:{parent},{largest},{:?}",arr);
        // 当前节点更换
        parent = largest;
        left = parent * 2+1;
    }
   

}

//基数排序
fn radix(arr: &mut Vec<usize>){
    //计算位数
    let digist = get_radix(arr);
   //有多少数准备多大空间
   let mut bucket = vec![0;arr.len()];
   for d in 1..digist+1{
        let mut count = vec![0;10];//计算各个位上的数出现的次数
        for i in 0..arr.len(){
            let index = get_digist(arr[i],d);
            count[index] += 1;
        }
        //计算前缀和
        for i in 1..count.len(){
            count[i] += count[i-1];
        }
        //取出数据
        for i in (0..arr.len()).rev(){
            //获取当前数据arr[i]在d位的数字
            let index = get_digist(arr[i], d);
            //将数据arr[i]分配到bucket的对应位置
            bucket[count[index]-1] = arr[i];
            //将count对应索引为index的count数减去1
            count[index] -=1;
        }
        //将bucket数据给arr
       for i in 0..bucket.len(){
           arr[i] = bucket[i];
       }    
   }
}

use std::usize;
fn get_digist(num:usize,d:usize)->usize{
    return num /usize::pow(10,(d-1)as u32)%10
}

//找出数组中数的最大位数
fn get_radix(arr: &Vec<usize>)->usize{
    let mut max = 0;
    for i in 0..arr.len(){
        let mut d =0;
        let mut num = arr[i];
        while num !=0 {
            num /= 10;
            d += 1;
        }
        if d>max{
            max = d;
        }
    }
    return max;
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

    #[test]
    fn test_quick_sort(){
        let mut s = vec![3, 2, 1, 5, 9, 7];
        quick_sort(&mut s);
        assert_eq!(s,vec![1, 2, 3, 5, 7, 9]);
    }

    #[test]
    fn test_heap_sort(){
        let mut s = vec![3, 2, 1, 5, 9, 7,4];
        heap_sort(&mut s);
        assert_eq!(s,vec![1, 2, 3,4, 5, 7, 9]);
    }
    #[test]
    fn test_radix_sort(){
        let mut s = vec![3, 2, 1, 5, 9, 7,4];
        radix(&mut s);
        assert_eq!(s,vec![1, 2, 3,4, 5, 7, 9]);
    }
}
