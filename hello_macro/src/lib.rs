pub trait HelloMacro {
    fn hello_macro();
}
// 特征对象的方法不能带有任何泛型参数
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Write};

    #[test]
    fn main1() -> Result<(), Error> {
        let path = "lines.txt";

        // 创建文件
        let mut output = File::create(path)?;
        // 写入三行内容
        write!(output, "Rust\n💖\nFun")?;

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        // 迭代文件中的每一行内容，line 是字符串
        for line in buffered.lines() {
            println!("{}", line?);
        }

        Ok(())
    }

    #[test]
    fn test_filter_map() {
        let v = vec!["1", "2", "we"];
        let mut iter = v.iter().filter_map(|s| s.parse().ok());
        assert_eq!(iter.next(), Some(1));
    }
}
