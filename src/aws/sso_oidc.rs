use crate::aws::dto::DeviceClientCredentials;
use crate::aws::DeviceAuthCredentials;
use aws_config::retry::{RetryConfig, RetryMode};
use aws_config::SdkConfig;
use aws_sdk_ssooidc as sso_oidc;

pub async fn ssoidc_client(config: &SdkConfig, retries: u32) -> sso_oidc::Client {
    sso_oidc::Client::from_conf(
        sso_oidc::config::Builder::from(config)
            .retry_config(
                RetryConfig::standard()
                    .with_max_attempts(retries)
                    .with_retry_mode(RetryMode::Standard),
            )
            .build(),
    )
}

pub async fn register_device_credentials(
    client: &sso_oidc::client::Client,
) -> Result<DeviceClientCredentials, sso_oidc::Error> {
    let client_registration = client
        .register_client()
        .client_name("aws-sso-rs")
        .client_type("public")
        .send()
        .await?;

    // Parse client id and secret
    let client_id = client_registration.client_id.unwrap();
    let client_secret = client_registration.client_secret.unwrap();

    Ok(DeviceClientCredentials {
        client_id,
        client_secret,
    })
}

pub async fn get_device_authorization_credentials(
    client: &sso_oidc::client::Client,
    device_credentials: &DeviceClientCredentials,
    start_url: &String,
) -> Result<DeviceAuthCredentials, sso_oidc::Error> {
    let get_device_authorization = client
        .start_device_authorization()
        .client_id(&device_credentials.client_id)
        .client_secret(&device_credentials.client_secret)
        .start_url(start_url)
        .send()
        .await?;

    Ok(DeviceAuthCredentials {
        user_code: get_device_authorization.user_code.unwrap(),
        device_code: get_device_authorization.device_code.unwrap(),
        verification_url: get_device_authorization.verification_uri_complete.unwrap(),
    })
}

pub async fn generate_token(
    client: &sso_oidc::client::Client,
    device_client_credentials: &DeviceClientCredentials,
    device_auth_credentials: &DeviceAuthCredentials,
) -> Result<String, sso_oidc::Error> {
    // Generate and return the token
    let generate_token_output = client
        .create_token()
        .client_id(&device_client_credentials.client_id)
        .client_secret(&device_client_credentials.client_secret)
        .set_grant_type(Some(
            "urn:ietf:params:oauth:grant-type:device_code".to_owned(),
        ))
        .device_code(&device_auth_credentials.device_code)
        .code(&device_auth_credentials.user_code)
        .send()
        .await?;

    Ok(generate_token_output.access_token.unwrap())
}
