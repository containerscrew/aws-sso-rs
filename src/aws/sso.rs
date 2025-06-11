use crate::aws::AccountCredentials;
use aws_config::retry::{RetryConfig, RetryMode};
use aws_config::SdkConfig;
use aws_sdk_sso as sso;
use aws_sdk_sso::types::AccountInfo;

pub async fn sso_client(config: &SdkConfig, retries: u32) -> sso::Client {
    aws_sdk_sso::Client::from_conf(
        sso::config::Builder::from(config)
            .retry_config(
                RetryConfig::standard()
                    .with_max_attempts(retries)
                    .with_retry_mode(RetryMode::Standard),
            )
            .build(),
    )
}

pub async fn get_account_list(
    client: &sso::client::Client,
    token: &String,
) -> Result<Vec<AccountInfo>, sso::Error> {
    let account_list_output = client
        .list_accounts()
        .access_token(token)
        .max_results(123)
        .send()
        .await?;

    Ok(account_list_output.account_list.unwrap())
}

pub async fn get_account_credentials(
    client: &sso::client::Client,
    account_id: &String,
    token: &String,
    account_name: &String,
) -> Result<Vec<AccountCredentials>, sso::Error> {
    let roles = client
        .list_account_roles()
        .account_id(account_id)
        .access_token(token)
        .send()
        .await?;

    let mut account_credentials: Vec<AccountCredentials> = vec![];

    for role in roles.role_list.unwrap() {
        let role_credentials = client
            .get_role_credentials()
            .role_name(role.role_name.clone().unwrap())
            .account_id(account_id)
            .access_token(token)
            .send()
            .await?;

        account_credentials.push(AccountCredentials {
            account_name: account_name.replace(" ", ""),
            role_name: role.role_name.clone().unwrap(),
            aws_access_key_id: role_credentials
                .role_credentials
                .clone()
                .unwrap()
                .access_key_id
                .unwrap(),
            aws_secret_access_key: role_credentials
                .role_credentials
                .clone()
                .unwrap()
                .secret_access_key
                .unwrap(),
            aws_session_token: role_credentials
                .role_credentials
                .clone()
                .unwrap()
                .session_token
                .unwrap(),
        })
    }

    Ok(account_credentials)
}
