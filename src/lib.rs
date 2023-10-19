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
    pub resource: String,
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
                Some(r) => r.clone(),
                None => return Err("resource not found"),
            },
            Command::Show => String::from(args.get(1).unwrap()),
        };

        Ok(Config {
            command,
            resource,
        })
    }
}

pub fn run(config: Config) -> Result<(), std::io::Error> {
    match config.command {
        Command::Show => {
            println!("{}", config.resource);
            Ok(())
        },
        Command::Insert => {
            let login = get_user_input(UserInputType::Login);
            let password = get_user_input(UserInputType::Password);
            let data_to_write = format!("{}\n{}", login, password);
            create_file(&config.resource, data_to_write)
        }
    }
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
    let mut file = File::create(file_name)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}