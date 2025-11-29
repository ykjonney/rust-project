
fn main(){
    let mut s = vec![1,3,7,2,5];
    let r= s.len()-1;
    let sum = min_sum(&mut s,0,r);
    println!("{sum:}");
    println!("{:?}",s);
    assert_eq!(sum,12)
}
// 求一个数组的最小和例如[2,4,3,1]的最小和为0+2+2+0=4

fn min_sum(s:&mut Vec<i32>,l:usize,r:usize)->usize{
    if l==r{
        return 0;
    }
    let mid = l +((r-l)>>1);
    let l_sum = min_sum(s,l,mid);
    let r_sum = min_sum(s,mid+1,r);
    merge(s,l,mid,r)+l_sum+r_sum
}

fn merge(s:&mut Vec<i32>,l:usize,mid:usize,r:usize)->usize{
    let mut help = Vec::new();
    let mut p1 = l;
    let mut p2 = mid+1; 
    let mut res=0;
    while p1<=mid&&p2<=r{
        if s[p1]<s[p2]{
            res += (r-p2+1)*(s[p1]as usize);
            help.push(s[p1]);
            p1+=1;
        }else{
            // res +=0;
            help.push(s[p2]);
            p2+=1;
        }
    }
    while p1<=mid{
        help.push(s[p1]);
        p1+=1;
    }
    while p2<=r{
        help.push(s[p2]);
        p2+=1;
    }
    for i in 0..help.len(){
        s[l+i] = help[i];
    }
    return res;
}


