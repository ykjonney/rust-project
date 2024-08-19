

fn build_next(p:&str) ->Vec<i32>{
    let mut next = vec![0;p.len()];
    let mut k=0;
    let mut i=1;
    while i<p.len(){
        if p.chars().nth(i)==p.chars().nth(k){
            k+=1;
            next[i]=k as i32;
            i+=1;
        }else{
            if k>0{
                k = next[k as usize-1] as usize;
            }else{
                next[i]=0;
                i+=1;
            }
        }
    }
    next

}

fn kmp_search(text:&str,p:&str) ->Option<usize>{
    let next = build_next(p);
    let mut i=0;
    let mut j=0;
    while i<text.len() && j<p.len(){
        if text.chars().nth(i)==p.chars().nth(j){
            i+=1;
            j+=1;
        }else{
            if j != 0{
                j = next[j-1] as usize;
            }else{
                i+=1;
            }
        }
    }
    if j==p.len(){
        Some(i-j)
    }else{
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_kmp(){
        let text = "BBC ABCDAB ABCDABCDABDE";
        let p = "ABCDABD";
        assert_eq!(kmp_search(text,p),Some(15));
    }
}