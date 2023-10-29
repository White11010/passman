use std::io::{Write, stdout};
use std::{process, thread};
use std::time::Duration;
use bcrypt::BcryptError;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn run() {
    let master_password = get_master_password();
    if master_password.len() == 0 {
        println!("WARNING! You typed empty master password!")
    }

    let account = get_account_name();

    let data_to_encode = format!("{}{}", master_password.trim_end(), account.trim_end());

    let hashed_password = get_hashed_password(data_to_encode).unwrap_or_else(|err| {
        eprintln!("Error during hashing: {}", err);
        process::exit(1);
    });


    let encoded_password = get_encoded_password(hashed_password);

    set_password_to_clipboard(encoded_password);
}

fn get_master_password() -> String {
    rpassword::prompt_password("Master password: ").unwrap()
}

fn get_account_name() -> String {
    print!("Account: ");
    stdout().flush().unwrap();

    let mut input_result = String::new();
    std::io::stdin()
        .read_line(&mut input_result)
        .expect("Failed to read line");

    input_result
}

fn get_hashed_password(data: String) -> Result<String, BcryptError> {
    let hashed_password = bcrypt::hash_with_salt(data, 11, [7; 16])?.to_string();
    let hash = hashed_password.split("$")
        .nth(3)
        .unwrap()
        .chars()
        .skip(22)
        .take(16)
        .collect();
    Ok(hash)
}

fn get_encoded_password(data: String) -> String {
    base85::encode(data.as_bytes())
}

fn set_password_to_clipboard(password: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password).unwrap();

    let mut stdout = stdout();

    for i in 1..=15 {
        let seconds_left = match 15 - i {
            d if d < 10 => format!("0{}", d),
            d if d >= 10 => format!("{}", d),
            _ => format!("")
        };
        print!("\rIn clipboard! {}", seconds_left);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    ctx.clear().unwrap();
}

