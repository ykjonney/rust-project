use std::error::Error;
use std::sync::Arc;
use std::{fs, env};

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool
}

impl  Config {
    pub fn new(mut args:env::Args)->Result<Config, &'static str>{
        args.next();
        let query = match args.next(){
            Some(query)=>query,
            None =>return Err("didn't give a query string")
        };
        let filename = match args.next(){
            Some(filename) => filename,
            None => return Err("didn't give a filename")
        };
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query,filename,case_sensitive })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    println!("{}",config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("{}",config.case_sensitive);
    if config.case_sensitive{
        for line in search(&config.query, &contents){
            println!("{}",line);
        }
    }else{
        for line in search_case_insensitive(&config.query,&contents){
            println!("{}",line);
        }
    }
   
    Ok(())
}
pub fn search<'a>(query:&'a str,contents:&'a str)->Vec<&'a str>{
    // let mut result=Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         result.push(line);
    //     }
    // }
    // result
    //改良版
    contents.lines().filter(|x| x.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query:&'a str,contents:&'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}


struct Counter{
    count:u32,
}

impl Counter {
    fn new() -> Self{
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count <5{
            self.count +=1;
            Some(self.count)
        }else{
            None
        }
    }
}


use std::rc::Rc;
use std::cell::RefCell;



#[cfg(test)]
mod iterator_test{
    use super::*;
    #[test]
    fn test_iterator(){
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    #[test]
    fn using_other_iterator_trait_methods(){
        let counter = Counter::new();
        let sum:u32 = counter.zip(Counter::new().skip(1)).map(|(a,b)| a*b).
        filter(|x|x%3==0).sum();
        assert_eq!(18, sum);
    }
    #[test]
    fn cell_test(){
        let s = RefCell::new(vec![1,2,3]);
        let s1=&s;
        let s2=&s;
        s1.borrow_mut().push(4);
        s2.borrow_mut().push(5);
        assert_eq!(*s.borrow(),vec![1,2,3,4,5]);
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query="duct";
        let content="\
        Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."],search(query,content))
    }
    #[test]
    fn case_sensitive(){
        let query="duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";
        assert_ne!(vec!["safe, fast, productive."], search(query, contents));   
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

trait Draw {
    fn draw(&self)->String;
}

impl Draw for u8 {
    fn draw(&self)->String {
        format!("u8:{}",*self)
    }
}

impl Draw for f64 {
    fn draw(&self)->String {
        format!("f64:{}",*self)
    }
}

fn draw1(x:Box<dyn Draw>){
    let n = x.draw();
    println!("{}",n);
}
#[cfg(test)]
mod drawtest{
    use crate::draw1;
    #[test]
    fn draw_test(){
        let x = 5u8;
        draw1(Box::new(x));
    }
}



mod maptest{
    use std::collections::HashMap;

    #[test]
    fn map_test() {
        let team_list = vec![("中国",10),("越南",4)];
        let team_map: HashMap<_,_> = team_list.into_iter().collect();
        println!("{:?}",team_map);
    }
}

struct Container<T>(Arc<T>);
