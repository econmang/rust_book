pub mod parser {
    use std::env;
    use std::error::Error;
    use std::fs;

    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
            // clone has a runtime cost and more efficient methods of fixing ownership will be shown
            // in Chapter 13
            if args.len() < 3 {
                return Err("Not enough arguments");
            }
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config { query, file_path, ignore_case })
        }

        pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
            let contents = fs::read_to_string(config.file_path)?;

            if config.ignore_case == true {
                for line in search_case_insensitive(&config.query, &contents) {
                    println!("{line}");
                }
            } else {
                for line in search(&config.query, &contents) {
                    println!("{line}");
                }
            }

            Ok(())
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&'a str> = vec![];
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line.trim());
            }
        }
        return results;
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line.trim());
            }
        }

        return results;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], parser::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
            parser::search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], parser::search(query, contents));
    }
}
