use core::panic;
extern crate regex;
use regex::Regex;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = std::env::args();
    match cli_parser(args) {
        Ok(cli) => {
            let content = read_file(cli.path);
            search_patterns(content, cli.pattern);
        }
        Err(e) => {
            eprintln!("Error: {} \n", e);
            eprintln!("USAGE: \n grasp <pattern> <path>");
        }
    }
}
//Returns an ENUM of type Return, to handle errors
fn cli_parser(mut args: std::env::Args) -> Result<Cli, String> {
    //.nth returns an Option, using match to handle the Option values
    let pattern = match args.nth(1) {
        Some(args) => args, // If .nth returns Some(args), then pattern = args
        None => {
            // if .nth returns nothing, then return an error
            return Err(
                "The following required arguments were not given : \n <Pattern> \n <Path>"
                    .to_string(),
            );
        }
    };

    if pattern == "--help" {
        println!("USAGE: \n grasp <pattern> <path>");
        std::process::exit(0);
    }

    let path = match args.nth(0) {
        Some(args) => args,
        None => return Err("The following required argument was not given: \n <Path>".to_string()),
    };
    Ok(Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    })
}

fn read_file(file: std::path::PathBuf) -> String {
    let file_content = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            panic!(
                "*********************Exiting due to the following error *********************{}",
                e
            );
        }
    };
    return file_content;
}

fn search_patterns(content: String, pattern: String) {
    let myRegex = Regex::new(&pattern).unwrap();
    for line in content.lines() {
        if myRegex.is_match(line) {
            println!("{}", line);
        }
    }
}
