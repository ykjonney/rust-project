// 经典排序算法实现集合
// 本文件包含 7 种常见排序算法的 Rust 实现
// 包括：冒泡(O n²)、选择(O n²)、插入(O n²)、归并(O n log n)、
// 快速排序(O n log n avg)、堆排序(O n log n)、基数排序(O d*n)
// 所有排序作用于 i32 向量，大多采用原地排序方案

/// 冒泡排序（Bubble Sort）
/// 原理：相邻元素比较与交换，每轮将最大元素"冒"到末尾
/// 时间复杂度：O(n²)，空间复杂度：O(1)，稳定排序
/// 适用场景：教学、小数据集或几乎有序的数组
#[allow(dead_code)]
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

/// 选择排序（Selection Sort）
/// 原理：每轮从未排序部分选出最小元素，放到已排序末尾
/// 时间复杂度：O(n²)，空间复杂度：O(1)，不稳定排序
/// 适用场景：内存限制严格、寻求最少写操作
#[allow(dead_code)]
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

/// 插入排序（Insertion Sort）
/// 原理：从第二个元素开始，逐个插入已排序部分的正确位置
/// 时间复杂度：O(n²)，空间复杂度：O(1)，稳定排序
/// 适用场景：小数据集、几乎有序的数组，实际性能优于冒泡
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

/// 归并排序（Merge Sort）
/// 原理：分治法，递归分割数组为左右两部分，然后合并已排序的部分
/// 时间复杂度：O(n log n)，空间复杂度：O(n)，稳定排序
/// 适用场景：需要稳定排序、外排序、链表排序
fn merge_sort(s: &mut Vec<i32>, l: usize, r: usize) {
    if l == r {
        return;
    }

    let mid = l + ((r - l) >> 1);//防止l+r溢出
    merge_sort(s, l, mid);
    merge_sort(s, mid + 1, r);
    merge(s, l, mid, r)
}

/// 合并辅助函数：将已排序的左右子数组合并到 l..r 范围
/// 参数：l - 左起点, mid - 中间位置, r - 右结束
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

/// 快速排序（Quick Sort）
/// 原理：分治法，基于一个 pivot 元素对数组进行分区，然后递归排序子数组
/// 时间复杂度：O(n log n) 平均，O(n²) 最坏，空间复杂度：O(log n)，不稳定排序
/// 适用场景：平均性能最优，是实际应用最频繁的排序算法之一（被广泛用于库函数）
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

/// 分区辅助函数：选择 pivot 对数组进行三向分区（< = >）
/// 返回 (左边界, 右边界)，分区使用随机化以避免最坏情况
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

/// 堆排序（Heap Sort）
/// 原理：利用堆的性质（大顶堆或小顶堆）进行排序，首先建堆，然后逐个移除堆顶
/// 时间复杂度：O(n log n)，空间复杂度：O(1)，不稳定排序
/// 适用场景：需要O(1)空间、对最坏情况有保证的场景
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

/// 堆插入辅助函数
/// 向大顶堆中插入元素，并通过上浮操作维持堆的性质
/// 参数：arr-堆数组，i-要插入元素的索引
fn heap_insert(arr: &mut Vec<i32>,i: usize){
    let mut index = i;
    while arr[index]>arr[((index as isize-1)/2)as usize] {
        arr.swap(index,(index-1)/2);
        index = (index-1)/2
    }
}

/// 堆化辅助函数（Heapify）
/// 从索引i开始，通过下沉操作维持大顶堆的性质
/// 参数：arr-堆数组，i-开始下沉的索引，heap_size-堆的有效大小
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

/// 基数排序（Radix Sort）
/// 原理：从最低有效位到最高有效位，依次按数字的每一位进行计数排序
/// 时间复杂度：O(d*n)（d为最大数的位数），空间复杂度：O(n)，稳定排序
/// 适用场景：整数排序、非负数、位数固定的场景（不涉及对象比较）
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

/// 获取第d位数字辅助函数
/// 从数字的右侧开始计数，返回第d位的数字（例：123的第2位是2）
/// 参数：num-待提取数字，d-位置（从1开始）
fn get_digist(num:usize,d:usize)->usize{
    return num /usize::pow(10,(d-1)as u32)%10
}

/// 获取最大位数辅助函数
/// 扫描数组找出最大数的位数，作为基数排序的迭代次数
/// 参数：arr-数组
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
