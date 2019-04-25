extern crate config;
extern crate console;
extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Input};
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Debug)]
struct Config {
    message: String,
}

fn init_config<'a>() -> Result<Option<Config>, Box<Error>> {
    let theme = ColorfulTheme {
        ..ColorfulTheme::default()
    };

    let message = Input::with_theme(&theme)
        .with_prompt("Commit Message")
        .interact()?;

    Ok(Some(Config { message }))
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

    let mut branch_name = String::from(full_branch_name);
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

    let msg = format!("[{}] {}", branch_name, config.message);

    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings"))
        .unwrap();

    let gitpath = settings.get::<String>("gitpath").unwrap();

    let _git_add = Command::new(&gitpath)
        .arg("add")
        .arg(".")
        .output()
        .expect("command failed");

    let _git_message = Command::new(&gitpath)
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .output()
        .expect("command failed");
}
