// use std::collections::HashMap;


// fn main() {
    
//     let mut scores = HashMap::new();
//     scores.insert(String::from("bule"), 10);
//     scores.insert(String::from("yellow"), 50);
//     //获取hashmap值
//     //get
//     let team_name="bule".to_string();
//     let score=scores.get(&team_name);
//     println!("{:?}",score);
//     //循环遍历
//     for(k,v) in &scores{
//         println!("{},{}",k,v);
//     }
//     //更新hashmap的值
//     //覆盖已有的值
//     let old = scores.insert("bule".to_string(), 20);
//     assert_eq!(old,Some(10));
//     //查询新插入的值
//     let new = scores.get("bule");
//     assert_eq!(new,Some(&20));

//     //查询值若不存在则插入新值
//     let v = scores.entry("green".to_string()).or_insert(5);
//     assert_eq!(*v,5);
//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();
//     for word in text.split_whitespace(){
//         let count = map.entry(word).or_insert(0);
//         *count +=1;
//     }
//     println!("{:?}",map);
    
// }
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex,Condvar,mpsc};
use std::thread::{sleep};
use std::time::Instant;
fn main() {
//   
let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = pair.clone();
thread::spawn(move||{
    let (lock,cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    println!("started = {:?}",started);
    *started = true;
    cvar.notify_one();
});
let (lock,cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started {
   started =  cvar.wait(started).unwrap();
}
println!("started = {:?}",started);
//由于线程使用的是sender的clone 所以send并没有drop知道main线程结束才drop
 let (send,recv) = mpsc::channel();
 let num_threads = 2;
 for i in 0..num_threads {
    let thread_send = send.clone();
    thread::spawn(move||{
        thread_send.send(i).unwrap();
        println!("thread i: {} finished",i);
    });
}
//加上下面这句main就不会阻塞了
drop(send);

for x in recv{
    println!("recv: {}",x);
}
println!("finished iterating");


// 交替打印
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();
    let hand = thread::spawn(move||{
        let mut m: bool = {*cflag.lock().unwrap()};
        let mut counter = 0;
        while counter <3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }
            {
                m = false;
                *cflag.lock().unwrap() = false;
            }
            counter += 1;
            println!("inner counter:{}",counter);

        }
    });
    let mut counter = 0;
    loop{
        sleep(Duration::from_millis(100));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3{
            break;
        }
        println!("outer counter:{}",counter);
        cond.notify_one();
    }
    hand.join().unwrap();
    println!("{:?}",flag);

}

fn exec< F: FnMut(&str)>(mut f: F)  {
    f("hello")
}

// 冒泡排序
fn bubble_sort(s:&mut Vec<i32>){
    if s.len()<2{
        return ;
    }
    let length = s.len();
    for i in 0..length{
        for j in 0..length-i-1{
            if s[j] > s[j+1]{
                swap(s,j,j+1);
            }
        }
    }
}
//选择排序 选择最小的
fn select_sort(s:&mut Vec<i32>){
    if s.len()<2{
        return ;
    }
    let length = s.len();
    for i in 0..length{
        let mut min_index = i;
        for j in i+1..length{
            if s[min_index] > s[j]{
                min_index = j;
            } 
        }
        if min_index != i{
            swap(s, min_index, i);
        }
    }
}
//插入排序
fn insert_sort(s:&mut Vec<i32>){
    if s.len()<2{
        return ;
    }
    for i in 1..s.len(){
        for j in (0..i).rev(){
            if s[i]>s[j]{
                break;
            }
            swap(s, i, j);
        }
    } 

}

//异或运算 c= a^b a= c^b, b=c^a  0=c^c  
fn swap(s:&mut Vec<i32>,i:usize,j:usize){
    s[i] = s[i] ^ s[j];
    s[j] = s[i] ^ s[j];
    s[i] = s[i] ^ s[j];
}
//实现堆排序
fn heap_sort(s:&mut Vec<i32>){  
    let length = s.len();
    for i in (0..length/2).rev(){
        heap_adjust(s, i, length);
    }
    for i in (0..length).rev(){
        swap(s, 0, i);
        heap_adjust(s, 0, i);
    }

}
fn heap_adjust(s:&mut Vec<i32>, mut i:usize, length:usize){
    // 定义一个变量temp，用于存储当前元素
    let mut temp = s[i];
    // 从当前元素的父节点开始，从父节点到子节点的路径上继续遍历
    for mut j in (2*i+1..length).rev(){
        // 如果子节点存在且子节点的值小于父节点的值
        if j+1<length && s[j]<s[j+1]{
            // 将子节点的值赋值给父节点
            j+=1;
        }
        // 如果子节点的值小于父节点的值
        if temp<s[j]{
            // 将父节点的值赋值给子节点
            break;
        }
        // 将子节点的值赋值给父节点
        s[i] = s[j];
        // 让父节点赋值给当前元素
        i = j;
    }
    // 将当前元素赋值给父节点
    s[i] = temp;
}


