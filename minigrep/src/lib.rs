use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// Box<dyn Error>はトレイトオブジェクトでErrorトレイトを実装する型
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    for line in search(&config.query, &content) {
        println!("this is {}", line);
    }
    Ok(())
}

// fn parse(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough");
    //     }
    //     parse(args)
    // }
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough");
    //     }
    //     Ok(parse(args))
    // }

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // プログラム名なのでskip
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query not found"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename not found"),
        };
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    content.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn on_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}