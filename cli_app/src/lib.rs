pub mod arg_config {
    use std::env;
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }
    impl Config {
        pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("at least 2 arguments are expected");
            }
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }
}

pub mod run {
    use super::arg_config;
    use std::error::Error;
    use std::fs;

    //func to read file from arg and check agaist query
    pub fn read_file(configuration: &arg_config::Config) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&configuration.filename)?;
        let results = if configuration.case_sensitive {
            search(&configuration.query, &content)
        } else {
            search_case_insensitive(&configuration.query, &content)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }

    //func to search query in content
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
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
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            run::search(query, contents)
        );
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
            run::search_case_insensitive(query, contents)
        );
    }
}
