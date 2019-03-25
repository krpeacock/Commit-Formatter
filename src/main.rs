extern crate config;
extern crate console;
extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Input};
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

#[derive(Debug)]
struct Config {
    user: String,
    message: String,
}

fn init_config<'a>() -> Result<Option<Config>, Box<Error>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();
    let user = &settings_map["user"];

    let theme = ColorfulTheme {
        ..ColorfulTheme::default()
    };

    let message = Input::with_theme(&theme)
        .with_prompt("Commit Message")
        .interact()?;

    Ok(Some(Config {
        message,
        user: user.to_string(),
    }))
}

fn main() {
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => format(config),
        Err(err) => println!("error: {}", err),
    }
}

fn format(config: Config) {
    println!("{:#?}", config);
    let _make_commit = Command::new("/usr/local/bin/git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("command failed");
}
