use crate::aws::AccountCredentials;
use configparser::ini::Ini;
use std::io;
use std::io::Write;
use tracing::{error, info};

pub fn open_browser_url(url: &String) {
    // From the device authorization, open the URL in the browser
    if webbrowser::open(&*url).is_ok() {
        info!("Opening your default web browser to complete the authentication process");
    } else {
        error!("Opening your default browser to complete the authentication process");
    }
}
pub fn read_user_input() {
    info!("Type ENTER to continue");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading enter key!");
}

pub fn extend_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}

const CREDENTIALS_FILE_PATH: &str = "~/.aws/credentials";

pub fn write_configuration(all_credentials: Vec<AccountCredentials>, region_name: String) {
    //Start configparser to write data
    let mut configuration = Ini::new_cs();
    let file_path = extend_path(CREDENTIALS_FILE_PATH);

    for creds in all_credentials {
        configuration.set(
            &format!("{}@{}", creds.account_name, creds.role_name),
            "region",
            Some(region_name.parse().unwrap()),
        );
        configuration.set(
            &format!("{}@{}", creds.account_name, creds.role_name),
            "aws_access_key_id",
            Option::from(creds.aws_access_key_id),
        );
        configuration.set(
            &format!("{}@{}", creds.account_name, creds.role_name),
            "aws_secret_access_key",
            Option::from(creds.aws_secret_access_key),
        );
        configuration.set(
            &format!("{}@{}", creds.account_name, creds.role_name),
            "aws_session_token",
            Option::from(creds.aws_session_token),
        );

        match configuration.write(&file_path) {
            Ok(_) => {}
            Err(err) => error!("Error writing configuration file {}", err),
        };
    }

    info!("Configuration file saved: {}", CREDENTIALS_FILE_PATH)
}
