use crate::utils;
use aws_sso_auth::{
    generate_token, get_account_credentials, get_account_list,
    get_device_authorization_credentials, get_register_device_credentials, sso_client,
    ssoidc_client, AccountCredentials, DeviceAuthCredentials, DeviceClientCredentials,
};
use log::{debug, warn};
use tracing::{error, info};

pub fn start(region_name: String, start_url: String, workers: usize, retries: u32) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(workers) // more threads can't provide AWS API 429 responses
        .enable_time()
        .enable_io()
        .thread_name("aws-sso-auth")
        .build()
        .unwrap()
        .block_on(async_start(region_name, start_url, workers, retries));

    match runtime {
        Ok(_) => info!("All good, bye! 👋"),
        Err(err) => {
            error!(error = err, "Error executing tokio runtime");
            if err.to_string() == "InvalidRequestException" {
                error!("Is the configuration in ~/.aws/aws-sso-auth.json valid?");
            }
        }
    }
}

pub async fn async_start(
    region_name: String,
    start_url: String,
    workers: usize,
    retries: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // AWS Config
    let config = aws_sso_auth::init_config(region_name.clone()).await;

    // AWS SSOIDC CLIENT
    let ssoidc_client = ssoidc_client(&config, retries).await;

    // AWS SSO CLIENT
    let sso_client = sso_client(&config, retries).await;

    // Register device and get client id and client secret
    let device_credentials: DeviceClientCredentials =
        get_register_device_credentials(&ssoidc_client).await?;

    // Get device user&device codes and verification url
    let device_auth_credentials: DeviceAuthCredentials =
        get_device_authorization_credentials(&ssoidc_client, &device_credentials, &start_url)
            .await?;

    // Open default local browser with verification URL
    utils::open_browser_url(&device_auth_credentials.verification_url);

    // To continue this program, user must accept the approval in the browser, without this we can't continue
    // That's why we need to pause the program until the user press Enter
    utils::read_user_input();

    // Generate token
    let token = generate_token(
        &ssoidc_client,
        &device_credentials,
        &device_auth_credentials,
    )
    .await?;

    // Get account list using the previous generate token
    let account_list = get_account_list(&sso_client, &token).await?;

    // Provide info about all account that should be downloaded
    info!(
        "{} accounts to fetch. Each account can have multiple roles",
        account_list.len()
    );

    info!("Starting...");

    let mut all_credentials: Vec<AccountCredentials> = vec![];

    // Semaphore will control the number of concurrent threads
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(workers));

    let mut join_handles = Vec::new();

    // Start fetching credentials for all account/account-role
    for account in account_list {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
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
                Ok(account_credentials) => {
                    debug!("Credentials fetched for {}", &account_name);
                    Ok(account_credentials)
                }
                Err(err) => {
                    warn!(
                        "Error fetching credentials for {}. {}. Retrying...",
                        &account_name, err
                    );
                    Err(err)
                }
            };

            drop(permit);
            account_credentials
        }));
    }

    for handle in join_handles {
        match handle.await.unwrap() {
            Ok(account_credentials) => all_credentials.extend(account_credentials),
            Err(_) => {} // not necessary to do anything with the error, it's printed above
        };
    }

    utils::write_configuration(all_credentials, region_name);

    Ok(())
}
