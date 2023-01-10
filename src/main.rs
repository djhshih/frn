use clap::{Arg, Command, ArgAction};
use regex::Regex;
use std::fs;

fn main() {
    let matches = Command::new("frn")
        .version("0.1")
        .about("Rename files using regular expression")
        .arg(
            Arg::new("regex")
                .help("substitution expression 's/pattern/replacement/'")
        )
        .arg(
            Arg::new("file")
                .action(ArgAction::Append)
                .help("file(s) to rename")
        )
        .arg(
            Arg::new("apply").short('v').action(ArgAction::SetTrue)
        )
        .get_matches();

    let regex = matches.get_one::<String>("regex")
        .expect("regular expression is required");
    let files = matches.get_many::<String>("file")
        .expect("at least one file name is required")
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let apply = matches.get_flag("apply");
    
    println!("regex: {}", &regex);
    println!("files: {:?}", &files);

    // parse substitution expression
    let mut parts = regex.split('/');
    let keyword = parts.next().expect("regex must formatted as s/pattern/replacement/");
    assert_eq!(keyword, "s", "substitution command is missing");
    let pattern = parts.next().expect("pattern is missing in s/pattern/replacement/");
    let replacement = parts.next().expect("replacement is missing in s/pattern/replacement/");
    // parse options
    let global = match parts.next() {
        None => false,
        Some("") => false,
        Some("g") => true,
        Some(x) => {
            panic!("option '{}' is not supported", x)
        }
    };

    println!("pattern: {}", &pattern);
    println!("replacement: {}", &replacement);
    println!("global: {}", global);

    // apply substitution to file names
    let re = Regex::new(pattern).expect("regex pattern is not valid");
    let new_names = files.iter().map(|s| {
        match re.find(s) {
            None => None,
            Some(_) => Some(
                if global {
                    re.replace_all(s, replacement).into_owned()
                } else {
                    re.replace(s, replacement).into_owned()
                }
            ),
        }
    }).collect::<Vec<_>>();
    
    // print the file name substitutions
    for (x, y) in files.iter().zip(new_names.iter()) {
        match y {
            None => {},
            Some(y) => { println!("{} -> {}", x, y); },
        }
    }

    // execute the file rename operations
    if apply {
        for (x, y) in files.iter().zip(new_names.iter()) {
            match y {
                None => {},
                Some(y) => { 
                    match fs::rename(x, y) {
                        Ok(()) => {},
                        Err(_) => println!("Warning: could not rename {} -> {}", x, y)
                    }
                },
            }
        }
    }

}
