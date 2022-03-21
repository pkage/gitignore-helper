use clap::Parser;
use colored::*;

mod ignores;

#[derive(Parser, Debug)]
#[clap(
    version="0.1.0", 
    author="Patrick Kage (patrick@ka.ge)",
    about="Get and display .gitignore files for various languages directly from github."
)]
struct Opts {
    #[clap(name="language", help="Language to fetch .gitignore for")]
    name: Option<String>,

    #[clap(short='l', long="list", help="List all languages")]
    list: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.list {
        match ignores::get_template_list() {
            Ok(arr) => {
                println!("{}", "showing languages".blue());
                for lang in arr {
                    println!("{}", lang)
                }
            }
            Err(()) => {
                eprintln!("{}", "could not load language listing".red())
            }
        }
        return
    }

    let name: String = match opts.name {
        Some(name) => name,
        None => return
    };


    let template = match ignores::get_template(name.clone()) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("{}", format!("could not get template for '{}'", name).red());
            return
        }
    };

    eprintln!("writing template '{}' to stdout", template.name.blue());

    println!("## start {} gitignore", template.name);
    println!("{}", template.source);
    println!("## end {} gitignore", template.name);
}



