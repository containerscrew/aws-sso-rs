use crate::aws::{
    generate_token, get_account_credentials, get_account_list,
    get_device_authorization_credentials, register_device_credentials, AccountCredentials,
    DeviceAuthCredentials, DeviceClientCredentials,
};
use crate::cli::Args;
use crate::logger::setup_logger;
use crate::utils::{open_browser_url, read_user_input, write_configuration};
use clap::Parser;
use colored::Colorize;
use tracing::error;

mod aws;
mod cli;
mod logger;
mod macros;

mod utils;

const RETRIES: u32 = 8;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup cli
    let cli = Args::parse();

    println_orange!("If you want to debug AWS SDK errors, run the program using: $ aws-sso-rs --log-level debug --start-url ...");

    // Logging
    setup_logger(&cli.log_level);

    // Start AWS SDK APi Calls
    let config = aws::init_config(&cli.aws_region).await;

    // AWS SSO IDC CLIENT
    let sso_idc_client = aws::ssoidc_client(&config, RETRIES).await;

    // AWS SSO CLIENT
    let sso_client = aws::sso_client(&config, RETRIES).await;

    // Register device and get client id and client secret
    let device_credentials: DeviceClientCredentials =
        register_device_credentials(&sso_idc_client).await?;

    // Get device user&device codes and verification url
    let device_auth_credentials: DeviceAuthCredentials =
        get_device_authorization_credentials(&sso_idc_client, &device_credentials, &cli.start_url)
            .await?;

    // Open default local browser with verification URL
    open_browser_url(&device_auth_credentials.verification_url);

    // To continue this program, user must accept the approval in the browser, without this we can't continue
    // That's why we need to pause the program until the user press Enter
    read_user_input();

    // Generate token
    let token = generate_token(
        &sso_idc_client,
        &device_credentials,
        &device_auth_credentials,
    )
    .await?;

    // Get account list using the previous generate token
    let account_list = get_account_list(&sso_client, &token).await?;

    println_orange!("{} AWS accounts", account_list.len());

    let mut all_credentials: Vec<AccountCredentials> = vec![];

    // Store all join handles
    let mut join_handles = Vec::new();

    // Iterate over all accounts and get credentials for each account
    for account in account_list {
        let sso_client = sso_client.clone();
        let token = token.clone();
        join_handles.push(tokio::spawn(async move {
            let account_name = &account.account_name.unwrap();
            let account_credentials = match get_account_credentials(
                &sso_client,
                &account.account_id.unwrap(),
                &token,
                &account_name,
            )
            .await
            {
                Ok(account_credentials) => Ok(account_credentials),
                Err(err) => {
                    error!(
                        "Error fetching credentials for {}. {}. Retrying...",
                        &account_name, err
                    );
                    Err(err)
                }
            };
            account_credentials
        }));

        // tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Throttle requests to avoid rate limiting
    }

    // Wait for all tasks to complete
    for handle in join_handles {
        match handle.await.unwrap() {
            Ok(account_credentials) => all_credentials.extend(account_credentials),
            Err(_) => {} // not necessary to do anything with the error, it's printed above
        };
    }

    // Finally, write the config file
    write_configuration(all_credentials, cli.aws_region);

    Ok(())
}
