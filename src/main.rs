use clap::{Arg, Command, ArgAction};

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
        .get_matches();

    let regex = matches.get_one::<String>("regex")
        .expect("regular expression is required");
    let files = matches.get_many::<String>("file")
        .expect("at least one file name is required")
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    
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
}
