extern crate chrono;
extern crate config;
extern crate console;
extern crate dialoguer;
use chrono::{DateTime, Utc};
use dialoguer::{theme::ColorfulTheme, Input};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
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
    let git_head =
        fs::read_to_string("./.git/HEAD").expect("Something went wrong reading the Git file");

    let full_branch_name = git_head
        .split("heads/")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(0)
        .unwrap();

    let mut branch_name = String::from("");
    let mut _ticket = String::from("");

    // Logic for parsing our particular branch naming convention
    // E.G kyle/DEV-1234-descriptive-comments
    if full_branch_name.contains("DEV") {
        if full_branch_name.contains("/") {
            if full_branch_name.find("/") < full_branch_name.find("DEV") {
                _ticket = String::from(full_branch_name.split("/").nth(1).unwrap())
            } else {
                _ticket = String::from(full_branch_name)
            }
        } else {
            _ticket = String::from(full_branch_name);
        }
        let iter: Vec<&str> = _ticket.split("-").collect();

        branch_name = String::from(iter[0]);
        if iter.len() > 1 {
            let joined_string = format!("{}-{}", iter[0], iter[1]);
            branch_name = String::from(joined_string);
        }
    }

    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%a %b %e %T %Y");

    let msg = format!(
        "[{}] <{}>\n{} by {}",
        branch_name, config.message, timestamp, config.user
    );
    let _git_add = Command::new("/usr/local/bin/git")
        .arg("add")
        .arg(".")
        .output()
        .expect("command failed");

    let _git_message = Command::new("/usr/local/bin/git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .output()
        .expect("command failed");
}
