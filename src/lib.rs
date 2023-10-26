use std::{fs, io};
use std::fs::File;
use std::io::Write;

const INSERT_COMMAND: &str = "insert";

#[derive(Debug)]
pub enum Command {
    Insert,
    Show,
}

#[derive(Debug)]
pub struct Config {
    pub command: Command,
    pub resource: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no arguments");
        }

        let command = match args[1].as_str() {
            INSERT_COMMAND => Command::Insert,
            _ => Command::Show
        };
        let resource = match command {
            Command::Insert => match args.get(2) {
                Some(r) => Some(r.clone()),
                None => return Err("resource not found"),
            },
            Command::Show => match args.get(2) {
                None => None,
                Some(r) => Some(r.clone())
            },
        };

        Ok(Config {
            command,
            resource,
        })
    }
}

pub fn run(config: Config) -> Result<(), std::io::Error> {
    create_store_dir()?;

    match config.command {
        Command::Show => match config.resource {
            None => {
                show_all_passwords();
                Ok(())
            },
            Some(r) => {
                show_password(&r);
                Ok(())
            }
        },
        Command::Insert => {
            let login = get_user_input(UserInputType::Login);
            let password = get_user_input(UserInputType::Password);
            let data_to_write = format!("{}\n{}", login, password);
            create_file(&config.resource.unwrap(), data_to_write)
        }
    }
}

fn show_all_passwords() {
    let resources = fs::read_dir(STORE_DIR).unwrap();
    for resource in resources {
        println!("{}", resource.unwrap().file_name().to_str().unwrap())
    }
}
fn show_password(resource: &str) {
    let contents = fs::read_to_string(format!("{}/{}", STORE_DIR, resource)).expect("Should have been able to read the file");
    println!("credits for {}:\n{}", resource, contents);
}

const STORE_DIR: &str = ".passman-password-store";
fn create_store_dir() ->  io::Result<()> {
    fs::create_dir_all(STORE_DIR)
}

enum UserInputType {
    Login,
    Password
}
fn get_user_input(input_type: UserInputType) -> String {
    let requested_data = match input_type {
        UserInputType::Login => "login",
        UserInputType::Password => "password"
    };

    print!("Type {}: ", requested_data);
    std::io::stdout().flush().unwrap();

    let mut result = String::new();

    std::io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result
}

fn create_file(file_name: &str, data: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}/{}", STORE_DIR ,file_name))?;

    file.write_all(data.as_bytes())?;
    Ok(())
}