extern crate clap;
extern crate config;
use clap::{Arg, App};
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap();

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

    println!("{:?}",
        settings.try_into::<HashMap<String, String>>().unwrap());
}


