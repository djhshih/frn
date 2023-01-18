use clap::{Arg, Command, ArgAction, value_parser};
use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let matches = Command::new("frn")
        .version("0.1")
        .about("Rename files using regular expression")
        .arg(
            Arg::new("regex")
                .help("Substitution expression 's/pattern/replacement/g'")
        )
        .arg(
            Arg::new("file")
                .action(ArgAction::Append)
                .help("File(s) to rename")
        )
        .arg(
            Arg::new("history").short('l').long("history")
                .required(false)
                .help("File to record history")
                .default_value(".frn_history")
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            Arg::new("run").short('r').long("run")
                .action(ArgAction::SetTrue)
                .help("Run the rename operation (default is dry-run)")
        )
        .arg(
            Arg::new("force").short('f').long("force")
                .action(ArgAction::SetTrue)
                .help("Allow files to be overwritten")
        )
        .get_matches();

    let regex = matches.get_one::<String>("regex")
        .expect("regular expression is required");
    let files = matches.get_many::<String>("file")
        .expect("at least one file name is required")
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let history = matches.get_one::<PathBuf>("history")
        .expect("history file is invalid");

    let run = matches.get_flag("run");
    let force = matches.get_flag("force");

    // find second character to use for splitting
    let split_char = regex.chars().nth(1)
        .expect("regex must be formatted as s/pattern/replacement/");

    // parse substitution expression
    let mut parts = regex.split(split_char);
    let keyword = parts.next().expect("regex must be formatted as s/pattern/replacement/");
    assert_eq!(keyword, "s", "substitution command is missing");
    let pattern = parts.next().expect("pattern is missing in s/pattern/replacement/");
    let replacement_raw = parts.next().expect("replacement is missing in s/pattern/replacement/");
    // parse options
    let global = match parts.next() {
        None => false,
        Some("") => false,
        Some("g") => true,
        Some(x) => {
            panic!("regex option '{}' is not supported", x)
        }
    };

    // process replacement backferences from \1 to $1
    // because crate regex uses $ instead of \ for backreference
    let re_backref = Regex::new(r"\\(\d+)").unwrap();
    let replacement_new: String = re_backref.replace_all(replacement_raw, r"$$1").into();
    let replacement = &replacement_new;

    // run substitution to file names
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

    if run {
        // execute the file rename operations
        let mut outf = fs::OpenOptions::new().create(true).append(true).open(history)
            .expect("could not open history file for logging");
        for (x, y) in files.iter().zip(new_names.iter()) {
            match y {
                None => {},
                Some(y) => { 
                    if !Path::new(y).exists() || force {
                        match fs::rename(x, y) {
                            Ok(()) => {
                                println!("{} -> {}", x, y);
                                // record history to file
                                writeln!(&mut outf, "mv {} {}", x, y)
                                    .expect("failed to log history");
                            },
                            Err(_) => println!("Warning: could not rename {} -> {}", x, y)
                        }
                    } else {
                        println!("Warning: did not rename {} -> {} because target exists", x, y);
                    }
                },
            }
        }
    } else {
        // print proposed file rename
        let mut can_rename = false;
        for (x, y) in files.iter().zip(new_names.iter()) {
            match y {
                None => {},
                Some(y) => { 
                    println!("{} -> {}", x, y);
                    can_rename = true;
                }
            }
        }
        if can_rename {
            println!("No operation performed; confirm by `frn -r`")
        }
    }

}
