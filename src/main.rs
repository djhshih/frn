use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("frn")
        .version("0.1")
        .about("Rename files using regular expression")
        .arg(
            Arg::new("regex")
                .help("substitution specified as a regular expression")
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
}
