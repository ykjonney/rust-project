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

fn main() {
    // let mut s = String::new();

    // let update_string =  |str| s.push_str(str);

    // exec(update_string);

    // println!("{:?}",s);
    let mut s = vec![3,2,1,5,9,7];
    bubble_sort(&mut s);
    println!("{:?}",s);
    let mut s = vec![3,2,1,5,9,7];
    select_sort(&mut s);
    println!("{:?}",s);
    let mut s = vec![3,2,1,5,9,7];
    insert_sort(&mut s);
    println!("{:?}",s);

}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
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
