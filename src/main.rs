use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    regex_str: String,

    #[arg(short='n', long)]
    rename: String,
    
    #[arg(short, long)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let filter = match Regex::new(&args.regex_str) {
        Ok(ok) => ok,
        Err(err) => {
            println!("{}", err.to_string());
            std::process::exit(1)
        }
    };

    let files: Vec<String> = fs::read_dir("./")
        .expect("cannot read directory")
        .map(|n| n.unwrap().path().display().to_string())
        .filter(|name| filter.is_match(&name))
        .collect();

    if args.test {
        for name in files {
            let name = name.replace("./", "");
            println!("{name} -> {}", filter.replace(&name, &args.rename));
        }
    } else {
        for name in files {
            let name = name.replace("./", "");
            fs::rename(&name, filter.replace_all(&name, &args.rename).to_string()).expect("cannot rename");
        }
    }
}
