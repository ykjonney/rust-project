mod a {
    pub mod b {
        pub fn c() {
            println!("{:?}",crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}",Y);
}


struct A {
    a: i32,
}

impl A {
    fn new(a: i32) -> Self {
        Self { a }
    }

    fn get_a(&self) -> i32 {
        self.a
    }

    fn set_a(&mut self, a: i32) {
        self.a = a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = A::new(1);
        assert_eq!(a.get_a(), 1);
        a.set_a(2);
        a.set_a(3);
        assert_eq!(a.get_a(),3);
    }
}