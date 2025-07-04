use crate::aws::{
    generate_token, get_account_credentials, get_account_list,
    get_device_authorization_credentials, register_device_credentials, AccountCredentials,
    DeviceAuthCredentials, DeviceClientCredentials,
};
use crate::cli::Args;
use crate::logger::setup_logger;
use crate::utils::{open_browser_url, read_user_input, write_configuration};
use clap::Parser;
use console::{style, Emoji};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::Instant;
use tracing::error;

mod aws;
mod cli;
mod logger;

mod utils;

const RETRIES: u32 = 7;

static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":-)");

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();

    // Setup cli
    let cli = Args::parse();

    // Logging
    setup_logger(&cli.log_level);

    // Print welcome message
    println!(
        "~> Welcome to {} \n~> Press {} when you accept the request in your browser",
        style("aws-sso-rs").bold().red(),
        style("ENTER").bold().red(),
    );

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

    print!(
        "~> Device Code: {} ",
        style(&device_auth_credentials.user_code).bold().green()
    );

    // Open default local browser with verification URL
    open_browser_url(&device_auth_credentials.verification_url);

    // To continue this program, user must accept the approval in the browser, without this we can't continue
    // That's why we need to pause the program until the user press Enter
    read_user_input();

    // Generate token
    let token = Arc::new(
        generate_token(
            &sso_idc_client,
            &device_credentials,
            &device_auth_credentials,
        )
        .await?,
    );

    // Get account list using the previous generate token
    let account_list = get_account_list(&sso_client, &token).await?;

    let mut all_credentials: Vec<AccountCredentials> = vec![];

    // Store all join handles
    let mut join_handles = Vec::new();

    let pb = Arc::new(ProgressBar::new(account_list.len() as u64));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.red} [{elapsed_precise}] [\x1b[38;5;208m{bar:40}\x1b[0m] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    // Limit the number of concurrent tasks to avoid overwhelming the API
    let semaphore = Arc::new(Semaphore::new(12));

    // Iterate over all accounts and get credentials for each account
    for account in account_list {
        let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

        let sso_client = sso_client.clone();
        let token = Arc::clone(&token);
        let pb = Arc::clone(&pb);

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

            pb.set_message(format!("{account_name}"));
            pb.inc(1);

            drop(permit); // Release slot for the next task
            account_credentials
        }));
    }

    // Wait for all tasks to complete
    for handle in join_handles {
        match handle.await.unwrap() {
            Ok(account_credentials) => all_credentials.extend(account_credentials),
            Err(_) => {} // not necessary to do anything with the error, it's printed above
        };
    }

    // Finally, write the config file
    let role_overrides: HashMap<String, String> =
        cli.role_overrides.unwrap_or_default().into_iter().collect();
    let account_overrides: HashMap<String, String> = cli
        .account_overrides
        .unwrap_or_default()
        .into_iter()
        .collect();
    write_configuration(
        all_credentials,
        cli.aws_region,
        role_overrides,
        account_overrides,
    );

    println!(
        "~> Done in {} {}",
        HumanDuration(started.elapsed()),
        SPARKLE
    );

    Ok(())
}
