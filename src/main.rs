extern crate clap;
use clap::{Arg, App};
use git2::{Repository, Branch};

fn main() {
    let message = App::new("cmt")
        .version("1.0")
        .about("Formats a git commit message")
        .author("Kyle Peacock")
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("MESSAGE")
            .help("message to pass into your commit message")
            .required(true)
        )
        .get_matches();
    let response = message.value_of("message").unwrap_or("default.conf");
    println!("Value for response: {}", response);

}


