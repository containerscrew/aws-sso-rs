use crate::aws::AccountCredentials;
use configparser::ini::Ini;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use tracing::error;

pub fn open_browser_url(url: &String) {
    // From the device authorization, open the URL in the browser
    if !webbrowser::open(&*url).is_ok() {
        error!("Opening your default browser to complete the authentication process");
    }
}

pub fn read_user_input() {
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

pub fn write_configuration(
    all_credentials: Vec<AccountCredentials>,
    region_name: String,
    role_overrides: HashMap<String, String>,
    account_overrides: HashMap<String, String>,
) {
    let mut configuration = Ini::new_cs();
    let file_path = extend_path(CREDENTIALS_FILE_PATH);

    for creds in all_credentials {
        let account_name = &creds.account_name;
        let role_name = &creds.role_name;

        let account_part = account_overrides
            .get(account_name)
            .cloned()
            .unwrap_or_else(|| account_name.clone());

        // Check for override
        let profile_name = match role_overrides.get(role_name) {
            Some(override_val) if override_val.is_empty() => account_part.clone(),
            Some(override_val) => override_val.clone(),
            None => format!("{account_part}@{role_name}"),
        };

        configuration.set(&profile_name, "region", Some(region_name.clone()));
        configuration.set(
            &profile_name,
            "aws_access_key_id",
            Some(creds.aws_access_key_id),
        );
        configuration.set(
            &profile_name,
            "aws_secret_access_key",
            Some(creds.aws_secret_access_key),
        );
        configuration.set(
            &profile_name,
            "aws_session_token",
            Some(creds.aws_session_token),
        );

        if let Err(err) = configuration.write(&file_path) {
            error!("Error writing configuration file: {}", err);
        }
    }
}
