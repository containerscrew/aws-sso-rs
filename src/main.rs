use crate::aws::{
    AccountCredentials, DeviceAuthCredentials, DeviceClientCredentials, get_account_credentials,
    get_account_list, get_device_authorization_credentials, poll_token,
    register_device_credentials,
};
use crate::cli::Args;
use crate::utils::{open_browser_url, write_configuration};
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tiny_tracing::Logger;
use tokio::time::Instant;
use tracing::{Level, error, info, warn};

mod aws;
mod cli;
mod utils;

const RETRIES: u32 = 7;

type BoxError = Box<dyn Error + Send + Sync>;

fn logger_selector(level: &str) -> Level {
    match level {
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "trace" => Level::TRACE,
        "error" => Level::ERROR,
        "debug" => Level::DEBUG,
        _ => Level::INFO, // Default, even though clap cli restrict the value to one of from above
    }
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let started = Instant::now();

    // Setup cli
    let cli = Args::parse();

    // Logging
    let level = logger_selector(&cli.log_level.as_str());
    let _guard = Logger::new()
        .with_level(level)
        .with_env_filter_from_env()
        .with_timestamp(cli.with_timestamp)
        .init()?;

    info!(
        "Welcome to aws-sso-rs. Approve the request in your browser, the program continues on its own"
    );
    warn!("Your ~/.aws/credentials will be overwritten once the credentials are fetched");

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

    info!("Device code: {}", &device_auth_credentials.user_code);

    // Open default local browser with verification URL
    open_browser_url(&device_auth_credentials.verification_url);

    info!(
        "Verification URL: {}",
        &device_auth_credentials.verification_url
    );

    // The user must approve the request in the browser to continue, so poll until it is approved
    let token = Arc::new(
        poll_token(
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

    // Iterate over all accounts and get credentials for each account
    for account in account_list {
        let sso_client = sso_client.clone();
        let token = Arc::clone(&token);

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

            info!("{}", account_name);
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

    info!("Done in {:?}", started.elapsed());

    Ok(())
}
